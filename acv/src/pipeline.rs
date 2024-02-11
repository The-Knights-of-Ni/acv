use imageproc::definitions::Image;
use image::Rgb;

pub trait Pipeline {
    fn pipeline(&mut self, input: Image<Rgb<u8>>) -> crate::Result<Option<Image<Rgb<u8>>>>;

    fn output_color_type(&self) -> image::ColorType;
}
