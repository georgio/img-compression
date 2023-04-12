use crate::{Matrix8x8f, Matrix8x8u};

fn reduce_128(mat: Matrix8x8u) -> Matrix8x8f {
    let elements: Vec<u8> = Vec::from(mat.as_slice());
    Matrix8x8f::from_vec(
        elements
            .into_iter()
            .map(|x| ((x as i64) - 128) as f32)
            .collect(),
    )
}

fn expand_128(mat: Matrix8x8f) -> Matrix8x8u {
    Matrix8x8u::from_vec(
        Vec::from(mat.as_slice())
            .into_iter()
            .map(|x| (x.round() as u8) + 128)
            .collect(),
    )
}

pub(crate) fn dct_2d(original: Matrix8x8u) -> Matrix8x8f {
    let t = gen_t();
    let m = reduce_128(original);
    t * m * t.transpose()
}

pub(crate) fn dct_2d_inv(compressed: Matrix8x8f) -> Matrix8x8u {
    let t = gen_t();
    let intermediate = t.transpose() * compressed * t;
    expand_128(intermediate)
}

fn gen_t() -> Matrix8x8f {
    let t = vec![
        0.3536, 0.4904, 0.4619, 0.4157, 0.3536, 0.2778, 0.1913, 0.0975, 0.3536, 0.4157, 0.1913,
        -0.0975, -0.3536, -0.4904, -0.4619, -0.2778, 0.3536, 0.2778, -0.1913, -0.4904, -0.3536,
        0.0975, 0.4619, 0.4157, 0.3536, 0.0975, -0.4619, -0.2778, 0.3536, 0.4157, -0.1913, -0.4904,
        0.3536, -0.0975, -0.4619, 0.2778, 0.3536, -0.4157, -0.1913, 0.4904, 0.3536, -0.2778,
        -0.1913, 0.4904, -0.3536, -0.0975, 0.4619, -0.4157, 0.3536, -0.4157, 0.1913, 0.0975,
        -0.3536, 0.4904, -0.4619, 0.2778, 0.3536, -0.4904, 0.4619, -0.4157, 0.3536, -0.2778,
        0.1913, -0.0975,
    ];
    Matrix8x8f::from_vec(t)
}
