use std::io::prelude::*;
use std::fs::File;

fn main() {
    let input = get_input_string().unwrap_or(String::new());

    let mut num_found = 0u32;

    'a: for line in input.lines() {
        let (outside_brackets, inside_brackets) = split_line_by_brackets(line);

        for section in inside_brackets {
            let abbas = get_abbas(section);
            if abbas.len() > 0 {
                continue 'a;
            }
        }

        for section in outside_brackets {
            let abbas = get_abbas(section);
            if abbas.len() > 0 {
                num_found += 1;
                continue 'a;
            }
        }
    }

    println!("num_found: {}", num_found);
}

fn get_abbas(string: &str) -> Vec<String> {
    let mut abbas = Vec::<String>::new();

    for window in string.chars().collect::<Vec<char>>().windows(4) {
        if is_abba_window(window) {
            abbas.push(window.iter().cloned().collect());
        }
    }

    abbas
}

fn is_abba_window(window: &[char]) -> bool {
    window[0] == window[3] && window[1] == window[2] && window[0] != window[1]
}

fn split_line_by_brackets(line: &str) -> (Vec<&str>, Vec<&str>) {
    let parts: Vec<&str> = line.split(|x| x == '[' || x == ']').collect();

    let mut outside_parts: Vec<&str> = Vec::new();
    let mut inside_parts: Vec<&str> = Vec::new();

    for i in 0..parts.len() {
        if i % 2 == 0 {
            outside_parts.push(parts[i]);
        } else {
            inside_parts.push(parts[i]);
        }
    }

    (outside_parts, inside_parts)
}

fn get_input_string() -> Result<String, std::io::Error> {
    let mut input_file = File::open("../input/day-7.txt")?;
    let mut input_string = String::new();

    input_file.read_to_string(&mut input_string)?;

    Ok(input_string)
}

