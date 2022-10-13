use std::fs;

fn part1(input: String) -> i32 {
    let mut floor = 0;

    for (_, bracket) in input.chars().enumerate() {
        floor += if bracket == '(' { 1 } else { -1 };
    }

    floor
}

fn part2(input: String) -> u32 {
    let mut floor = 0;
    let mut pos: u32 = 0;

    for (_, bracket) in input.chars().enumerate() {
        floor += if bracket == '(' { 1 } else { -1 };
        pos += 1;
        if floor == -1 {
            return pos;
        }
    }

    0
}

fn main() {
    let input_1 = fs::read_to_string("input/part1.txt").expect("Unable to read file");
    println!("{}", part1(input_1));
    let input_2 = fs::read_to_string("input/part2.txt").expect("Unable to read file");
    println!("{}", part2(input_2));
}
