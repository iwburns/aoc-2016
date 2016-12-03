use std::io::prelude::*;
use std::fs::File;

fn main() {
    let input = get_input_string().unwrap_or(String::new());

    let triangles: Vec<&str> = input.lines()
        .map(|x| x.trim())
        .collect();

    let mut num_real_triangles = 0;

    for tri in triangles {

        let numbers: Vec<u32> = tri.split_whitespace()
            .map(|x| x.parse().expect("couldn't parse u32 from &str"))
            .collect();

        if numbers.len() == 3 {
            if is_valid_triangle(numbers[0], numbers[1], numbers[2]) {
                num_real_triangles += 1;
            }
        } else {
            unreachable!();
        }
    }

    println!("Real Triangles: {}", num_real_triangles);
}

fn is_valid_triangle(a: u32, b: u32, c: u32) -> bool {
    (a + b > c) && (b + c > a) && (c + a > b)
}

fn get_input_string() -> Result<String, std::io::Error> {
    let mut input_file = File::open("../input/day-3.txt")?;
    let mut input_string = String::new();

    input_file.read_to_string(&mut input_string)?;

    Ok(input_string)
}
