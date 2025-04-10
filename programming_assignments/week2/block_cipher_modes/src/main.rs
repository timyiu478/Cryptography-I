use aes::Aes128;
use cipher::{BlockEncrypt, KeyInit};

fn main() {
    let key = [0u8; 16]; // Example key
    let block = [0u8; 16]; // Example block
    let cipher = Aes128::new(&key.into());

    let mut block_copy = block.into(); // Convert block to GenericArray
    cipher.encrypt_block(&mut block_copy);
    println!("Encrypted block: {:?}", block_copy);
}
