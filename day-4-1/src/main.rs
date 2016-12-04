use std::io::prelude::*;
use std::fs::File;

fn main() {
    let input: Vec<String> = get_input_string()
        .unwrap_or(String::new())
        .lines()
        .collect();

    let mut sector_id_sum = 0;

    for line in input {
        let encrypted_name: String = line.split("[").take(1).collect();

        println!("name: {}", encrypted_name);
    }

}

fn get_input_string() -> Result<String, std::io::Error> {
    let mut input_file = File::open("../input/day-4.txt")?;
    let mut input_string = String::new();

    input_file.read_to_string(&mut input_string)?;

    Ok(input_string)
}
