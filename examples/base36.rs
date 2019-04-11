extern crate base_encode;

const CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let buf: Vec<u8> = (0..16).map(|_| rand::random()).collect();

    let encoded: String = base_encode::encode(&buf, 36).iter().map(|&x| CHARS[x as usize] as char).collect();
    println!("{}", encoded);
}
