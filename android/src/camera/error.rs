use std::fmt::{Debug, Display, Formatter};
use std::error::Error;

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
