extern crate crypto;

use crypto::md5::*;
use crypto::digest::Digest;

fn main() {
    let input = "ffykfhsq".to_string();

    let mut hasher = Md5::new();

    let mut hash_input = String::new();
    let mut hash_output = String::new();
    let mut password: [char; 8] = [' '; 8];
    let mut int_index = 0u64;

    for _ in 0..8 {

        loop {
            hash_input = input.clone() + &int_index.to_string();

            hasher.input(hash_input.as_bytes());

            hash_output = hasher.result_str();
            hasher.reset();
            int_index += 1;

            if hash_output.starts_with("00000") {

                if let Ok(index) = hash_output[5..6].parse::<usize>() {

                    if index < 8 && password[index] == ' ' {
                        password[index] = hash_output.chars().nth(6).unwrap();
                        break;
                    }
                }
            }
        }

        let temp: String = password.iter().cloned().collect();
        println!("pass: {}", temp);
    }
}
