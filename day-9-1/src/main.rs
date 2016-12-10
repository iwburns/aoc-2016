use std::io::prelude::*;
use std::fs::File;

struct Marker {
    pub length: u32,
    pub repetitions: u32,
}

fn main() {
    let input = get_input_string().unwrap_or(String::new());

    let mut final_string = String::new();

    let mut current_string = input;
    loop {

        let (first, second) = find_next_marker(current_string.as_str());

        if let (Some(first), Some(second)) = (first, second) {

            let marker = parse_marker(current_string.as_str(), first, second);

            {
                let (string_until_marker, _) = current_string.split_at(first);
                final_string.push_str(string_until_marker);
            }

            let start = second + 1 as usize;
            let end = start + (marker.length) as usize;

            for _ in 0..marker.repetitions {
                final_string.push_str(get_sub_string(current_string.as_str(), start, end));
            }

            let copy = current_string.clone();
            let (_, next) = copy.split_at(end);
            current_string = next.to_string();
        } else {
            break;
        }
    }

    let count = final_string.chars()
        .fold(0, |acc, x| {
            if char::is_whitespace(x) {
                return acc;
            }
            acc + 1
        });

    println!("{}", count);
}

fn find_next_marker(string: &str) -> (Option<usize>, Option<usize>) {
    (string.find("("), string.find(")"))
}

fn parse_marker(string: &str, first: usize, second: usize) -> Marker {
    let (temp, _) = string.split_at(second);
    let (_, temp) = temp.split_at(first + 1);

    let numbers: Vec<u32> = temp.split('x')
        .map(|x| {
            x.parse().unwrap_or(1u32)
        }).collect();

    Marker {
        length: numbers[0],
        repetitions: numbers[1],
    }
}

fn get_sub_string(string: &str, start: usize, end: usize) -> &str {
    let (temp, _) = string.split_at(end);
    let (_, sub_string) = temp.split_at(start);
    sub_string
}

fn get_input_string() -> Result<String, std::io::Error> {
    let mut input_file = try!(File::open("../input/day-9.txt"));
    let mut input_string = String::new();

    try!(input_file.read_to_string(&mut input_string));

    Ok(input_string)
}
