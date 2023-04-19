use img_compression::jpeg::{jpeg_compress_huffman, jpeg_expand_huffman};

fn main() {
    let original: Vec<u8> = vec![
        154, 192, 254, 239, 180, 128, 123, 110, 123, 180, 198, 180, 154, 136, 105, 136, 123, 136,
        154, 136, 136, 123, 110, 123, 123, 154, 154, 180, 167, 136, 149, 123, 123, 154, 180, 180,
        166, 154, 136, 123, 123, 154, 154, 166, 149, 180, 136, 136, 123, 136, 123, 123, 136, 198,
        180, 154, 136, 110, 123, 123, 136, 154, 166, 136,
    ];
    let res = jpeg_compress_huffman(original, 50);
    println!("{:?}", res.0);
    println!("{:?}", res.1);
    println!("{}", jpeg_expand_huffman(res.0, res.1, 50));
}
