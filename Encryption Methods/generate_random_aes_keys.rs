/*
Generate Random AES KEY , IV 
For More Codes : https://github.com/Whitecat18/Rust-for-Malware-Development.git
@5mukx
*/

use rand::RngCore;
use aes::Aes256;

fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    key
}

fn generate_iv() -> [u8; 16] {
    let mut iv = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut iv);
    iv
}

fn main() {
    let key = generate_key();
    let iv = generate_iv();

    println!("Generated AES key: {:?}", key);
    println!("Generated IV: {:?}", iv);
}
