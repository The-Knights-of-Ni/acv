#![no_std]
#![cfg_attr(
    feature = "unstable-512-bit-simd",
    feature(link_llvm_intrinsics, repr_simd, simd_ffi, platform_intrinsics)
)]

#[cfg(feature = "serde")]
mod serde_impl;

use core::{
    fmt,
    hash::{Hash, Hasher},
    ops::{BitAnd, BitOr, BitXor, Deref, DerefMut},
    slice,
};

#[cfg(feature = "space")]
use space::Metric;

cfg_if::cfg_if! {
    if #[cfg(feature = "unstable-512-bit-simd")] {
        #[repr(simd)]
        #[derive(Copy, Clone)]
        struct Tup(u128, u128, u128, u128);

        #[allow(improper_ctypes, dead_code)]
        extern "C" {
            #[link_name = "llvm.ctpop.v4i128"]
            fn ctpop_512(x: Tup) -> Tup;
            #[link_name = "llvm.experimental.vector.reduce.add.v4i128"]
            fn reduce_add_512(x: Tup) -> u128;
        }

        extern "platform-intrinsic" {
            fn simd_xor<T>(x: T, y: T) -> T;
        }

        /// Split the bytes up into number of operations of size (512, 64, 8)
        const fn split_up_simd(n: usize) -> (usize, usize, usize) {
            let n_512 = n >> 6;
            let bytes_512 = n_512 << 6;
            let n_64 = (n - bytes_512) >> 3;
            let bytes_64 = n_64 << 3;
            let n_8 = n - bytes_512 - bytes_64;
            (n_512, n_64, n_8)
        }
    } else {
        /// Split the bytes up into number of operations of size (512, 64, 8)
        const fn split_up_simd(n: usize) -> (usize, usize) {
            let n_64 = n >> 3;
            let bytes_64 = n_64 << 3;
            let n_8 = n - bytes_64;
            (n_64, n_8)
        }
    }
}

/// A constant sized array of bits. `B` defines the number of bytes.
/// This has an alignment of 64 to maximize the efficiency of SIMD operations.
/// It will automatically utilize SIMD at runtime where possible.
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct BitArray<const B: usize> {
    pub bytes: [u8; B],
}

impl<const B: usize> BitArray<B> {
    /// Create a new `BitArray`.
    ///
    /// ```
    /// use bitarray::BitArray;
    /// let array = BitArray::new([0]);
    /// assert_eq!(*array.bytes(), [0]);
    /// ```
    pub fn new(bytes: [u8; B]) -> Self {
        Self { bytes }
    }

    /// Create a new `BitArray` with all zeros.
    ///
    /// ```
    /// use bitarray::BitArray;
    /// let array = BitArray::zeros();
    /// assert_eq!(array, BitArray::new([0]));
    /// assert_eq!(*array, [0]);
    /// ```
    pub fn zeros() -> Self {
        Self { bytes: [0; B] }
    }

    /// Retrieve the byte array of a `BitArray`.
    ///
    /// ```
    /// use bitarray::BitArray;
    /// let array = BitArray::new([1, 2]);
    /// assert_eq!(*array, [1, 2]);
    /// ```
    pub fn bytes(&self) -> &[u8; B] {
        &self.bytes
    }

    /// Retrieve the mutable byte array of a `BitArray`.
    ///
    /// ```
    /// use bitarray::BitArray;
    /// let mut array = BitArray::new([1, 2]);
    /// array.bytes_mut()[0] = 3;
    /// assert_eq!(*array, [3, 2]);
    /// ```
    pub fn bytes_mut(&mut self) -> &mut [u8; B] {
        &mut self.bytes
    }

