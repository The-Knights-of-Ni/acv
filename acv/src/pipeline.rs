use imageproc::definitions::Image;
use image::Rgb;

pub trait Pipeline {
    /// The number of samples to run the pipeline.
    fn num_samples(&self) -> usize {
        3
    }

    fn pipeline(&mut self, input: Image<Rgb<u8>>) -> crate::Result<Option<Image<Rgb<u8>>>>;
}
