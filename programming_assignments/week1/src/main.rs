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

    for line in fs::read_to_string(cipher_text_file_path).unwrap().lines() {
        cipher_texts.push(line.to_string().into_bytes());
    }

    for cipher in cipher_texts.iter() {
        let cipher_str = str::from_utf8(&cipher).unwrap();
        println!("{cipher_str}");
    }
    
    let target_cipher_str = str::from_utf8(&target_cipher).unwrap();
    println!("{target_cipher_str}");
}
