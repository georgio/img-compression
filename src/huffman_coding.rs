use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    freq: i64,
    character: Option<i8>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn new_node(freq: i64, character: Option<i8>) -> Node {
    Node {
        freq,
        character,
        left: None,
        right: None,
    }
}

fn new_box(n: Node) -> Box<Node> {
    Box::new(n)
}

fn frequency(s: &Vec<i8>) -> HashMap<i8, i64> {
    let mut h = HashMap::new();
    for character in s {
        let counter = h.entry(*character).or_insert(0);
        *counter += 1;
    }
    h
}

fn assign_codes(p: &Box<Node>, h: &mut HashMap<i8, Vec<i8>>, mut s: Vec<i8>) {
    if let Some(character) = p.character {
        h.insert(character, s);
    } else {
        if let Some(ref l) = p.left {
            let mut tmp = s.clone();
            tmp.push(0);
            assign_codes(l, h, tmp);
        }
        if let Some(ref r) = p.right {
            s.push(1);
            assign_codes(r, h, s);
        }
    }
}

fn encode_string(original_data: &Vec<i8>, h: &HashMap<i8, Vec<i8>>) -> Vec<i8> {
    let mut r: Vec<i8> = vec![];
    let mut t: Option<&Vec<i8>>;

    for character in original_data {
        t = h.get(character);
        r.extend(t.unwrap());
    }
    r
}

fn decode_string(compressed_data: &Vec<i8>, root: &Box<Node>) -> Vec<i8> {
    let mut output: Vec<i8> = vec![];
    let mut node_pointer = root;

    for x in compressed_data {
        if *x == 0 {
            if let Some(ref left) = node_pointer.left {
                node_pointer = left;
            }
        } else if let Some(ref right) = node_pointer.right {
            node_pointer = right;
        }

        if let Some(token) = node_pointer.character {
            output.push(token);
            node_pointer = root;
        }
    }
    output
}

pub(crate) fn huffman_encode(msg: Vec<i8>) -> (Vec<i8>, Box<Node>) {
    let h = frequency(&msg);

    let mut p: Vec<Box<Node>> = h
        .iter()
        .map(|x| new_box(new_node(*(x.1), Some(*(x.0)))))
        .collect();
    while p.len() > 1 {
        p.sort_by(|a, b| b.freq.cmp(&(a.freq)));
        let a = p.pop().unwrap();
        let b = p.pop().unwrap();
        let mut c = new_box(new_node(a.freq + b.freq, None));
        c.left = Some(a);
        c.right = Some(b);
        p.push(c);
    }

    let root = p.pop().unwrap();
    let mut h: HashMap<i8, Vec<i8>> = HashMap::new();

    assign_codes(&root, &mut h, vec![]);

    let enc = encode_string(&msg, &h);

    (enc, root)
}

pub(crate) fn huffman_decode(compressed: Vec<i8>, tree: Box<Node>) -> Vec<i8> {
    decode_string(&compressed, &tree)
}

#[cfg(test)]
mod test {
    use super::{huffman_decode, huffman_encode};
    #[test]
    fn huffman_correctness() {
        let msg: Vec<i8> = vec![2, 3, 5, 109, 2, 127, 49, 49];
        let (enc, tree) = huffman_encode(msg.clone());

        let exp = huffman_decode(enc.clone(), tree);
        println!("original({:?} bytes): {:?}", msg.len(), &msg);
        println!("encoded({:?} bits): {:?}", enc.len(), &enc);
        println!("decoded({:?} bytes): {:?}", exp.len(), exp.clone());
        assert_eq!(exp, msg);
    }
}
