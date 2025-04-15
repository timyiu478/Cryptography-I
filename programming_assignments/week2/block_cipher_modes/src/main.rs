use block_ciphers::aes128cbccipher::Aes128CbcCipher;
use block_ciphers::aes128ctrcipher::Aes128CtrCipher;
use block_ciphers::BlockCipher;
use cipher::generic_array::GenericArray;
use generic_array::typenum::U16;


fn hex_to_vec(hex: &str) -> Vec<u8> {
    let mut bytes = Vec::new();

    // Iterate over the hex string in chunks of 2 characters
    for chunk in hex.as_bytes().chunks(2) {
        // Convert the chunk into a string slice
        let pair = std::str::from_utf8(chunk).expect("Invalid UTF-8");

        // Parse the hex pair into a u8
        let byte = u8::from_str_radix(pair, 16).expect("Invalid hex");
        bytes.push(byte);
    }

    bytes
}
fn hex_to_16_bytes_array(hex: &str) -> GenericArray<u8, U16> {
    let bytes = hex_to_vec(hex);

    GenericArray::from_exact_iter(bytes).expect("Conversion failed due to mismatched size")
}

fn test_aes_cbc() {
    let key = GenericArray::from([0u8; 16]);
    let plaintext = b"Hello, AES CBC Cipher!";

    let cipher = Aes128CbcCipher::new(key);
    let ciphertext = cipher.encrypt(plaintext.to_vec());
    let decryptedtext = cipher.decrypt(ciphertext.clone());
    let readable = String::from_utf8_lossy(&decryptedtext);

    println!("---- Text AES 128 CBC Cipher ----");
    println!("Plaintext: {:?}", plaintext);
    println!("Ciphertext: {:?}", ciphertext);
    println!("Decryptedtext: {:?}", decryptedtext);
    println!("Readable Decryptedtext: {:?}", readable);
}

fn test_aes_ctr() {
    let key = GenericArray::from([0u8; 16]);
    let plaintext = b"Hello, AES CTR Cipher!";

    let cipher = Aes128CtrCipher::new(key);
    let ciphertext = cipher.encrypt(plaintext.to_vec());
    let decryptedtext = cipher.decrypt(ciphertext.clone());
    let readable = String::from_utf8_lossy(&decryptedtext);

    println!("---- Text AES 128 CTR Cipher ----");
    println!("Plaintext: {:?}", plaintext);
    println!("Ciphertext: {:?}", ciphertext);
    println!("Decryptedtext: {:?}", decryptedtext);
    println!("Readable Decryptedtext: {:?}", readable);
}

fn main() {
    test_aes_cbc();
    test_aes_ctr();

    // Q1
    let q1_cbc_key: GenericArray<u8, U16> = hex_to_16_bytes_array("140b41b22a29beb4061bda66b6747e14");
    let q1_ciphertext: Vec<u8> = hex_to_vec("4ca00ff4c898d61e1edbf1800618fb2828a226d160dad07883d04e008a7897ee2e4b7465d5290d0c0e6c6822236e1daafb94ffe0c5da05d9476be028ad7c1d81");
    let q1_cipher = Aes128CbcCipher::new(q1_cbc_key);
    println!("Q1: {:?}", String::from_utf8_lossy(&q1_cipher.decrypt(q1_ciphertext.clone())));

    // Q2
    let q2_cbc_key: GenericArray<u8, U16> = hex_to_16_bytes_array("140b41b22a29beb4061bda66b6747e14");
    let q2_ciphertext: Vec<u8> = hex_to_vec("5b68629feb8606f9a6667670b75b38a5b4832d0f26e1ab7da33249de7d4afc48e713ac646ace36e872ad5fb8a512428a6e21364b0c374df45503473c5242a253");
    let q2_cipher = Aes128CbcCipher::new(q2_cbc_key);
    println!("Q2: {:?}", String::from_utf8_lossy(&q2_cipher.decrypt(q2_ciphertext.clone())));


    // Q3
    let q3_ctr_key: GenericArray<u8, U16> = hex_to_16_bytes_array("36f18357be4dbd77f050515c73fcf9f2");
    let q3_ciphertext: Vec<u8> = hex_to_vec("69dda8455c7dd4254bf353b773304eec0ec7702330098ce7f7520d1cbbb20fc388d1b0adb5054dbd7370849dbf0b88d393f252e764f1f5f7ad97ef79d59ce29f5f51eeca32eabedd9afa9329");
    let q3_cipher = Aes128CtrCipher::new(q3_ctr_key);
    println!("Q3: {:?}", String::from_utf8_lossy(&q3_cipher.decrypt(q3_ciphertext.clone())));

    // Q4
    let q4_ctr_key: GenericArray<u8, U16> = hex_to_16_bytes_array("36f18357be4dbd77f050515c73fcf9f2");
    let q4_ciphertext: Vec<u8> = hex_to_vec("770b80259ec33beb2561358a9f2dc617e46218c0a53cbeca695ae45faa8952aa0e311bde9d4e01726d3184c34451");
    let q4_cipher = Aes128CtrCipher::new(q4_ctr_key);
    println!("Q4: {:?}", String::from_utf8_lossy(&q4_cipher.decrypt(q4_ciphertext.clone())));
}
