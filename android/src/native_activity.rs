use android_sys::*;

pub struct NativeWindow {
    window: *mut ANativeWindow,
}

impl NativeWindow {
    pub fn new(env: &mut JNIEnv, surface: jobject) -> Self {
        unsafe {
            let window = ANativeWindow_fromSurface(env, surface);
            if window.is_null() {
                panic!("ANativeWindow_fromSurface failed");
            }
            let window = Self { window };
            window
        }
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut ANativeWindow {
        self.window
    }

    pub unsafe fn as_ptr(&self) -> *const ANativeWindow {
        self.window
    }
}
