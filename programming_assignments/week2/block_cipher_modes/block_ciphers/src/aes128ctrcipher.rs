use crate::BlockCipher;
use aes::Aes128;
use cipher::{generic_array::GenericArray, BlockEncrypt, BlockDecrypt, KeyInit};
use generic_array::typenum::U16;
use rand::Rng;

pub struct Aes128CtrCipher {
    key: GenericArray<u8, U16>,
}

impl Aes128CtrCipher {
    pub fn new(key: GenericArray<u8, U16>) -> Self {
        Self { key }
    }

    fn generate_iv() -> GenericArray<u8, U16> {
        let mut iv = GenericArray::<u8, U16>::default();
        rand::rng().fill(iv.as_mut_slice());
        iv
    }

    fn increment_iv(iv: &mut GenericArray<u8, U16>) {
        let len = iv.len();
        for i in (0..len).rev() {
            if iv[i] == 0b11111111 { continue; }
            iv[i] = iv[i].wrapping_add(1); 
            break;
        }
    }

    fn xor_arrays(arr1: &[u8], arr2: &[u8]) -> Vec<u8> {
        arr1.iter()
            .zip(arr2.iter()) // Pair elements from both arrays
            .map(|(a, b)| a ^ b) // Apply XOR operation
            .collect() // Collect results into a Vec<u8>
    }

}

impl BlockCipher for Aes128CtrCipher {
    fn encrypt(&self, plaintext: Vec<u8>) -> Vec<u8> {

        let cipher = Aes128::new(&self.key);
        let mut cipherBlocks: Vec<Vec<u8>> = Vec::new();
        let mut outBlock: Vec<u8> = Vec::new();
        let mut iv = Aes128CtrCipher::generate_iv();
        let mut encyptedIV: GenericArray<u8, U16> = iv.clone();

        cipher.encrypt_block(&mut encyptedIV);
        cipherBlocks.push(iv.to_vec());

        for (index, blockVec) in plaintext.chunks(16).enumerate() {

            outBlock = Aes128CtrCipher::xor_arrays(blockVec, &encyptedIV.to_vec());
            cipherBlocks.push(outBlock);

            Aes128CtrCipher::increment_iv(&mut iv);
            encyptedIV = iv.clone();
            cipher.encrypt_block(&mut encyptedIV);
        }

        let mut ciphertext: Vec<u8> = Vec::new();

        for cipherBlock in cipherBlocks {
            ciphertext.extend(cipherBlock.clone());
        }

        ciphertext
    }

    // The IV in ciphertext is not encrypted which is different to CBC mode
    fn decrypt(&self, ciphertext: Vec<u8>) -> Vec<u8> {
        let cipher = Aes128::new(&self.key);

        let mut plaintext: Vec<u8> = Vec::new();
        let mut plainBlocks: Vec<Vec<u8>> = Vec::new();
        let mut encyptedIV: GenericArray<u8, U16> = GenericArray::default();
        let mut iv: GenericArray<u8, U16> = GenericArray::default();

        for (index, blockVec) in ciphertext.chunks(16).enumerate() {
            if index == 0 { 
                // encrypt IV
                iv = GenericArray::clone_from_slice(blockVec);
                cipher.encrypt_block_b2b(&iv, &mut encyptedIV);
                continue;
            }

            plainBlocks.push(Aes128CtrCipher::xor_arrays(blockVec, &encyptedIV.to_vec()));

            Aes128CtrCipher::increment_iv(&mut iv);
            encyptedIV = iv;
            cipher.encrypt_block(&mut encyptedIV);
        }

        for plainBlock in plainBlocks {
            plaintext.extend(plainBlock.clone());
        }
        
        plaintext
    }
}
