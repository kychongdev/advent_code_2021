use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_line_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
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
