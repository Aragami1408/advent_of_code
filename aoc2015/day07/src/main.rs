use std::{fs::File, io, io::BufRead, path::Path, collections::HashMap};
use fancy_regex::Regex;


enum Operation {
    Assign { // ^(\w+) -> (\w+)
        lhs: String,
        rhs: String
    },
    Unary { // ^NOT (\w+) -> (\w+)
        src: String,
        dst: String
    },
    Binary { // ^(\w+) (\w+) (\w+) -> (\w+)
        src1: Token,
        src2: Token,
        op: String,
        dst: String
    },
    Nop
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

macro_rules! capture_group {
    ($parser:ident,$text:ident,$index:expr) => {
        $parser.captures($text).expect("").expect("").get($index).expect("").as_str().to_string()
    };
}

fn decode(instruction: &str) -> Operation {
    let assign_parser = Regex::new(r"^(\w+) -> (\w+)").unwrap();
    let unary_parser = Regex::new(r"^NOT (\w+) -> (\w+)").unwrap();
    let binary_parser = Regex::new(r"^(\w+) (\w+) (\w+) -> (\w+)").unwrap();
    
    
    if assign_parser.is_match(instruction).is_ok() {
        let lhs = capture_group!(assign_parser, instruction, 1);
        let rhs = capture_group!(assign_parser, instruction, 2);

        Operation::Assign {lhs, rhs}
    }
    else if unary_parser.is_match(instruction).is_ok() {
        let src = capture_group!(unary_parser, instruction, 1);
        let dst = capture_group!(unary_parser, instruction, 2);

        Operation::Unary {src, dst}

    }
    else if binary_parser.is_match(instruction).is_ok() {
        let src1 = capture_group!(binary_parser, instruction, 1);
        let op = capture_group!(binary_parser, instruction, 2);
        let src2 = capture_group!(binary_parser, instruction, 3);
        let dst = capture_group!(binary_parser, instruction, 4);

        Operation::Binary {src1, src2, op, dst}

    }
    else {
        Operation::Nop
    }

}


fn main() {

    if let Ok(lines) = read_lines("input/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
            }
        }
    }
}
