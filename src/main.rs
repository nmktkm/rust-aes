mod aes;
use aes::*;
use std::time;

fn main() {
    let encryption = AES128::new_from_str("Tachibana_Hinano");
    let data = [0x00u8; 16];
    let now = time::Instant::now();
    let cipher = (encryption.encrypt)(&encryption, &data);
    println!("{:?}", now.elapsed());
    print!("{:X?}", cipher);
}