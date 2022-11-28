use fancy_regex::Regex;
use std::{fs::File, io, io::BufRead, path::Path, collections::HashMap};

type Command<'a> = (&'a str, (u32, u32), (u32, u32)); //No. I'm too lazy for struct syntax

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn command_parser(command: &str) -> Command {

    let caps = Regex::new(r"(\w+) (\d+),(\d+) through (\d+),(\d+)")
        .unwrap()
        .captures(command)
        .expect("error running regex")
        .expect("no match found");

    let action = caps.get(1).expect("can't parse action").as_str();
    let x1 = caps
        .get(2)
        .expect("can't parse first x position")
        .as_str()
        .parse::<u32>()
        .unwrap();
    let y1 = caps
        .get(3)
        .expect("can't parse first y position")
        .as_str()
        .parse::<u32>()
        .unwrap();
    let x2 = caps
        .get(4)
        .expect("can't parse second x position")
        .as_str()
        .parse::<u32>()
        .unwrap();
    let y2 = caps
        .get(5)
        .expect("can't parse second y position")
        .as_str()
        .parse::<u32>()
        .unwrap();

    (action, (x1, y1), (x2, y2))
}

fn part1(command: Command, map: &mut HashMap<(u32,u32),u32>) {
    
    let action = command.0;
    let x1  = (command.1).0;
    let y1 = (command.1).1;
    let x2 = (command.2).0;
    let y2 = (command.2).1;

    for x in x1..(x2 + 1) {
        for y in y1..(y2 + 1) {
            match action {
                "on" => {
                    map.insert((x,y),1);
                }
                "off" => {
                    map.remove(&(x,y));
                }
                "toggle" => {
                    if map.contains_key(&(x,y)) {
                       map.remove(&(x,y));
                    }
                    else {
                        map.insert((x,y), 1);
                    }
                }
                _ => {}
            }
        }
    }

}

fn part2(command: Command, map: &mut HashMap<(u32,u32),u32>) {
    let action = command.0;
    let x1  = (command.1).0;
    let y1 = (command.1).1;
    let x2 = (command.2).0;
    let y2 = (command.2).1;

    for x in x1..(x2 + 1) {
        for y in y1..(y2 + 1) {
            match action {
                "on" => {
                    if let Some(brightness) = map.get_mut(&(x,y)) {
                        *brightness += 1;
                    }
                    else {
                        map.insert((x,y),1);
                    }
                }
                "off" => {
                    if let Some(brightness) = map.get_mut(&(x,y)) {
                        if *brightness > 0 {
                            *brightness -= 1;
                        }
                    }
                    else {
                        map.insert((x,y),0);
                    }
                }
                "toggle" => {
                    if let Some(brightness) = map.get_mut(&(x,y)) {
                        *brightness += 2;
                    }
                    else {
                        map.insert((x,y),2);
                    }
                }
                _ => {}
            }
        }
    }

}

fn main() {
    let mut result: u32 = 0;
    let mut litmap: HashMap<(u32,u32), u32> = HashMap::new();
    let mut brightmap: HashMap<(u32,u32), u32> = HashMap::new();
    if let Ok(lines) = read_lines("input/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let command = command_parser(ip.as_str());
                part1(command, &mut litmap);
                part2(command, &mut brightmap);
            }
        }
    }
    let mut values: Vec<u32> = litmap.into_values().collect();
    result = values.iter().sum();
    println!("{}", result);

    values = brightmap.into_values().collect();
    result = values.iter().sum();
    println!("{}", result);
}
