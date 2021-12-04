mod day1;

use crate::day1::read_line_from_file;

fn main() {
    // part1_solution();
    day1_part2();
}

fn day1_part1() {
    let file = read_line_from_file("./src/day1.txt");
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

fn day1_part2() {
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

    println!("{}", count);
}
