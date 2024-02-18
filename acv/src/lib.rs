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
pub mod output;
pub mod pipeline;
pub mod util;

// TODO: Differentiate between the different types of errors
type Result<T> = std::result::Result<T, Error>;

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

    pub async fn process_frame(&mut self) {
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
    pub async fn run(&mut self) {
        loop {
            self.process_frame().await;
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
