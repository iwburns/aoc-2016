use std::io::prelude::*;
use std::fs::File;

fn main() {
    let input = get_input_string().unwrap_or(String::new());

    let mut num_found = 0u32;

    'a: for line in input.lines() {
        let line_parts: Vec<&str> = line.split('[').collect();
        for line_part in line_parts {

            let bracket_split: Vec<&str> = line_part.split(']').collect();
            if bracket_split.len() > 1 {
                // [0] is the part inside the brackets.
                for window in bracket_split[0].chars().collect::<Vec<char>>().windows(4) {
                    if window[0] == window[3] && window[1] == window[2] && window[0] != window[1] {
                        // don't count this line, we found an abba section in the brackets.
                        continue 'a;
                    }
                }
            }
        }
        let line_parts: Vec<&str> = line.split('[').collect();
        for line_part in line_parts {

            let bracket_split: Vec<&str> = line_part.split(']').collect();
            if bracket_split.len() > 1 {
                // [1] is the part inside the brackets.
                for window in bracket_split[1].chars().collect::<Vec<char>>().windows(4) {
                    if window[0] == window[3] && window[1] == window[2] && window[0] != window[1] {
                        num_found += 1;
                        // skip the rest of this line, we don't want to over count
                        continue 'a;
                    }
                }
            } else {
                // [0] is the part inside the brackets.
                for window in bracket_split[0].chars().collect::<Vec<char>>().windows(4) {
                    if window[0] == window[3] && window[1] == window[2] && window[0] != window[1] {
                        num_found += 1;
                        // skip the rest of this line, we don't want to over count
                        continue 'a;
                    }
                }
            }
        }

    }

    println!("num_found: {}", num_found);

}

fn get_input_string() -> Result<String, std::io::Error> {
    let mut input_file = File::open("../input/day-7.txt")?;
    let mut input_string = String::new();

    input_file.read_to_string(&mut input_string)?;

    Ok(input_string)
}

