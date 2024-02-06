use image::Rgb;
use imageproc::definitions::Image;

#[cfg(feature = "jni")]
pub mod jni;

pub trait FrameGenerator {
    fn frame(&mut self) -> crate::Result<Image<Rgb<u8>>>;
}
