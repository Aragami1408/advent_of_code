use std::{fs::File, io, io::BufRead, path::Path};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



fn main() {
    let mut calories_vec: Vec<u32> = vec![];
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut temp: u32 = 0;
        for line in lines {

            if let Ok(ip) = line {
                if let Ok(cal) = ip.parse::<u32>() {
                    temp += cal;
                }
                else {
                    calories_vec.push(temp);
                    temp = 0;
                }
            }
        }
    }

    println!("{}", calories_vec.iter().max().unwrap());
    calories_vec.sort_by(|a,b|b.cmp(a));
    println!("{:?}", calories_vec); 
}
