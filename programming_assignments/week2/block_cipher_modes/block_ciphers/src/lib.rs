// Define a trait for block cipher functionality
pub trait BlockCipher {
    fn encrypt(&self, plaintext: Vec<u8>) -> Vec<u8>;
    fn decrypt(&self, ciphertext: Vec<u8>) -> Vec<u8>;
}

pub mod aes128cbccipher;
pub mod aes128ctrcipher;
