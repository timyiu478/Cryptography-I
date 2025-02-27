use std::env;
use std::fs;
use std::str;
use hex;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
struct XorPair {
    a: usize,
    b: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cipher_text_file_path = &args[1];
    let decrypt_target_file_path = &args[2];
    let mut cipher_texts = Vec::new();

    for line in fs::read_to_string(cipher_text_file_path).unwrap().lines() {
        cipher_texts.push(hex::decode(line.to_string()).unwrap());
    }
    let target_cipher_text = fs::read_to_string(decrypt_target_file_path).unwrap();
    let target_cipher = hex::decode(&target_cipher_text[0..target_cipher_text.len()-2]).unwrap();

    let mut xor_map: HashMap<XorPair, Vec<u8>> = HashMap::new();
    let mut az_counter_map: HashMap<usize, Vec<f64>> = HashMap::new();


    // init az_counter_map
    for i in 0..cipher_texts.len() {
        let az_counter:Vec<f64> = vec![0.0; target_cipher.len()]; 
        az_counter_map.insert(i, az_counter);
    }
    let az_counter:Vec<f64> = vec![0.0; target_cipher.len()]; 
    az_counter_map.insert(cipher_texts.len(), az_counter);

    // xor cipher_texts[i] and cipher_texts[j]
    for i in 0..cipher_texts.len() {
        for j in 0..cipher_texts.len() {
            if i == j {
                continue;
            }
            let mut plain_text: Vec<u8> = Vec::new();
            for k in 0..target_cipher.len() {
                let b = cipher_texts[j][k] ^ cipher_texts[i][k];
                let az_counter = az_counter_map.get_mut(&i).unwrap();
                if b >= 65 && b <= 90 || b >= 97 && b <= 122{
                    az_counter[k] += 1.0;
                }
                plain_text.push(b);
            }
            let xor_pair: XorPair = XorPair{a: i, b: j};
            xor_map.insert(xor_pair, plain_text);
        }
        // xor cipher_texts[i] and target_cipher = cipher_texts[cipher_texts.len()]
        let mut plain_text: Vec<u8> = Vec::new();
        for k in 0..target_cipher.len() {
            let b = target_cipher[k] ^ cipher_texts[i][k];
            let az_counter_1 = az_counter_map.get_mut(&i).unwrap();
            if b >= 65 && b <= 90 || b >= 97 && b <= 122{
                az_counter_1[k] += 1.0;
            }
            let az_counter_2 = az_counter_map.get_mut(&cipher_texts.len()).unwrap();
            if b >= 65 && b <= 90 || b >= 97 && b <= 122{
                az_counter_2[k] += 1.0;
            }
            plain_text.push(b);
        }
        let xor_pair: XorPair = XorPair{a: i, b: cipher_texts.len()};
        xor_map.insert(xor_pair, plain_text);
    }

    let mut target_plain_text: Vec<u8> = Vec::new();
    for k in 0..target_cipher.len() {
        let mut max_count = 0.0;
        let mut max_count_index = cipher_texts.len();

        for (cipher_index, az_counter) in az_counter_map.iter() {
            if az_counter[k] > max_count {
                max_count = az_counter[k];
                max_count_index = *cipher_index;
            }
        }
        if max_count_index < cipher_texts.len() {
            let plain_text = xor_map.get(&XorPair{a: max_count_index, b: cipher_texts.len()}).unwrap();
            target_plain_text.push(plain_text[k]);
            // reset counter so that the next lopp will select the second max count index
            let az_counter = az_counter_map.get_mut(&max_count_index).unwrap();
            az_counter[k] = 0.0;
        } else {
            target_plain_text.push(32);
        }
    }
    
    let target_plain_text_str: &str = str::from_utf8(&target_plain_text).unwrap();
    println!("{target_plain_text_str}");
}
