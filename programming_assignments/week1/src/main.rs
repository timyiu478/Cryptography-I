use std::env;
use std::fs;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cipher_text_file_path = &args[1];
    let decrypt_target_file_path = &args[2];
    let mut cipher_texts = Vec::new();
    let target_cipher = fs::read_to_string(decrypt_target_file_path)
        .expect("Should have been able to read the file").into_bytes();
    let target_text: Vec<u8> = Vec::new();

    for line in fs::read_to_string(cipher_text_file_path).unwrap().lines() {
        cipher_texts.push(line.to_string().into_bytes());
    }

    for target_byte in target_cipher {
        for cipher in cipher_texts.iter() {
            for byte in cipher {
                println!("{byte}");
            }
        }
    }
}
