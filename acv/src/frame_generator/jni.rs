use std::ops::Deref;
use image::Rgb;
use imageproc::definitions::Image;
use robusta_jni::jni::objects::JObject;
use robusta_jni::jni;
use crate::Result;
use crate::frame_generator::FrameGenerator;

struct JNIFrameGenerator<'lifetime> {
    class: JObject<'lifetime>,
}

impl<'lifetime> JNIFrameGenerator<'lifetime> {
    fn new(class: JObject<'lifetime>) -> Self {
        JNIFrameGenerator { class }
    }
}

impl FrameGenerator for JNIFrameGenerator<'_> {
    fn frame(&mut self) -> Result<Image<Rgb<u8>>> {
        self.class;
    }
}
