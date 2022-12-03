use std::{fs::File, io, io::BufRead, path::Path, collections::HashSet};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_to_vec(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = io::BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}

fn char_intersection(a: &str, b: &str) -> char {
    let a_vec: HashSet<char> = a.chars().collect();
    let b_vec: HashSet<char> = b.chars().collect();
    
    let intersection = a_vec.intersection(&b_vec).collect::<Vec<_>>();

    *intersection[0]
}


fn main() {
    let mut sum = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let first_half = &ip[..ip.len()/2];
                let last_half = &ip[ip.len()/2..];
                
                let c = char_intersection(first_half, last_half);         
                let priority = if c.is_lowercase() {(c as i32) - 96} else {(c as i32) - 38};
                sum += priority;
            }
        }
    }
    sum = 0;
    println!("{}", sum);

    let lines = read_to_vec("input.txt");
    for i in (0..lines.len()).step_by(3) {
        let a = lines[i].to_owned();
        let b = lines[i+1].to_owned();
        let c = lines[i+2].to_owned();
    }




}
