use std::{collections::HashMap, fs};

fn update_house(x: i32, y: i32, house_map: &mut HashMap<(i32, i32), i32>) {
    if !house_map.contains_key(&(x, y)) {
        house_map.insert((x, y), 1);
    } else {
        if let Some(a) = house_map.get_mut(&(x, y)) {
            *a += 1;
        }
    }
}

fn update_coord(x: &mut i32, y: &mut i32, direction: char) {
    match direction {
        '>' => *x += 1,
        '<' => *x -= 1,
        '^' => *y += 1,
        'v' => *y -= 1,
        _ => println!("character not supported."),
    }
}

fn part1(directions: &String) -> i32 {
    let mut house_map: HashMap<(i32, i32), i32> = HashMap::new();
    let (mut x, mut y) = (0, 0);

    house_map.insert((x, y), 1);

    for (_, direction) in directions.chars().enumerate() {
        update_coord(&mut x, &mut y, direction);
        update_house(x, y, &mut house_map);
    }

    house_map.len().try_into().unwrap()
}

fn part2(directions: &String) -> u32 {
    let mut house_map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut santa_x = 0;
    let mut santa_y = 0;
    let mut robo_x = 0;
    let mut robo_y = 0;
    let mut is_santa_turn = true;

    update_house(santa_x, santa_y, &mut house_map);
    update_house(robo_x, robo_y, &mut house_map);
    for (_, direction) in directions.chars().enumerate() {
        if is_santa_turn {
            update_coord(&mut santa_x, &mut santa_y, direction);
            update_house(santa_x, santa_y, &mut house_map);
            is_santa_turn = false;
        } else {
            update_coord(&mut robo_x, &mut robo_y, direction);
            update_house(robo_x, robo_y, &mut house_map);
            is_santa_turn = true;
        }
    }
    house_map.len().try_into().unwrap()
}

fn main() {
    let input = fs::read_to_string("input/part1.txt").expect("Unable to read file");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
