use block_ciphers::BlockCipher;
use block_ciphers::aes128cbccipher::Aes128CbcCipher;
use cipher::{generic_array::GenericArray};

fn main() {
    let key = GenericArray::from([0u8; 16]);
    let plaintext = b"Hello, AES CBC Cipher!"; // Example plaintext

    let cipher = Aes128CbcCipher::new(key);
    let ciphertext = cipher.encrypt(plaintext);

    println!("Ciphertext: {:?}", ciphertext);
}
