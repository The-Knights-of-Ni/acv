use android_sys::{android_get_application_target_sdk_version, android_get_device_api_level};

pub fn get_application_target_sdk_version() -> i32 {
    unsafe { android_get_application_target_sdk_version() }
}

pub fn get_device_api_level() -> i32 {
    unsafe { android_get_device_api_level() }
}
