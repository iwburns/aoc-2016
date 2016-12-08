use std::io::prelude::*;
use std::fs::File;

fn main() {
    let input = get_input_string().unwrap_or(String::new());

    let mut num_found = 0u32;

    'a: for line in input.lines() {
        let (outside_brackets, inside_brackets) = split_line_by_brackets(line);

        let mut outside_abas = Vec::new();
        for section in outside_brackets {
            let mut abas = get_abas(section);
            outside_abas.append(&mut abas);
        }

        let mut inside_abas = Vec::new();
        for section in inside_brackets {
            let mut abas = get_abas(section);
            inside_abas.append(&mut abas);
        }

        for aba in outside_abas {
            let bab = invert_aba(aba.clone());
            if inside_abas.contains(&bab) {
                num_found += 1;
                continue 'a;
            }
        }
    }

    println!("num_found: {}", num_found);
}

fn invert_aba(aba: String) -> String {
    let chars: Vec<char> = aba.chars().collect();
    let mut bab = String::new();
    bab.push(chars[1]);
    bab.push(chars[0]);
    bab.push(chars[1]);
    bab
}

fn get_abas(string: &str) -> Vec<String> {
    let mut abas = Vec::<String>::new();

    for window in string.chars().collect::<Vec<char>>().windows(3) {
        if is_aba_window(window) {
            abas.push(window.iter().cloned().collect());
        }
    }

    abas
}

fn is_aba_window(window: &[char]) -> bool {
    window[0] == window[2] && window[0] != window[1]
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

