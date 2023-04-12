use crate::{
    dct::{dct_2d, dct_2d_inv},
    huffman_coding::{huffman_compress, huffman_expand, Node},
    Matrix8x8f, Matrix8x8i, Matrix8x8u,
};

use std::cmp::Ordering::{Equal, Greater, Less};

fn gen_quantization_matrix(quality: u8) -> Matrix8x8u {
    Matrix8x8u::from_vec(
        vec![
            16.0, 12.0, 14.0, 14.0, 18.0, 24.0, 49.0, 72.0, 11.0, 12.0, 13.0, 17.0, 22.0, 35.0,
            64.0, 92.0, 10.0, 14.0, 16.0, 22.0, 37.0, 55.0, 78.0, 95.0, 16.0, 19.0, 24.0, 29.0,
            56.0, 64.0, 87.0, 98.0, 24.0, 26.0, 40.0, 51.0, 68.0, 81.0, 103.0, 112.0, 40.0, 58.0,
            57.0, 87.0, 109.0, 104.0, 121.0, 100.0, 51.0, 60.0, 69.0, 80.0, 103.0, 113.0, 120.0,
            103.0, 61.0, 55.0, 56.0, 62.0, 77.0, 92.0, 101.0, 99.0,
        ]
        .into_iter()
        .map(|x| match quality.cmp(&50) {
            Less => ((x * 50.0 / quality as f64).round() as u8).clamp(1, 255),
            Equal => x as u8,
            Greater => ((x * (100.0 - quality as f64) / 50.0).round() as u8).clamp(1, 255),
        })
        .collect(),
    )
}

fn quantize(d: Matrix8x8f, q: Matrix8x8u) -> Matrix8x8i {
    Matrix8x8i::from_vec(
        d.as_slice()
            .iter()
            .zip(q.as_slice().iter())
            .map(|(d_ij, q_ij)| (*d_ij / *q_ij as f32).round() as i8)
            .collect(),
    )
}

fn quantize_inv(c: Matrix8x8i, q: Matrix8x8u) -> Matrix8x8f {
    Matrix8x8f::from_vec(
        c.as_slice()
            .iter()
            .zip(q.as_slice().iter())
            .map(|(c_ij, q_ij)| (*c_ij as f32 * *q_ij as f32))
            .collect(),
    )
}

fn zigzag_traverse(c: Matrix8x8i) -> Vec<i8> {
    ZIGZAG_INDEX.into_iter().map(|x| c[x]).collect()
}

fn zigzag_reconstruct(zigzag: Vec<i8>) -> Matrix8x8i {
    let mut matrix: Matrix8x8i = Matrix8x8i::zeros();
    for (i, x) in zigzag.into_iter().enumerate() {
        matrix[ZIGZAG_INDEX[i]] = x;
    }
    matrix
}

pub fn jpeg_compress(img: Vec<u8>, quality: u8) -> (Vec<i8>, Box<Node>) {
    let m = Matrix8x8u::from_vec(img);
    let d = dct_2d(m);
    let q = gen_quantization_matrix(quality);
    let c = quantize(d, q);
    let zig = zigzag_traverse(c);
    huffman_compress(zig)
}

pub fn jpeg_expand(compresssed: Vec<i8>, tree: Box<Node>, quality: u8) -> Matrix8x8u {
    let q = gen_quantization_matrix(quality);
    let zig = huffman_expand(compresssed, tree);
    let c = zigzag_reconstruct(zig);

    let r = quantize_inv(c, q);
    dct_2d_inv(r)
}

const ZIGZAG_INDEX: [(usize, usize); 64] = [
    (0, 0),
    (0, 1),
    (1, 0),
    (2, 0),
    (1, 1),
    (0, 2),
    (0, 3),
    (1, 2),
    (2, 1),
    (3, 0),
    (4, 0),
    (3, 1),
    (2, 2),
    (1, 3),
    (0, 4),
    (0, 5),
    (1, 4),
    (2, 3),
    (3, 2),
    (4, 1),
    (5, 0),
    (6, 0),
    (5, 1),
    (4, 2),
    (3, 3),
    (2, 4),
    (1, 5),
    (0, 6),
    (0, 7),
    (1, 6),
    (2, 5),
    (3, 4),
    (4, 3),
    (5, 2),
    (6, 1),
    (7, 0),
    (7, 1),
    (6, 2),
    (5, 3),
    (4, 4),
    (3, 5),
    (2, 6),
    (1, 7),
    (2, 7),
    (3, 6),
    (4, 5),
    (5, 4),
    (6, 3),
    (7, 2),
    (7, 3),
    (6, 4),
    (5, 5),
    (4, 6),
    (3, 7),
    (4, 7),
    (5, 6),
    (6, 5),
    (7, 4),
    (7, 5),
    (6, 6),
    (5, 7),
    (6, 7),
    (7, 6),
    (7, 7),
];
#[cfg(test)]

mod test {
    use super::{
        dct_2d, gen_quantization_matrix, huffman_compress, huffman_expand, quantize,
        zigzag_traverse, Matrix8x8u,
    };
    #[test]
    fn jpeg_test() {
        let original: Vec<u8> = vec![
            154, 192, 254, 239, 180, 128, 123, 110, 123, 180, 198, 180, 154, 136, 105, 136, 123,
            136, 154, 136, 136, 123, 110, 123, 123, 154, 154, 180, 167, 136, 149, 123, 123, 154,
            180, 180, 166, 154, 136, 123, 123, 154, 154, 166, 149, 180, 136, 136, 123, 136, 123,
            123, 136, 198, 180, 154, 136, 110, 123, 123, 136, 154, 166, 136,
        ];

        let m = Matrix8x8u::from_vec(original);

        let d = dct_2d(m);
        let q50 = gen_quantization_matrix(50);

        let c = quantize(d, q50);

        println!("{}", c.clone());

        let zig = zigzag_traverse(c);

        let (msg, tree) = huffman_compress(zig.clone());
        println!("{:?}", msg);

        let exp = huffman_expand(msg, tree);

        assert!(zig == exp);
    }
}
