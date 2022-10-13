use fancy_regex::Regex;
use std::{collections::HashMap, fs::File, io, io::BufRead, ops::Range, path::Path};

type Command<'a> = (&'a str, (u16, u16), (u16, u16)); //No. I'm too lazy for struct syntax

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_command(command: &str) -> Command {
    let caps = Regex::new(r"(\w+) (\d+),(\d+) through (\d+),(\d+)")
        .unwrap()
        .captures(command)
        .expect("Error running regex")
        .expect("No match found");

    let action = caps.get(0).expect("Can't parse action").as_str();
    let x1: u16 = caps
        .get(1)
        .expect("Can't parse first x position")
        .as_str()
        .parse()
        .unwrap();
    let y1: u16 = caps
        .get(2)
        .expect("Can't parse first y position")
        .as_str()
        .parse()
        .unwrap();
    let x2: u16 = caps
        .get(3)
        .expect("Can't parse second x position")
        .as_str()
        .parse()
        .unwrap();
    let y2: u16 = caps
        .get(4)
        .expect("Can't parse second y position")
        .as_str()
        .parse()
        .unwrap();

    (action, (x1, y1), (x2, y2))
}

#[test]
fn test_parse_command() {
    assert_eq!(
        parse_command("turn on 0,0 through 999,999"),
        ("on", (0, 0), (999, 999))
    );
}

fn part1(command: &str) -> usize {
    let mut litvec: Vec<(u16, u16)> = vec![];

    let caps = Regex::new(r"(\w+) (\d+),(\d+) through (\d+),(\d+)")
        .unwrap()
        .captures(command)
        .expect("Error running regex")
        .expect("No match found");

    let action = caps.get(0).expect("Can't parse action").as_str();
    let x1 = caps
        .get(1)
        .expect("Can't parse first x position")
        .as_str()
        .parse::<u16>()
        .unwrap();
    let y1 = caps
        .get(2)
        .expect("Can't parse first y position")
        .as_str()
        .parse::<u16>()
        .unwrap();
    let x2 = caps
        .get(3)
        .expect("Can't parse second x position")
        .as_str()
        .parse::<u16>()
        .unwrap();
    let y2 = caps
        .get(4)
        .expect("Can't parse second y position")
        .as_str()
        .parse::<u16>()
        .unwrap();

    for x in x1..(x2 + 1) {
        for y in y1..(y2 + 1) {
            let index = litvec.iter().position(|a| *a == (x, y)).unwrap();
            match action {
                "on" => {}
                "off" => {}
                "toggle" => {}
                _ => {}
            }
        }
    }

    litvec.len()
}

fn main() {
    println!("Hello, world!");

    if let Ok(lines) = read_lines("input/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {}
        }
    }
}
