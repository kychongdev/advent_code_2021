mod part1;

use crate::part1::read_line_from_file;

fn main() {
    let file = read_line_from_file("./src/part1.txt");
    let mut first_number = &file[0];
    let mut count = 0;
    for line in file.iter().skip(1) {
        if line > first_number {
            count += 1;
        }
        first_number = line;
    }

    println!("{}", count);
}
