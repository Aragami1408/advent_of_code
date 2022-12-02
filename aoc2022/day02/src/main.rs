use std::{fs::File, io, io::BufRead, path::Path, hash::Hash, collections::HashMap};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>//{{{
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}//}}}

fn main() {
    let mut score1 = 0;
    let mut score2 = 0;

    let mut bonus_map = HashMap::new();
    bonus_map.insert('A' as i32, 1); // ROCK
    bonus_map.insert('B' as i32, 2); // PAPER
    bonus_map.insert('C' as i32, 3); // SCISSOR
                                     //
    if let Ok(lines) = read_lines("input1.txt") {{{{    
        for line in lines {
            if let Ok(ip) = line {
                let mut split = ip.split(' ');
                let (opponent, player) = (split.next().unwrap().parse::<char>().unwrap(), split.next().unwrap().parse::<char>().unwrap());
                let opponent_ascii = opponent as i32;
                let player_ascii = player as i32;
                let diff = player_ascii - opponent_ascii;
                if opponent_ascii == player_ascii {
                    score1 += 3;
                }
                else if diff == 1 || diff == -2 {
                    score1 += 6;
                }

                score1 += *bonus_map.get(&player_ascii).unwrap();
            }
        }
    }}}}

    if let Ok(lines) = read_lines("input2.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let mut split = ip.split(' ');
                let (opponent, player) = (split.next().unwrap().parse::<char>().unwrap(), split.next().unwrap().parse::<char>().unwrap());
                let opponent_ascii = opponent as i32;

                if player == 'X' {
                    let mut temp = opponent_ascii - 1;
                    if temp < 65 {
                        temp = 67;
                    }

                    score2 += bonus_map.get(&temp).unwrap();
                }
                else if player == 'Y' {
                    score2 += 3;
                    score2 += *bonus_map.get(&opponent_ascii).unwrap();
                }
                else if player == 'Z' {
                    score2 += 6;
                    let mut temp = opponent_ascii + 1;
                    if temp > 67 {
                        temp = 65;
                    }

                    score2 += bonus_map.get(&temp).unwrap();
                }
            }
        }
    }

    println!("{}", score1);
    println!("{}", score2);
}

