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

    pub fn get_format(&self) -> Result<i32, i32> {
        let resp = unsafe { ANativeWindow_getFormat(self.window) };
        if resp < 0 {
            Err(resp)
        } else {
            Ok(resp) // TODO: Convert to enum
        }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { ANativeWindow_getWidth(self.window) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { ANativeWindow_getHeight(self.window) }
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
