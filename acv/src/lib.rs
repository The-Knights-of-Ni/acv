pub use imageproc::definitions::Image;
pub use error::Error;
pub use imageproc;

pub mod error;

type Result<T> = std::result::Result<T, Error>;

pub trait Pipeline {
    /// The number of samples to run the pipeline.
    fn num_samples(&self) -> usize {
        3
    }

    fn pipeline<P>(&self, input: Image<P>) -> Result<Option<Image<P>>>;
}
