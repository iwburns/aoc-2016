use std::io::prelude::*;
use std::fs::File;

fn main() {
    let input = get_input_string().unwrap_or(String::new());
    let input: Vec<&str> = input.lines()
        .collect();

    let mut sector_id_sum = 0;

    for line in input {
        let name_and_sector_id: String = line.split("[")
            .take(1)
            .collect();

        let sector_id: &str = name_and_sector_id.split("-")
            .last()
            .unwrap_or("");

        let name: String = name_and_sector_id.split(sector_id)
            .take(1)
            .collect();

        let name: String = name.split("-").collect();

        let checksum = find_checksum(name);

        let temp: String = line.split("[")
            .skip(1)
            .collect();

        if temp.contains(&checksum) {
            sector_id_sum += sector_id.parse().unwrap_or(0u32);
        }
    }

    println!("sector_id_sum: {}", sector_id_sum);

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