    /// Compute the hamming weight (number of ones) of the `BitArray`.
    ///
    /// This is also called `count_ones` in the standard library.
    ///
    /// ```
    /// use bitarray::BitArray;
    /// let array = BitArray::new([0xAA; 83]);
    /// assert_eq!(array.weight(), 4 * 83);
    /// ```
    #[allow(clippy::cast_ptr_alignment)]
    pub fn weight(&self) -> u32 {
        cfg_if::cfg_if! {
            if #[cfg(feature = "unstable-512-bit-simd")] {
                let (n_512, n_64, n_8) = split_up_simd(self.bytes.len());
                let sum_512 = unsafe {
                    slice::from_raw_parts(self.bytes.as_ptr() as *const Tup, n_512)
                        .iter()
                        .copied()
                        .map(|chunk| reduce_add_512(ctpop_512(chunk)) as u32)
                        .sum::<u32>()
                };
                let sum_64 = unsafe {
                    slice::from_raw_parts(self.bytes.as_ptr() as *const u64, n_64)
                        .iter()
                        .copied()
                        .map(|chunk| chunk.count_ones())
                        .sum::<u32>()
                };

                let sum_8 = self.bytes[self.bytes.len() - n_8..]
                    .iter()
                    .copied()
                    .map(|b| b.count_ones())
                    .sum::<u32>();

                sum_512 + sum_64 + sum_8
            } else {
                let (n_64, n_8) = split_up_simd(self.bytes.len());
                let sum_64 = unsafe {
                    slice::from_raw_parts(self.bytes.as_ptr() as *const u64, n_64)
                        .iter()
                        .copied()
                        .map(|chunk| chunk.count_ones())
                        .sum::<u32>()
                };

                let sum_8 = self.bytes[self.bytes.len() - n_8..]
                    .iter()
                    .copied()
                    .map(|b| b.count_ones())
                    .sum::<u32>();

                sum_64 + sum_8
            }
        }
    }

    /// Compute the hamming distance to another `BitArray`.
    ///
    /// ```
    /// use bitarray::BitArray;
    ///
    /// // All the bits are different.
    /// let a = BitArray::new([0xAA; 65]);
    /// let b = BitArray::new([0x55; 65]);
    /// assert_eq!(a.distance(&b), 8 * 65);
    ///
    /// // None of the bits are different.
    /// let a = BitArray::new([0xAA; 65]);
    /// let b = BitArray::new([0xAA; 65]);
    /// assert_eq!(a.distance(&b), 0);
    /// ```
    #[allow(clippy::cast_ptr_alignment)]
    pub fn distance(&self, other: &Self) -> u32 {
        cfg_if::cfg_if! {
            if #[cfg(feature = "unstable-512-bit-simd")] {
                let simd_len = B >> 6;
                let simd_bytes = simd_len << 6;
                let simd_sum = unsafe {
                    slice::from_raw_parts(self.bytes.as_ptr() as *const Tup, simd_len)
                        .iter()
                        .copied()
                        .zip(
                            slice::from_raw_parts(other.bytes.as_ptr() as *const Tup, simd_len)
                                .iter()
                                .copied(),
                        )
                        .map(|(a, b)| reduce_add_512(ctpop_512(simd_xor(a, b))) as u32)
                        .sum::<u32>()
                };
                let remaining_sum = self.bytes[simd_bytes..]
                    .iter()
                    .copied()
                    .zip(other.bytes[simd_bytes..].iter().copied())
                    .map(|(a, b)| (a ^ b).count_ones())
                    .sum::<u32>();
                simd_sum + remaining_sum
            } else {
                self.bytes
                    .iter()
                    .copied()
                    .zip(other.bytes.iter().copied())
                    .map(|(a, b)| (a ^ b).count_ones())
                    .sum::<u32>()
            }
        }
    }
}

impl<const B: usize> BitAnd for BitArray<B> {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        for (d, s) in self.iter_mut().zip(rhs.iter().copied()) {
            *d &= s;
        }
        self
    }
}

impl<const B: usize> BitOr for BitArray<B> {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        for (d, s) in self.iter_mut().zip(rhs.iter().copied()) {
            *d |= s;
        }
        self
    }
}

impl<const B: usize> BitXor for BitArray<B> {
    type Output = Self;

    fn bitxor(mut self, rhs: Self) -> Self::Output {
        for (d, s) in self.iter_mut().zip(rhs.iter().copied()) {
            *d ^= s;
        }
        self
    }
}

impl<const B: usize> PartialEq for BitArray<B> {
    fn eq(&self, other: &Self) -> bool {
        self.bytes
            .iter()
            .zip(other.bytes.iter())
            .all(|(&a, &b)| a == b)
    }
}

impl<const B: usize> Eq for BitArray<B> {}

impl<const B: usize> fmt::Debug for BitArray<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.bytes[..].fmt(f)
    }
}

impl<const B: usize> Hash for BitArray<B> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bytes[..].hash(state)
    }
}

/// ```
/// use bitarray::BitArray;
/// let mut array = BitArray::new([1, 2]);
/// assert_eq!(*array, [1, 2]);
/// ```
impl<const B: usize> Deref for BitArray<B> {
    type Target = [u8; B];

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

/// ```
/// use bitarray::BitArray;
/// let mut array = BitArray::zeros();
/// array[0] = 1;
/// array[1] = 2;
/// assert_eq!(*array, [1, 2]);
/// ```
impl<const B: usize> DerefMut for BitArray<B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bytes
    }
}

/// Provides [hamming distance](https://en.wikipedia.org/wiki/Hamming_distance) as a metric.
#[cfg(feature = "space")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hamming;

#[cfg(feature = "space")]
impl<const B: usize> Metric<BitArray<B>> for Hamming {
    type Unit = u32;

    fn distance(&self, a: &BitArray<B>, b: &BitArray<B>) -> u32 {
        a.distance(b) as u32
    }
}

/// Provides [Jaccard distance](https://en.wikipedia.org/wiki/Jaccard_index) as a metric.
///
/// The Jaccard similarity is computed and then subtracted from `1.0`
/// so that items are ordered by Jaccard distance/dissimilarity.
#[cfg(feature = "space")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Jaccard;

#[cfg(feature = "space")]
impl<const B: usize> Metric<BitArray<B>> for Jaccard {
    type Unit = u32;

    fn distance(&self, &a: &BitArray<B>, &b: &BitArray<B>) -> u32 {
        let intersection = (a & b).weight();
        let union = (a | b).weight();
        if union == 0 {
            0
        } else {
            (1.0 - intersection as f32 / union as f32).to_bits()
        }
    }
}
