use image::Rgb;
use imageproc::definitions::Image;

#[cfg(feature = "camera-jni")]
pub mod jni;

#[cfg(feature = "camera-ndk")]
pub mod ndk;

pub trait FrameGenerator {
    fn frame(&mut self) -> crate::Result<Image<Rgb<u8>>>;
}
