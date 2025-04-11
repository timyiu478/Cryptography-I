use crate::BlockCipher;
use rand::Rng;
use aes::Aes128;
use generic_array::{typenum::U16};
use cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray};

pub struct Aes128CbcCipher {
    key: GenericArray<u8, U16>
}

impl Aes128CbcCipher {
    pub fn new(key: GenericArray<u8, U16>) -> Self {
        Self { key }
    }

    fn generate_iv() -> GenericArray<u8, U16> {
        let mut iv = GenericArray::<u8, U16>::default();
        rand::rng().fill(iv.as_mut_slice());
        iv
    }

    // Pad the plaintext by PKCS5 padding method
    //
    // Ref: https://www.cryptosys.net/pki/manpki/pki_paddingschemes.html
    fn pkcs5Padding(plaintext: &[u8]) -> Vec<u8> {
        let blockSize = 16;
        let mut plaintextWithPadding: Vec<u8> = plaintext.to_vec();
        let paddingLength = blockSize - (plaintext.len() % blockSize); 

        plaintextWithPadding.extend(vec![paddingLength as u8; paddingLength]);

        plaintextWithPadding
    }

    fn xor_generic_arrays(arr1: GenericArray<u8, U16>, arr2: GenericArray<u8, U16>) -> GenericArray<u8, U16> {
        GenericArray::from_exact_iter(
            arr1.iter()
                .zip(arr2.iter()) // Pair elements from both arrays
                .map(|(a, b)| a ^ b) // Apply XOR operation to each pair
        ).expect("GenericArray must have the exact size")
    }
}

impl BlockCipher for Aes128CbcCipher {

    fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        let iv = Aes128CbcCipher::generate_iv();
        let plaintextWithPadding = Aes128CbcCipher::pkcs5Padding(plaintext);

        let cipher = Aes128::new((&self.key));
        let mut cipherBlocks:Vec<GenericArray<u8, U16>> = Vec::new();
        let mut inBlock: GenericArray<u8, U16> = GenericArray::default();
        let mut outBlock: GenericArray<u8, U16> = GenericArray::default();

        cipherBlocks.push(iv);

        for (index, blockVec) in plaintextWithPadding.chunks(16).enumerate() {
            let block: GenericArray<u8, U16> = GenericArray::clone_from_slice(blockVec);

            if index == 0 {
                let mut encyptedIV = iv;
                cipher.encrypt_block(&mut encyptedIV);

                inBlock = Aes128CbcCipher::xor_generic_arrays(encyptedIV, block);
            } else {
                let previousBlock = cipherBlocks.last().expect("No previous block");
                inBlock = Aes128CbcCipher::xor_generic_arrays(*previousBlock, block);
            }

            cipher.encrypt_block_b2b(&inBlock, &mut outBlock);

            cipherBlocks.push(outBlock);
        }

        let mut ciphertext: Vec<u8> = Vec::new();

        for cipherBlock in cipherBlocks {
            ciphertext.extend(cipherBlock.iter().copied());
        }

        ciphertext
    }

    fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8> {
        let plaintext: Vec<u8> = Vec::new();

        plaintext
    }
}
