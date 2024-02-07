use android_sys::*;

pub struct NativeWindow {
    window: *mut ANativeWindow,
}

impl NativeWindow {
    pub fn new(env: &mut JNIEnv, surface: jobject) -> Option<Self> {
        unsafe {
            let window = ANativeWindow_fromSurface(env, surface);
            if window.is_null() {
                return None;
            }
            let window = Self { window };
            Some(window)
        }
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut ANativeWindow {
        self.window
    }

    pub unsafe fn as_ptr(&self) -> *const ANativeWindow {
        self.window
    }
}

impl Drop for NativeWindow {
    fn drop(&mut self) {
        unsafe {
            ANativeWindow_release(self.window);
        }
    }
}
