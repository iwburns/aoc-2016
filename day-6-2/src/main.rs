use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let input = get_input_string().unwrap_or(String::new());
    let mut lines = input.lines();
    let mut maps = Vec::new();

    let num_chars_per_line = lines.next().unwrap().chars().count();

    for _ in 0..num_chars_per_line {
        maps.push(HashMap::<char, u32>::new());
    }

    let lines = input.lines();

    for line in lines {
        for (i, character) in line.chars().enumerate() {
            let count = maps[i].entry(character).or_insert(0);
            *count += 1;
        }
    }

    let mut temp_vec: Vec<(char, u32)>;

    for map in maps {
        temp_vec = Vec::new();

        for (key, value) in map {
            temp_vec.push((key, value));
        }

        temp_vec.sort_by(|&a, &b| {
            a.1.cmp(&b.1)
        });

        print!("{}", temp_vec[0].0);
    }
}

fn get_input_string() -> Result<String, std::io::Error> {
    let mut input_file = File::open("../input/day-6.txt")?;
    let mut input_string = String::new();

    input_file.read_to_string(&mut input_string)?;

    Ok(input_string)
}
