use std::io::prelude::*;
use std::fs::File;

fn main() {
    let input = get_input_string().unwrap_or(String::new());
    let input: Vec<&str> = input.lines()
        .collect();

    for line in input {
        let name_and_sector_id: String = line.split("[")
            .take(1)
            .collect();

        let sector_id: &str = name_and_sector_id.split("-")
            .last()
            .unwrap_or("");

        let name_with_dashes: String = name_and_sector_id.split(sector_id)
            .take(1)
            .collect();

        let name: String = name_with_dashes.split("-").collect();

        let checksum = find_checksum(name);

        let temp: String = line.split("[")
            .skip(1)
            .collect();

        if temp.contains(&checksum) {
            let sector_id: u32 = sector_id.parse().unwrap_or(0u32);

            print!("{} ", sector_id);
            for character in name_with_dashes.chars() {
                if character == '-' {
                    print!(" ");
                } else {
                    print!("{}", rotate_forward(character, sector_id));
                }

            }
            println!("");
        }
    }
}

fn rotate_forward(character: char, amt: u32) -> char {
    let u8_char = character as u8;
    let mut u32_char = u8_char as u32;
    u32_char -= 97;
    u32_char += amt;
    u32_char %= 26;
    u32_char += 97;
    let u8_char = u32_char as u8;
    u8_char as char
}

fn find_checksum(name: String) -> String {
    let mut chars_found: Vec<(char, u32)> = Vec::new();

    for character in name.chars() {
        let pos = chars_found.iter()
            .position(|&tuple| {
                tuple.0 == character
            });

        match pos {
            Some(i) => chars_found[i].1 += 1,
            None => chars_found.push((character, 1)),
        }
    }

    chars_found.sort_by(|&a, &b| {
        if a.1 != b.1 {
            b.1.cmp(&a.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let checksum: String = chars_found.iter()
        .take(5)
        .map(|x| x.0)
        .collect();

    checksum
}

fn get_input_string() -> Result<String, std::io::Error> {
    let mut input_file = File::open("../input/day-4.txt")?;
    let mut input_string = String::new();

    input_file.read_to_string(&mut input_string)?;

    Ok(input_string)
}
