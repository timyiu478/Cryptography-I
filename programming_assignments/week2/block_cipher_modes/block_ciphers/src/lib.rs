// Define a trait for block cipher functionality
pub trait BlockCipher {
    fn encrypt(&self, plaintext: &[u8]) -> Vec<u8>;
    fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8>;
}

pub mod aes128cbccipher;
