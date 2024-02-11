use image::Rgb;
use imageproc::definitions::Image;
use jni::objects::{JByteArray, JObject};
use jni;
use jni::sys::jbyteArray;
use crate::Result;
use crate::frame_generator::FrameGenerator;

pub struct JNIFrameGenerator<'lifetime> {
    class: JObject<'lifetime>,
    env: jni::JNIEnv<'lifetime>
}

impl<'lifetime> JNIFrameGenerator<'lifetime> {
    pub fn new(env: jni::JNIEnv<'lifetime>, class: JObject<'lifetime>) -> Self {
        JNIFrameGenerator {
            class,
            env
        }
    }
}

impl FrameGenerator for JNIFrameGenerator<'_> {
    fn frame(&mut self) -> Result<Image<Rgb<u8>>> {
        let v = self.env.call_method(&self.class, "getFrame", "()V", &[]).unwrap();
        let array = v.l().map_err(|_| "Java method did not return jobject".to_string())?;
        let jni_array: jbyteArray = array.as_raw();
        let width_object = self.env.get_field(&self.class, "width", "I").unwrap();
        let width = width_object.i().unwrap();
        let height_object = self.env.get_field(&self.class, "height", "I").unwrap();
        let height = height_object.i().unwrap();
        let wrapped_array = unsafe { JByteArray::from_raw(jni_array) };
        let data = self.env.convert_byte_array(&wrapped_array).unwrap();
        let image = Image::from_raw(width as u32, height as u32, data).ok_or("Failed to create image from raw data")?;
        Ok(image)
    }
}
