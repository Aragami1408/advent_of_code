use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part_1(l: u32, w: u32, h: u32) -> u32 {
    let lw = l * w;
    let lh = l * h;
    let wh = w * h;

    (2 * lw + 2 * lh + 2 * wh + cmp::min(lw, cmp::min(lh, wh))) as u32
}

fn part_2(l: u32, w: u32, h: u32) -> u32 {
    let lw = (l + w) * 2;
    let lh = (l + h) * 2;
    let wh = (w + h) * 2;

    cmp::min(lw, cmp::min(lh, wh)) + l * w * h
}

fn main() {
    let mut result1 = 0;
    let mut result2 = 0;
    if let Ok(lines) = read_lines("input/part1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let dims = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
                for caps in dims.captures_iter(&ip) {
                    let l: u32 = (caps.get(1).unwrap().as_str()).parse::<u32>().unwrap();
                    let w: u32 = (caps.get(2).unwrap().as_str()).parse::<u32>().unwrap();
                    let h: u32 = (caps.get(3).unwrap().as_str()).parse::<u32>().unwrap();

                    result1 += part_1(l, w, h);
                    result2 += part_2(l, w, h);
                }
            }
        }
    }

    println!("{} {}", result1, result2);
}
