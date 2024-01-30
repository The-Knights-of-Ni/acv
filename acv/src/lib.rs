use std::rc::Rc;
use image::{GrayImage, Luma, Rgb, Rgba};
pub use imageproc::definitions::Image;
pub use error::Error;
pub use imageproc;

pub mod error;

type Result<T> = std::result::Result<T, Error>;

struct Hsv {
    h: f32,
    s: f32,
    v: f32,
}

impl From<Rgb<u8>> for Hsv {
    fn from(value: Rgb<u8>) -> Self {
        let r_prime = value[0] as f32 / 255.0;
        let g_prime = value[1] as f32 / 255.0;
        let b_prime = value[2] as f32 / 255.0;
        let c_max = r_prime.max(g_prime).max(b_prime);
        let c_min = r_prime.min(g_prime).min(b_prime);
        let delta = c_max - c_min;
        let h = if delta == 0.0 {
            0.0
        } else if c_max == r_prime {
            60.0 * (((g_prime - b_prime) / delta) % 6.0)
        } else if c_max == g_prime {
            60.0 * (((b_prime - r_prime) / delta) + 2.0)
        } else {
            60.0 * (((r_prime - g_prime) / delta) + 4.0)
        };
        let s = if c_max == 0.0 {
            0.0
        } else {
            delta / c_max
        };
        let v = c_max;
        Hsv { h, s, v }
    }
}


pub fn in_range_hsv(src: &Image<Rgb<u8>>, lower: Hsv, higher: Hsv, dst: &mut GrayImage) {
    for (src_pixel, dst_pixel) in src.pixels().zip(dst.pixels_mut()) {
        let hsv = Hsv::from(*src_pixel);
        if hsv.h >= lower.h && hsv.h <= higher.h &&
            hsv.s >= lower.s && hsv.s <= higher.s &&
            hsv.v >= lower.v && hsv.v <= higher.v {
            *dst_pixel = Luma([255]);
        } else {
            *dst_pixel = Luma([0]);
        }
    }
}


pub fn in_range_rgb(src: &Image<Rgb<u8>>, lower: Rgb<u8>, higher: Rgb<u8>, dst: &mut GrayImage) {
    for (src_pixel, dst_pixel) in src.pixels().zip(dst.pixels_mut()) {
        if src_pixel[0] >= lower[0] && src_pixel[0] <= higher[0] &&
            src_pixel[1] >= lower[1] && src_pixel[1] <= higher[1] &&
            src_pixel[2] >= lower[2] && src_pixel[2] <= higher[2] {
            *dst_pixel = Luma([255]);
        } else {
            *dst_pixel = Luma([0]);
        }
    }
}

pub trait Pipeline {
    /// The number of samples to run the pipeline.
    fn num_samples(&self) -> usize {
        3
    }

    fn pipeline(&self, input: Image<Rgb<u8>>) -> Result<Option<Image<Rgb<u8>>>>;
}
