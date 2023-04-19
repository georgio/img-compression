pub(crate) mod dct;
pub(crate) mod huffman_coding;
pub mod jpeg;
pub mod rice_golomb_coding;

use nalgebra::SMatrix;

pub(crate) type Matrix8x8f = SMatrix<f32, 8, 8>;
pub type Matrix8x8u = SMatrix<u8, 8, 8>;
pub(crate) type Matrix8x8i = SMatrix<i8, 8, 8>;
