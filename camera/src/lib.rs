use android::camera::CameraCaptureSession;

pub struct AndroidCamera {
    pub width: u32,
    pub height: u32,
    pub camera: CameraCaptureSession
}

impl AndroidCamera {
    pub fn new(width: u32, height: u32, camera_name: &str) -> Self {
        let camera = CameraCaptureSession::new(camera_name).unwrap();
        AndroidCamera {
            width,
            height,
            camera
        }
    }
}
