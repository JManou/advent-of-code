use std::collections::HashMap;

fn main() {
    let mut result_part_1: u32 = 0;
    let mut result_part_2: u32 = 0;
    let mut raw_line = String::new();

    println!("start");
    loop {
        raw_line.clear();
        let _ = std::io::stdin().read_line(&mut raw_line);
        let line = raw_line.trim();
        if line.len() == 0 {
            println!("part1: {}", result_part_1);
            println!("part2: {}", result_part_2);
            return;
        }

        result_part_1 += parse_game(line);
        result_part_2 += min_cubes(line);
    }
}

fn parse_game(game: &str) -> u32 {
    let map = HashMap::<&'static str, u32>::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut split_game = game.split(':');
    let game_id =
        u32::from_str_radix(split_game.next().unwrap().split(' ').last().unwrap(), 10).unwrap();
    for round in split_game.last().unwrap().trim().split("; ") {
        for rgb in round.split(", ") {
            let mut rgb_split = rgb.split(' ');
            let value = u32::from_str_radix(rgb_split.next().unwrap(), 10).unwrap();

            if value > map[rgb_split.next().unwrap()] {
                return 0;
            }
        }
    }

    return game_id;
}

fn min_cubes(game: &str) -> u32 {
    let mut map = HashMap::<&'static str, u32>::from([("red", 0), ("green", 0), ("blue", 0)]);
    let split_game = game.split(':');
    for round in split_game.last().unwrap().trim().split("; ") {
        for rgb in round.split(", ") {
            let mut rgb_split = rgb.split(' ');
            let value = u32::from_str_radix(rgb_split.next().unwrap(), 10).unwrap();
            let key = rgb_split.next().unwrap();
            if value > map[key] {
                *map.entry(key).or_insert(0) = value;
            }
        }
    }

    let mut result = 1;

    for value in map.values() {
        result *= *value;
    }

    return result;
}
