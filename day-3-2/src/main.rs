use std::io::prelude::*;
use std::fs::File;

fn main() {
    let input = get_input_string().unwrap_or(String::new());

    let triples: Vec<&str> = input.lines()
        .map(|x| x.trim())
        .collect();

    let mut num_real_triangles = 0;

    let mut i = 0;
    loop {
        if i >= triples.len() {
            break;
        }

        let row_1_nums: Vec<u32> = triples.get(i)
            .expect("couldn't get() on triples at index offset 0")
            .split_whitespace()
            .map(|x| x.parse().expect("couldn't parse u32 from &str for row_1_nums"))
            .collect();

        let row_2_nums: Vec<u32> = triples.get(i + 1)
            .expect("couldn't get() on triples at index offset 1")
            .split_whitespace()
            .map(|x| x.parse().expect("couldn't parse u32 from &str for row_2_nums"))
            .collect();

        let row_3_nums: Vec<u32> = triples.get(i + 2)
            .expect("couldn't get() on triples at index offset 2")
            .split_whitespace()
            .map(|x| x.parse().expect("couldn't parse u32 from &str for row_3_nums"))
            .collect();

        for j in 0..3 {
            let mut triangle = Vec::new();

            triangle.push(row_1_nums.get(j)
                .expect("couldn't get() from row_1_nums")
                .clone());
            triangle.push(row_2_nums.get(j)
                .expect("couldn't get() from row_2_nums")
                .clone());
            triangle.push(row_3_nums.get(j)
                .expect("couldn't get() from row_3_nums")
                .clone());

            let smallest = triangle.iter()
                .min()
                .expect("couldn't find min value in triangle")
                .clone();

            let largest = triangle.iter()
                .max()
                .expect("couldn't find max value in triangle")
                .clone();

            let smallest_index = triangle.iter()
                .position(|&x| {
                    x == smallest
                }).expect("couldn't find index of smallest in triangle");

            let largest_index = triangle.iter()
                .position(|&x| {
                    x == largest
                }).expect("couldn't find index of largest in triangle");

            if smallest_index > largest_index {
                triangle.remove(smallest_index);
                triangle.remove(largest_index);
            } else {
                triangle.remove(largest_index);
                triangle.remove(smallest_index);
            }

            let middle = triangle.remove(0);
            if smallest + middle > largest {
                num_real_triangles += 1;
            }
        }

        i += 3;
    }
    println!("Real Triangles: {}", num_real_triangles);
}

fn get_input_string() -> Result<String, std::io::Error> {
    let mut input_file = File::open("../input/day-3.txt")?;
    let mut input_string = String::new();

    input_file.read_to_string(&mut input_string)?;

    Ok(input_string)
}
