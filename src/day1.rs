use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn read_line_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .filter(|item| item.is_ok())
        .map(|item| item.unwrap())
        .map(|item| item.parse::<i32>())
        .filter(|item| item.is_ok())
        .map(|item| item.unwrap())
        .collect()
}

pub fn day1_part1() {
    let file = read_line_from_file("./src/day1.txt");
    let mut first_number = &file[0];
    let mut count = 0;
    for line in file.iter().skip(1) {
        if line > first_number {
            count += 1;
        }
        first_number = line;
    }

    println!("Day 1 part 1 answer: {}", count);
}

pub fn day1_part2() {
    let file = read_line_from_file("./src/day1.txt");

    let mut count = 0;
    for (index, line) in file.iter().enumerate() {
        if index + 3 > file.len() - 1 {
            break;
        }

        if line < &file[index + 3] {
            count += 1;
        }
    }

    println!("Day 1 part 2 answer: {}", count);
}
