use std::{fs::File, io, io::BufRead, path::Path, collections::HashSet};
use fancy_regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse(pair: &str) -> (i32,i32,i32,i32) {
    let caps = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap().captures(pair).expect("Error running regex").expect("No match found");
    let x1 = caps.get(1).expect("can't capture").as_str().parse::<i32>();
    let x2 = caps.get(2).expect("can't capture").as_str().parse::<i32>();
    let y1 = caps.get(3).expect("can't capture").as_str().parse::<i32>();
    let y2 = caps.get(4).expect("can't capture").as_str().parse::<i32>();

    (x1.unwrap(),x2.unwrap(),y1.unwrap(),y2.unwrap())
}

#[test]
fn test_parse() {
    assert_eq!(parse("16-80,80-87"), (16,80,80,87));
}



fn main() {
    let mut part1 = 0;
    let mut part2 = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let (x1,x2,y1,y2) = parse(&ip.to_string());
                if ((x1 <= y1) && (x2 >= y2)) || ((x1 >= y1) && (x2 <= y2)) {part1 += 1;part2 += 1;}
                else if (y1 <= x2 && x2 <= y2) || (y1 <= x1 && x1 <= y2) {part2 += 1;} 
            }
        }
    }

    println!("{}", part1);
    println!("{}", part2);
}
