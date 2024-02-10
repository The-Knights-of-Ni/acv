use std::error::Error;
use std::ffi::c_char;
use std::fmt::{Debug, Display};
use android_sys::*;
pub use error::CameraError;

pub mod error;


pub struct CameraManager {
    manager: ACameraManager
}

impl CameraManager {
    pub fn new() -> Result<Self, i32> {
        unsafe {
            let manager = ACameraManager_create().as_mut();
            if let Some(manager) = manager {
                Ok(Self {
                    manager: *manager
                })
            } else {
                Err(0)
            }
        }
    }

    pub fn open_camera(&mut self, camera_id: &str) -> Result<(), CameraError> {
        unsafe {
            let mut callback = ACameraDevice_stateCallbacks {
                context: std::ptr::null_mut(),
                onDisconnected: None,
                onError: None
            };
            let mut device = std::ptr::null_mut();
            let resp = ACameraManager_openCamera(self.as_raw_mut(), camera_id.as_ptr() as *mut c_char, &mut callback, &mut device);
            if resp != 0 {
                Err(resp as i32)?;
            }
            Ok(())
        }
    }

    pub fn get_camera_id_list(&mut self) -> Result<Vec<String>, CameraError> {
        unsafe {
            let mut camera_id_list = std::ptr::null_mut();
            // TODO: ACameraManager_deleteCameraIdList not freed properly, which will cause a memory leak
            let resp = ACameraManager_getCameraIdList(self.as_raw_mut(), &mut camera_id_list);
            if resp != 0 {
                Err(resp as i32)?;
            }
            let mut camera_ids = Vec::new();
            while !camera_id_list.is_null() {
                let camera_id = std::ffi::CStr::from_ptr(*(*camera_id_list).cameraIds).to_str().unwrap().to_string();
                camera_ids.push(camera_id);
                camera_id_list = camera_id_list.offset(1);
            }
            ACameraManager_deleteCameraIdList(camera_id_list);
            Ok(camera_ids)
        }
    }

    pub fn get_camera_characteristics(&mut self, camera_id: &str) -> Result<ACameraMetadata, CameraError> {
        unsafe {
            let mut characteristics = std::ptr::null_mut();
            let resp = ACameraManager_getCameraCharacteristics(self.as_raw_mut(), camera_id.as_ptr() as *mut c_char, &mut characteristics);
            if resp != 0 {
                Err(resp as i32)?;
            }
            Ok(*characteristics)
        }
    }

    pub fn into_raw(self) -> ACameraManager {
        self.manager
    }

    pub fn as_raw(&self) -> &ACameraManager {
        &self.manager
    }

    pub fn as_raw_mut(&mut self) -> &mut ACameraManager {
        &mut self.manager
    }
}


impl From<ACameraManager> for CameraManager {
    fn from(manager: ACameraManager) -> Self {
        CameraManager {
            manager
        }
    }
}

pub struct CameraDevice {
    device: ACameraDevice,
    pub manager: CameraManager
}

impl CameraDevice {
    pub fn new(camera_id: &str, callback: &mut ACameraDevice_StateCallbacks) -> Result<Self, i32> {
        unsafe {
            let mut manager = CameraManager::new()?;
            let mut device = std::ptr::null_mut();
            let resp = ACameraManager_openCamera(manager.as_raw_mut(), camera_id.as_ptr() as *mut c_char, callback, &mut device);
            if resp != 0 {
                return Err(resp.into());
            }
            Ok(Self {
                device: *device,
                manager
            })
        }
    }

    pub fn into_raw(self) -> ACameraDevice {
        self.device
    }

    pub fn as_raw(&self) -> &ACameraDevice {
        &self.device
    }

    pub fn as_raw_mut(&mut self) -> &mut ACameraDevice {
        &mut self.device
    }

    pub fn manager(&self) -> &CameraManager {
        &self.manager
    }

    pub fn close(mut self) {
        unsafe {
            ACameraDevice_close(&mut self.device);
        }
    }
}

impl Drop for CameraDevice {
    fn drop(&mut self) {
        unsafe {
            ACameraDevice_close(&mut self.device);
        }
    }
}

pub struct CaptureSessionOutput {
    output: ACaptureSessionOutput
}

impl CaptureSessionOutput {
    pub fn new(surface: &mut ANativeWindow) -> Self {
        unsafe {
            let mut output = std::ptr::null_mut();
            ACaptureSessionOutput_create(surface, &mut output);
            Self {
                output: *output
            }
        }
    }

    pub fn into_raw(self) -> ACaptureSessionOutput {
        self.output
    }

    pub fn as_raw(&self) -> &ACaptureSessionOutput {
        &self.output
    }

    pub fn as_raw_mut(&mut self) -> &mut ACaptureSessionOutput {
        &mut self.output
    }
}

impl Drop for CaptureSessionOutput {
    fn drop(&mut self) {
        unsafe {
            ACaptureSessionOutput_free(&mut self.output);
        }
    }
}

pub struct CaptureSessionOutputContainer {
    output: ACaptureSessionOutputContainer
}

