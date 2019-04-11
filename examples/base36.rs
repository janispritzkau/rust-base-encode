extern crate base_encode;

const CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let buf: Vec<u8> = (0..16).map(|_| rand::random()).collect();

    let encoded = base_encode::to_string(&buf, 36, CHARS).unwrap();
    assert_eq!(buf, base_encode::from_str(&encoded, 36, CHARS).unwrap());

    println!("{}", encoded);
}
