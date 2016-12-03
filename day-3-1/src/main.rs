use std::io::prelude::*;
use std::fs::File;

fn main() {
    let input = get_input_string().unwrap_or(String::new());

    let triangles: Vec<&str> = input.lines()
        .map(|x| x.trim())
        .collect();

    let mut num_real_triangles = 0;

    for tri in triangles {

        let mut numbers: Vec<u32> = tri.split_whitespace()
            .map(|x| x.parse().expect("couldn't parse u32 from &str"))
            .collect();

        let smallest = numbers.iter()
            .min()
            .unwrap()
            .clone();

        let largest = numbers.iter()
            .max()
            .unwrap()
            .clone();

        let smallest_index = numbers.iter()
            .position(|&x| {
                x == smallest
            }).unwrap();

        let largest_index = numbers.iter()
            .position(|&x| {
                x == largest
            }).unwrap();

        if smallest_index > largest_index {
            numbers.remove(smallest_index);
            numbers.remove(largest_index);
        } else {
            numbers.remove(largest_index);
            numbers.remove(smallest_index);
        }

        let middle = numbers.remove(0);

        if smallest + middle > largest {
            num_real_triangles += 1;
        }
    }

    println!("Real Triangles: {}", num_real_triangles);
}

fn get_input_string() -> Result<String, std::io::Error> {
    let mut input_file = File::open("../input/day-3.txt")?;
    let mut input_string = String::new();

    input_file.read_to_string(&mut input_string)?;

    Ok(input_string)
}
