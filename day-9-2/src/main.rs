use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
struct Marker {
    pub length: u64,
    pub repetitions: u64,
}

fn main() {
    let input = get_input_string().unwrap_or(String::new());

//    let input = "(10x2)(3x3)abcde".to_string();
//    let target_count = "abcabcabcdeabcabcabcde".len();

//    let count = calc_num_characters(input.clone());

//    println!("target_count: {}", target_count);
//    println!("count: {}", count);

    let mut current_string = input;
    let mut sum: u64 = 0;
    loop {
        if let (Some(start), Some(end)) = find_next_marker(current_string.as_str()) {

            let marker = parse_marker(current_string.as_str(), start, end);
            let territory_end = end + 1 + marker.length as usize;

            let full_marker_string = get_sub_string(current_string.as_str(), start, territory_end).to_string();

            sum += calc_num_characters(full_marker_string);
            println!("sum: {}", sum);
            if territory_end == current_string.len() {
                break;
            }

            let copy = current_string.clone();
            let (_, next) = copy.split_at(territory_end);

            current_string = next.to_string();
        } else {
            break;
        }
    }

    println!("count: {}", sum);
}

fn calc_num_characters(string: String) -> u64 {
    if let (Some(start), Some(end)) = find_next_marker(string.as_str()) {
        let marker = parse_marker(string.as_str(), start, end);
//        print!("{:?}", marker);
        let marker_territory = get_sub_string(string.as_str(), 1 + end as usize, string.len());

//        println!("territory: {}", marker_territory);

        if marker_territory.contains('(') {
//            println!("found more");
            let num_characters: u64 = calc_num_characters(marker_territory.to_string());
            let marker_repetitions: u64 = marker.repetitions;
//            println!("characters: {}", num_characters);
//            println!("reps: {}", marker_repetitions);
            return calc_num_characters(marker_territory.to_string()) * marker.repetitions;
        } else {
//            println!("no more");
            let num_leftover_chars = (marker_territory.len() as u64 - marker.length) as u64;
            return (marker.length * marker.repetitions) + num_leftover_chars;
        }
    }
    return 0u64;
}

fn find_next_marker(string: &str) -> (Option<usize>, Option<usize>) {
    (string.find("("), string.find(")"))
}

fn parse_marker(string: &str, first: usize, second: usize) -> Marker {
    let (temp, _) = string.split_at(second);
    let (_, temp) = temp.split_at(first + 1);

    let numbers: Vec<u64> = temp.split('x')
        .map(|x| {
            x.parse().unwrap_or(1u64)
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
