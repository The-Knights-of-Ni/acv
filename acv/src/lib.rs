use std::sync::Arc;
use tokio::sync::Mutex;
use image::{GrayImage, Luma, Rgb};
pub use imageproc::definitions::Image;
pub use error::Error;
pub use imageproc;
use log::error;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpSocket, UdpSocket, UnixListener, UnixStream};
use output::Output;
use pipeline::Pipeline;
use crate::frame_generator::FrameGenerator;

pub mod error;
pub mod frame_generator;
mod output;
mod pipeline;

// TODO: Differentiate between the different types of errors
type Result<T> = std::result::Result<T, Error>;

pub struct Hsv {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

impl From<Rgb<u8>> for Hsv {
    fn from(value: Rgb<u8>) -> Self {
        let r_prime = value[0] as f32 / 255.0;
        let g_prime = value[1] as f32 / 255.0;
        let b_prime = value[2] as f32 / 255.0;
        let c_max = r_prime.max(g_prime).max(b_prime);
        let c_min = r_prime.min(g_prime).min(b_prime);
        let delta = c_max - c_min;
        let h = if delta == 0.0 {
            0.0
        } else if c_max == r_prime {
            60.0 * (((g_prime - b_prime) / delta) % 6.0)
        } else if c_max == g_prime {
            60.0 * (((b_prime - r_prime) / delta) + 2.0)
        } else {
            60.0 * (((r_prime - g_prime) / delta) + 4.0)
        };
        let s = if c_max == 0.0 {
            0.0
        } else {
            delta / c_max
        };
        let v = c_max;
        Hsv { h, s, v }
    }
}


pub fn in_range_hsv(src: &Image<Rgb<u8>>, lower: Hsv, higher: Hsv, dst: &mut GrayImage) {
    for (src_pixel, dst_pixel) in src.pixels().zip(dst.pixels_mut()) {
        let hsv = Hsv::from(*src_pixel);
        if hsv.h >= lower.h && hsv.h <= higher.h &&
            hsv.s >= lower.s && hsv.s <= higher.s &&
            hsv.v >= lower.v && hsv.v <= higher.v {
            *dst_pixel = Luma([255]);
        } else {
            *dst_pixel = Luma([0]);
        }
    }
}


pub fn in_range_rgb(src: &Image<Rgb<u8>>, lower: Rgb<u8>, higher: Rgb<u8>, dst: &mut GrayImage) {
    for (src_pixel, dst_pixel) in src.pixels().zip(dst.pixels_mut()) {
        if src_pixel[0] >= lower[0] && src_pixel[0] <= higher[0] &&
            src_pixel[1] >= lower[1] && src_pixel[1] <= higher[1] &&
            src_pixel[2] >= lower[2] && src_pixel[2] <= higher[2] {
            *dst_pixel = Luma([255]);
        } else {
            *dst_pixel = Luma([0]);
        }
    }
}

pub struct SinglePipelineCamera {
    pub width: u32,
    pub height: u32,
    pub pipeline: Option<Arc<Mutex<dyn Pipeline>>>,
    pub output: Arc<Mutex<dyn Output>>,
    pub camera: Arc<Mutex<dyn FrameGenerator>>
}

impl SinglePipelineCamera {
    pub fn new(width: u32, height: u32, camera: Arc<Mutex<dyn FrameGenerator>>) -> Self {
        SinglePipelineCamera {
            width,
            height,
            pipeline: None,
            output: Arc::new(Mutex::new(output::NoOutput::default())),
            camera
        }
    }

    pub fn set_pipeline(&mut self, pipeline: Option<Arc<Mutex<dyn Pipeline>>>) {
        self.pipeline = pipeline;
    }

    pub fn set_output(&mut self, output: Option<Arc<Mutex<dyn Output>>>) {
        if let Some(output) = output {
            self.output = output;
        } else {
            self.output = Arc::new(Mutex::new(output::NoOutput::default()));
        }
    }

    pub async fn run(&mut self) {
        loop {
            let frame_result = self.camera.lock().await.frame();
            match frame_result {
                Ok(frame) => {
                    if let Some(pipeline) = &self.pipeline {
                        let mut pipeline = pipeline.lock().await;
                        let frame = pipeline.pipeline(frame);
                        let mut output_sender = self.output.lock().await;
                        let output_result = output_sender.output(frame, pipeline.output_color_type());
                        if let Err(e) = output_result {
                            error!("Error sending output: {}", e);
                        }
                    }
                },
                Err(e) => {
                    error!("Error getting frame from camera: {}", e);
                }
            }
        }
    }
}

async fn terminate_on_signal(mut socket: UnixStream) {
    loop {
        let mut response = String::new();
        let res = socket.read_to_string(&mut response).await; // TODO: Do something with the response
        if response == "terminate" {
            return;
        }
    }
}

async fn udp_socket_terminate_on_signal(socket: UdpSocket) {
    loop {
        let mut buf = [0; 1024];
        let resp = socket.recv(&mut buf).await.unwrap(); // TODO: Do something with the response
        let cut_buff = &buf[..resp];
        if cut_buff == b"terminate" {
            return;
        }
    }
}

#[allow(non_snake_case)]
#[cfg(feature = "input-jni")]
pub mod android {
    use std::sync::Arc;
    use jni::JNIEnv;
    use jni::objects::{JClass, JObject, JString};
    use jni::sys::jboolean;
    use tokio::net::{UdpSocket, UnixStream};
    use tokio::select;
    use tokio::sync::Mutex;

    #[no_mangle]
    #[tokio::main]
    pub async unsafe extern fn Java_org_knightsofni_acv_RustNative_nativeRun<'local>(mut env: JNIEnv<'local>,
                                                                                     class: JClass<'local>,
                                                                                     storage_class: JObject<'local>,
                                                                                     camera_name: JString<'local>,
                                                                                     socket_path: JString<'local>,
                                                                                     use_socket_input: jboolean) {
        let camera_name: String = env.get_string(&camera_name).expect("Couldn't get camera name").into();
        let path: String = env.get_string(&socket_path).expect("Couldn't get socket path").into();
        let use_socket_input: bool = use_socket_input != 0;
        let frame_generator = Arc::new(Mutex::new(crate::frame_generator::jni::JNIFrameGenerator::new(env, storage_class)));
        let mut camera = crate::SinglePipelineCamera::new(640, 480, frame_generator);
        let terminate_future = if use_socket_input {
            let input_stream = UnixStream::connect(path.clone() + "_input")?;
            let output_stream = UnixStream::connect(path + "_output")?;
            let output = Arc::new(Mutex::new(crate::output::StreamOutput::from_socket(output_stream)));
            camera.set_output(Some(output));
            crate::terminate_on_signal(input_stream)
        } else {
            let input_socket = UdpSocket::bind(path.clone() + "0").await.unwrap();
            let output_socket = UdpSocket::bind(path + "1").await.unwrap();
            crate::udp_socket_terminate_on_signal(input_socket)
        };

        let camera_future = camera.run();
        select! {
            _ = terminate_future => panic!(),
            _ = camera_future => panic!(),
        }
    }
}
