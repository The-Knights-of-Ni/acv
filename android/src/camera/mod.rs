use std::error::Error;
use std::ffi::c_char;
use std::fmt::{Debug, Display, Formatter};
use android_sys::*;

pub enum CameraError {
    #[doc = "Camera operation has failed due to an unspecified cause."]
    Unknown = -10000,
    #[doc = "Camera operation has failed due to an invalid parameter being passed to the method."]
    InvalidParameter = -10001,
    #[doc = "Camera operation has failed because the camera device has been closed, possibly because a\n higher-priority client has taken ownership of the camera device."]
    CameraDisconnected = -10002,
    #[doc = "Camera operation has failed due to insufficient memory."]
    NotEnoughMemory = -10003,
    #[doc = "Camera operation has failed due to the requested metadata tag cannot be found in input\n {@link ACameraMetadata} or {@link ACaptureRequest}."]
    MetadataNotFound = -10004,
    #[doc = "Camera operation has failed and the camera device has encountered a fatal error and needs to\n be re-opened before it can be used again."]
    CameraDevice = -10005,
    #[doc = "Camera operation has failed and the camera service has encountered a fatal error.\n\n <p>The Android device may need to be shut down and restarted to restore\n camera function, or there may be a persistent hardware problem.</p>\n\n <p>An attempt at recovery may be possible by closing the\n ACameraDevice and the ACameraManager, and trying to acquire all resources\n again from scratch.</p>"]
    CameraService = -10006,
    #[doc = "The {@link ACameraCaptureSession} has been closed and cannnot perform any operation other\n than {@link ACameraCaptureSession_close}."]
    SessionClosed = -10007,
    #[doc = "Camera operation has failed due to an invalid internal operation. Usually this is due to a\n low-level problem that may resolve itself on retry"]
    InvalidOperation = -10008,
    #[doc = "Camera device does not support the stream configuration provided by application in\n {@link ACameraDevice_createCaptureSession} or {@link\n ACameraDevice_isSessionConfigurationSupported}."]
    StreamConfigureFail = -10009,
    #[doc = "Camera device is being used by another higher priority camera API client."]
    CameraInUse = -10010,
    #[doc = "The system-wide limit for number of open cameras or camera resources has been reached, and\n more camera devices cannot be opened until previous instances are closed."]
    MaxCameraInUse = -10011,
    #[doc = "The camera is disabled due to a device policy, and cannot be opened."]
    CameraDisabled = -10012,
    #[doc = "The application does not have permission to open camera."]
    PermissionDenied = -10013,
    #[doc = "The operation is not supported by the camera device."]
    UnsupportedOperation = -10014
}

impl Debug for CameraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CameraError::Unknown => write!(f, "Unknown"),
            CameraError::InvalidParameter => write!(f, "InvalidParameter"),
            CameraError::CameraDisconnected => write!(f, "CameraDisconnected"),
            CameraError::NotEnoughMemory => write!(f, "NotEnoughMemory"),
            CameraError::MetadataNotFound => write!(f, "MetadataNotFound"),
            CameraError::CameraDevice => write!(f, "CameraDevice"),
            CameraError::CameraService => write!(f, "CameraService"),
            CameraError::SessionClosed => write!(f, "SessionClosed"),
            CameraError::InvalidOperation => write!(f, "InvalidOperation"),
            CameraError::StreamConfigureFail => write!(f, "StreamConfigureFail"),
            CameraError::CameraInUse => write!(f, "CameraInUse"),
            CameraError::MaxCameraInUse => write!(f, "MaxCameraInUse"),
            CameraError::CameraDisabled => write!(f, "CameraDisabled"),
            CameraError::PermissionDenied => write!(f, "PermissionDenied"),
            CameraError::UnsupportedOperation => write!(f, "UnsupportedOperation")
        }
    }
}

impl Display for CameraError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CameraError::Unknown => write!(f, "Unknown"),
            CameraError::InvalidParameter => write!(f, "InvalidParameter"),
            CameraError::CameraDisconnected => write!(f, "CameraDisconnected"),
            CameraError::NotEnoughMemory => write!(f, "NotEnoughMemory"),
            CameraError::MetadataNotFound => write!(f, "MetadataNotFound"),
            CameraError::CameraDevice => write!(f, "CameraDevice"),
            CameraError::CameraService => write!(f, "CameraService"),
            CameraError::SessionClosed => write!(f, "SessionClosed"),
            CameraError::InvalidOperation => write!(f, "InvalidOperation"),
            CameraError::StreamConfigureFail => write!(f, "StreamConfigureFail"),
            CameraError::CameraInUse => write!(f, "CameraInUse"),
            CameraError::MaxCameraInUse => write!(f, "MaxCameraInUse"),
            CameraError::CameraDisabled => write!(f, "CameraDisabled"),
            CameraError::PermissionDenied => write!(f, "PermissionDenied"),
            CameraError::UnsupportedOperation => write!(f, "UnsupportedOperation")
        }
    }
}

impl Error for CameraError {}

impl From<i32> for CameraError {
    fn from(error: i32) -> Self {
        match error {
            -10000 => CameraError::Unknown,
            -10001 => CameraError::InvalidParameter,
            -10002 => CameraError::CameraDisconnected,
            -10003 => CameraError::NotEnoughMemory,
            -10004 => CameraError::MetadataNotFound,
            -10005 => CameraError::CameraDevice,
            -10006 => CameraError::CameraService,
            -10007 => CameraError::SessionClosed,
            -10008 => CameraError::InvalidOperation,
            -10009 => CameraError::StreamConfigureFail,
            -10010 => CameraError::CameraInUse,
            -10011 => CameraError::MaxCameraInUse,
            -10012 => CameraError::CameraDisabled,
            -10013 => CameraError::PermissionDenied,
            -10014 => CameraError::UnsupportedOperation,
            _ => CameraError::Unknown
        }
    }
}

pub struct Device {
    device: ACameraDevice
}

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

struct CameraDevice {
    device: ACameraDevice,
    manager: CameraManager
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

    fn close(mut self) {
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
            let mut request = std::ptr::null_mut();
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
            let mut session = std::ptr::null_mut();
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
