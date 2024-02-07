use image::Rgb;
use imageproc::definitions::Image;
use robusta_jni::jni::objects::JObject;
use robusta_jni::jni;
use robusta_jni::jni::sys::jbyteArray;
use crate::Result;
use crate::frame_generator::FrameGenerator;

struct JNIFrameGenerator<'lifetime> {
    class: JObject<'lifetime>,
    env: jni::JNIEnv<'lifetime>
}

impl<'lifetime> JNIFrameGenerator<'lifetime> {
    fn new(env: jni::JNIEnv<'lifetime>, class: JObject<'lifetime>) -> Self {
        JNIFrameGenerator {
            class,
            env
        }
    }
}

impl FrameGenerator for JNIFrameGenerator<'_> {
    fn frame(&mut self) -> Result<Image<Rgb<u8>>> {
        let v = self.env.call_method(self.class, "getFrame", "()V", &[]).unwrap();
        let array = v.l().map_err(|_| "Java method did not return jobject".to_string())?;
        let jni_array: jbyteArray = unsafe {
            // Pointer Transmutation (we know it's a byte array)
            std::mem::transmute(array.into_inner())
        };
        let mut vec_array: Vec<u8> = self.env.convert_byte_array(jni_array.to_owned()).unwrap();
        let width = u32::from_be_bytes([vec_array[0], vec_array[1], vec_array[2], vec_array[3]]);
        let height = u32::from_be_bytes([vec_array[4], vec_array[5], vec_array[6], vec_array[7]]);
        vec_array.drain(0..8);
        // TODO: Fix width and height
        let image = Image::from_raw(width, height, vec_array).ok_or("Failed to create image from raw data")?;
        Ok(image)
    }
}
