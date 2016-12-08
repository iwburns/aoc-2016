use std::io::prelude::*;
use std::fs::File;

struct Display {
    pixels: Vec<Vec<u8>>,
}

impl Display {
    fn new(width: usize, height: usize) -> Display {
        let mut vec = Vec::with_capacity(height);

        for _ in 0..height {
            let mut temp = Vec::with_capacity(width);
            for _ in 0..width {
                temp.push(0);
            }
            vec.push(temp);
        }

        Display {
            pixels: vec
        }
    }

    fn rect(&mut self, width: usize, height: usize) {
        for i in 0..height {
            for j in 0..width {
                self.pixels[i][j] = 1;
            }
        }
    }

    fn rotate_row(&mut self, row_index: usize, amt: i32) {
        let mut row_copy = self.pixels[row_index].clone();

        let row_len = row_copy.len() as i32;

        for i in 0..self.pixels[row_index].len() {
            let offset: usize = (((i as i32) + amt) % row_len) as usize;
            row_copy[offset] = self.pixels[row_index][i];
        }

        self.pixels[row_index] = row_copy;
    }

    fn rotate_col(&mut self, col_index: usize, amt: i32) {
        let mut col_copy = Vec::with_capacity(self.pixels.len());
        for i in 0..self.pixels.len() {
            col_copy.push(self.pixels[i][col_index]);
        }

        let col_len = col_copy.len() as i32;

        for i in 0..self.pixels.len() {
            let offset: usize = (((i as i32) + amt) % col_len) as usize;
            col_copy[offset] = self.pixels[i][col_index];
        }

        for i in 0..self.pixels.len() {
            self.pixels[i][col_index] = col_copy[i];
        }
    }

    fn print(&self) {
        for i in 0..self.pixels.len() {
            print!("[");
            for j in 0..self.pixels[i].len() {
                print!("{}", self.pixels[i][j]);
            }
            println!("]");
        }
    }

    fn count_lit_pixels(&self) -> i32 {
        let mut count = 0;

        for i in 0..self.pixels.len() {
            for j in 0..self.pixels[i].len() {
                count += self.pixels[i][j] as i32;
            }
        }

        count
    }
}

enum Command {
    Rect(usize, usize),
    RotateRow(usize, i32),
    RotateCol(usize, i32),
}

fn main() {
    let input = get_input_string().unwrap_or(String::new());

    let mut display = Display::new(50, 6);

    for line in input.lines() {
        match parse_command(line) {
            Command::Rect(width, height) => display.rect(width, height),
            Command::RotateRow(index, amount) => display.rotate_row(index, amount),
            Command::RotateCol(index, amount) => display.rotate_col(index, amount),
        }
    }

    println!("{}", display.count_lit_pixels());

}

fn parse_command(string: &str) -> Command {
    let parts: Vec<&str> = string.split_whitespace().collect();

    if parts[0] == "rect" {
        let amounts: Vec<&str> = parts[1].split('x').collect();
        let amt_1: u32 = amounts[0].parse().unwrap_or(0u32);
        let amt_2: u32 = amounts[1].parse().unwrap_or(0u32);

        return Command::Rect(amt_1 as usize, amt_2 as usize)
    }

    let index = parts[2].split('=')
        .collect::<Vec<&str>>()
        .last()
        .unwrap_or(&"0")
        .parse::<usize>()
        .unwrap_or(0usize);

    let amount = parts[4].parse::<i32>().unwrap_or(0i32);

    match parts[1] {
        "row" => Command::RotateRow(index, amount),
        "column" => Command::RotateCol(index, amount),
        _ => { unreachable!(); }
    }
}

fn get_input_string() -> Result<String, std::io::Error> {
    let mut input_file = File::open("../input/day-8.txt")?;
    let mut input_string = String::new();

    input_file.read_to_string(&mut input_string)?;

    Ok(input_string)
}
