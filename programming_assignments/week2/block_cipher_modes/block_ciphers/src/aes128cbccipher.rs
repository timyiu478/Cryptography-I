use crate::BlockCipher;
use aes::Aes128;
use cipher::{generic_array::GenericArray, BlockEncrypt, BlockDecrypt, KeyInit};
use generic_array::typenum::U16;
use rand::Rng;

pub struct Aes128CbcCipher {
    key: GenericArray<u8, U16>,
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
    fn pkcs5Padding(plaintext: Vec<u8>) -> Vec<u8> {
        let blockSize = 16;
        let mut plaintextWithPadding: Vec<u8> = plaintext.clone();
        let paddingLength = blockSize - (plaintext.len() % blockSize);

        plaintextWithPadding.extend(vec![paddingLength as u8; paddingLength]);

        plaintextWithPadding
    }

    fn pkcs5DePadding(plaintextWithPadding: Vec<u8>) -> Vec<u8> {
        let mut plaintext: Vec<u8> = plaintextWithPadding.clone();
        let paddingLength: usize = usize::from(*plaintext.last().unwrap());

        for i in 1..paddingLength {
            if plaintextWithPadding[plaintext.len() - 1 - i] != *plaintext.last().unwrap() {
                panic!("De padding error");
            }
        }

        plaintext.truncate(plaintext.len() - paddingLength);

        plaintext
    }

    fn xor_generic_arrays(
        arr1: GenericArray<u8, U16>,
        arr2: GenericArray<u8, U16>,
    ) -> GenericArray<u8, U16> {
        GenericArray::from_exact_iter(
            arr1.iter()
                .zip(arr2.iter()) // Pair elements from both arrays
                .map(|(a, b)| a ^ b), // Apply XOR operation to each pair
        )
        .expect("GenericArray must have the exact size")
    }
}

impl BlockCipher for Aes128CbcCipher {
    fn encrypt(&self, plaintext: Vec<u8>) -> Vec<u8> {
        let iv = Aes128CbcCipher::generate_iv();
        let plaintextWithPadding = Aes128CbcCipher::pkcs5Padding(plaintext);

        let cipher = Aes128::new(&self.key);
        let mut cipherBlocks: Vec<GenericArray<u8, U16>> = Vec::new();
        let mut inBlock: GenericArray<u8, U16> = GenericArray::default();
        let mut outBlock: GenericArray<u8, U16> = GenericArray::default();
        let mut encyptedIV = iv;

        // Don't use in PROD; the IV should be encrypted by a seperate key to prevent chosen plaintext attack
        cipher.encrypt_block(&mut encyptedIV);
        cipherBlocks.push(encyptedIV);

        for (index, blockVec) in plaintextWithPadding.chunks(16).enumerate() {
            let block: GenericArray<u8, U16> = GenericArray::clone_from_slice(blockVec);

            let previousBlock = cipherBlocks.last().expect("No previous block");
            inBlock = Aes128CbcCipher::xor_generic_arrays(*previousBlock, block);

            cipher.encrypt_block_b2b(&inBlock, &mut outBlock);

            cipherBlocks.push(outBlock);
        }

        let mut ciphertext: Vec<u8> = Vec::new();

        for cipherBlock in cipherBlocks {
            ciphertext.extend(cipherBlock.iter().copied());
        }

        ciphertext
    }

    // The 16-byte encryption IV is chosen at random and is prepended to the ciphertext
    fn decrypt(&self, ciphertext: Vec<u8>) -> Vec<u8> {
        let cipher = Aes128::new(&self.key);

        let mut plaintext: Vec<u8> = Vec::new();
        let mut plainBlocks: Vec<GenericArray<u8, U16>> = Vec::new();
        let mut inBlock: GenericArray<u8, U16> = GenericArray::default();
        let mut outBlock: GenericArray<u8, U16> = GenericArray::default();
        let mut prevBlock: GenericArray<u8, U16> = GenericArray::default();

        for (index, blockVec) in ciphertext.chunks(16).enumerate() {

            inBlock = GenericArray::clone_from_slice(blockVec);

            if index == 0 { 
                // get the encrypted IV
                prevBlock = inBlock;
                continue;
            }

            cipher.decrypt_block_b2b(&inBlock, &mut outBlock);

            plainBlocks.push(Aes128CbcCipher::xor_generic_arrays(prevBlock, outBlock));

            prevBlock = inBlock;
        }

        for plainBlock in plainBlocks {
            plaintext.extend(plainBlock.iter().copied());
        }
        
        plaintext = Aes128CbcCipher::pkcs5DePadding(plaintext);

        plaintext
    }
}
