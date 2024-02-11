use android_sys::{__android_log_print, android_LogPriority_ANDROID_LOG_UNKNOWN, android_LogPriority_ANDROID_LOG_DEBUG, android_LogPriority_ANDROID_LOG_INFO, android_LogPriority_ANDROID_LOG_WARN, android_LogPriority_ANDROID_LOG_ERROR, android_LogPriority_ANDROID_LOG_VERBOSE};

pub enum LogPriority {
    Unknown = android_LogPriority_ANDROID_LOG_UNKNOWN as isize,
    Verbose = android_LogPriority_ANDROID_LOG_VERBOSE as isize,
    Debug = android_LogPriority_ANDROID_LOG_DEBUG as isize,
    Info = android_LogPriority_ANDROID_LOG_INFO as isize,
    Warn = android_LogPriority_ANDROID_LOG_WARN as isize,
    Error = android_LogPriority_ANDROID_LOG_ERROR as isize,
}

pub fn v(tag: &str, msg: &str) {
    log(LogPriority::Unknown, tag, msg)
}

pub fn d(tag: &str, msg: &str) {
    log(LogPriority::Debug, tag, msg)
}

pub fn i(tag: &str, msg: &str) {
    log(LogPriority::Info, tag, msg)
}

pub fn w(tag: &str, msg: &str) {
    log(LogPriority::Warn, tag, msg)
}

pub fn e(tag: &str, msg: &str) {
    log(LogPriority::Error, tag, msg)
}

pub fn log(priority: LogPriority, tag: &str, msg: &str) {
    unsafe {
        __android_log_print(priority as i32, tag.as_ptr() as *const i8, msg.as_ptr() as *const i8);
    }
}
