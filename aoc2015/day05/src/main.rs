use std::{fs::File, io, io::BufRead, path::Path};

use fancy_regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1(input: &str) -> bool {
    let first_condition = Regex::new(r"(?:[^aeiou\W]*[aeiuo]){3,}[^aeiou\W]*\b").unwrap();
    let second_condition = Regex::new(r"(.)\1+").unwrap();

    first_condition.is_match(input).unwrap()&& // at least 3 vowels
        second_condition.is_match(input).unwrap()&& // at least 1 letter appear twice in a row
        // no contains"ab" "cd" "pq" "xy"
        !(input.contains("ab")
            || input.contains("cd")
            || input.contains("pq")
            || input.contains("xy"))
}

fn part2(input: &str) -> bool {
    let first_condition = Regex::new(r"(.)(.).*\1\2").unwrap().is_match(input).unwrap();
    let mut second_condition: bool = false;
    for i in 0..(input.len()-2) {
        let a = input.to_string().as_bytes()[i as usize] as char;
        let b = input.to_string().as_bytes()[i+2 as usize] as char;
        if a == b {second_condition = true;}
    }

    first_condition && second_condition
}

fn main() {
    let mut nice_strings = 0;
    if let Ok(lines) = read_lines("input/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if part1(&ip) {
                    nice_strings += 1;
                }
            }
        }
    }
    println!("{}", nice_strings);
    nice_strings = 0;
    if let Ok(lines) = read_lines("input/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if part2(&ip) {
                    nice_strings += 1;
                }
            }
        }
    }
    println!("{}", nice_strings);

}
