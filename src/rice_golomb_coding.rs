// This should be as close as possible to the sqrt of the largest value we will encode
const M: u8 = 4;
const B: i8 = 1 << M;

pub fn rice_golomb_encode(data: Vec<i8>) -> Vec<bool> {
    data.into_iter().fold(vec![], |mut acc, value| {
        let quotient = value / B;
        let remainder = value % B;
        for _ in 0..quotient {
            acc.push(false);
        }
        acc.push(true);
        for i in (0..M).rev() {
            acc.push((remainder >> i) & 1 != 0);
        }
        return acc;
    })
}

pub(crate) fn rice_golomb_decode(data: Vec<bool>) -> Vec<i8> {
    let mut result = vec![];
    let mut idx = 0;
    while idx < data.len() {
        let mut quotient = 0;
        while data[idx] == false {
            quotient += 1;
            idx += 1;
        }
        idx += 1;
        let mut remainder = 0;
        for i in (0..M).rev() {
            remainder |= (data[idx] as i32) << i;
            idx += 1;
        }
        result.push(quotient * B + remainder as i8);
    }
    result
}

#[cfg(test)]
mod test {
    use super::{rice_golomb_decode, rice_golomb_encode};
    #[test]
    fn rice_golomb_correctness() {
        let msg: Vec<i8> = vec![2, 3, 5, 109, 2, 127, 49, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let enc = rice_golomb_encode(msg.clone());

        let exp = rice_golomb_decode(enc.clone());
        println!("original({:?} bytes): {:?}", msg.len(), &msg);
        println!("encoded({:?} bits): {:?}", enc.len(), &enc);
        println!("decoded({:?} bytes): {:?}", exp.len(), exp.clone());
        assert_eq!(exp, msg);
    }
}