impl CaptureSessionOutputContainer {
    pub fn new() -> Self {
        unsafe {
            let mut output = std::ptr::null_mut();
            ACaptureSessionOutputContainer_create(&mut output);
            Self {
                output: *output
            }
        }
    }

    pub fn add(&mut self, output: CaptureSessionOutput) -> Result<(), CameraError> {
        unsafe {
            let status = ACaptureSessionOutputContainer_add(self.as_raw_mut(), output.as_raw());
            if status != 0 {
                Err(status)?;
            }
            Ok(())
        }
    }

    pub fn remove(&mut self, output: CaptureSessionOutput) -> Result<(), CameraError> {
        unsafe {
            let status = ACaptureSessionOutputContainer_remove(self.as_raw_mut(), output.as_raw());
            if status != 0 {
                Err(status)?;
            }
            Ok(())
        }
    }

    pub fn into_raw(self) -> ACaptureSessionOutputContainer {
        self.output
    }

    pub fn as_raw(&self) -> &ACaptureSessionOutputContainer {
        &self.output
    }

    pub fn as_raw_mut(&mut self) -> &mut ACaptureSessionOutputContainer {
        &mut self.output
    }
}

impl Drop for CaptureSessionOutputContainer {
    fn drop(&mut self) {
        unsafe {
            ACaptureSessionOutputContainer_free(&mut self.output);
        }
    }
}


pub enum RequestTemplate {
    Preview = ACameraDevice_request_template_TEMPLATE_PREVIEW as isize,
    StillCapture = ACameraDevice_request_template_TEMPLATE_STILL_CAPTURE as isize,
    Record = ACameraDevice_request_template_TEMPLATE_RECORD as isize,
    VideoSnapshot = ACameraDevice_request_template_TEMPLATE_VIDEO_SNAPSHOT as isize,
    ZeroShutterLag = ACameraDevice_request_template_TEMPLATE_ZERO_SHUTTER_LAG as isize,
    Manual = ACameraDevice_request_template_TEMPLATE_MANUAL as isize
}

pub struct CaptureRequest {
    request: ACaptureRequest
}

impl CaptureRequest {
    pub fn new(device: &CameraDevice, request_template: RequestTemplate) -> Self {
        unsafe {
            let request = std::ptr::null_mut();
            ACameraDevice_createCaptureRequest(device.as_raw(), request_template as isize as ACameraDevice_request_template, request);
            Self {
                request: **request
            }
        }
    }

    pub fn into_raw(self) -> ACaptureRequest {
        self.request
    }

    pub fn as_raw(&self) -> &ACaptureRequest {
        &self.request
    }

    pub fn as_raw_mut(&mut self) -> &mut ACaptureRequest {
        &mut self.request
    }

}

pub struct CameraCaptureSession {
    session: ACameraCaptureSession,
    container: CaptureSessionOutputContainer
}

impl CameraCaptureSession {
    pub fn new(camera_id: &str) -> Result<Self, CameraError> {
        unsafe {
            // TODO: Callback support
            let callback = ACameraCaptureSession_stateCallbacks {
                context: std::ptr::null_mut(),
                onClosed: None,
                onReady: None,
                onActive: None,
            };
            let mut device_callback = ACameraDevice_stateCallbacks {
                context: std::ptr::null_mut(),
                onDisconnected: None,
                onError: None,
            };
            let mut device = CameraDevice::new(camera_id, &mut device_callback)?;
            let session = std::ptr::null_mut();
            let output = CaptureSessionOutputContainer::new();
            let resp = ACameraDevice_createCaptureSession(device.as_raw_mut(), output.as_raw(), &callback, session);
            if resp != 0 {
                Err(resp as i32)?;
            }
            Ok(Self {
                session: **session,
                container: output
            })
        }
    }

    pub fn into_raw(self) -> ACameraCaptureSession {
        self.session
    }

    pub fn as_raw(&self) -> &ACameraCaptureSession {
        &self.session
    }

    pub fn as_raw_mut(&mut self) -> &mut ACameraCaptureSession {
        &mut self.session
    }

    pub fn capture(&mut self, capture_request: Vec<CaptureRequest>) -> Result<(), CameraError> {
        unsafe {
            let mut requests = Vec::new();
            for request in capture_request {
                requests.push(*request.as_raw());
            }
            let capture_sequence_id = std::ptr::null_mut(); // TODO: Don't set to null (see docs)
            let status = ACameraCaptureSession_captureV2(self.as_raw_mut(), std::ptr::null_mut(), requests.len() as i32, vec![requests.as_mut_ptr()].as_mut_ptr(), capture_sequence_id);
            if status != 0 {
                Err(status)?;
            }
            Ok(())
        }
    }

    pub fn abort_capture(&mut self) -> Result<(), CameraError> {
        unsafe {
            let status = ACameraCaptureSession_abortCaptures(self.as_raw_mut());
            if status != 0 {
                Err(status)?;
            }
            Ok(())
        }
    }

    pub fn get_device(&mut self) -> Result<CameraDevice, CameraError> {
        todo!("Implement get_device")
    }

    pub fn close(self) {
        drop(self);
    }
}

impl Drop for CameraCaptureSession {
    fn drop(&mut self) {
        unsafe {
            ACameraCaptureSession_close(&mut self.session);
        }
    }
}
