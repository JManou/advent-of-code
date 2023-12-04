use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut result_part_1: u32 = 0;
    let mut result_part_2: u32 = 0;
    let mut raw_line = String::new();
    let mut recordings = HashMap::<u32, u32>::new();

    println!("start");
    loop {
        raw_line.clear();
        let _ = std::io::stdin().read_line(&mut raw_line);
        let line = raw_line.trim();
        if line.len() == 0 {
            break;
        }

        result_part_1 += parse_game(line);
        part2(line, &mut recordings)
    }

    println!("part1: {}", result_part_1);
    println!("part2: {}", recordings.values().sum::<u32>());
}

fn parse_game(game: &str) -> u32 {
    let mut split_game = game.split(": ");
    let game_id =
        u32::from_str_radix(split_game.next().unwrap().split(' ').last().unwrap(), 10).unwrap();
    let mut card_deck = split_game.last().unwrap().trim().split("| ");
    let hash_set = HashSet::<u32>::from_iter(
        card_deck
            .next()
            .unwrap()
            .split(' ')
            .filter(|c| *c != "")
            .map(|v| u32::from_str_radix(v, 10).unwrap()),
    );

    let mut winner = 0;
    for round in card_deck.next().unwrap().split(' ') {
        let value = match u32::from_str_radix(round, 10) {
            Ok(s) => s,
            Err(_) => continue,
        };

        if hash_set.contains(&value) {
            winner += 1;
        }
    }

    if winner > 0 {
        return u32::pow(2, winner - 1);
    }

    return 0;
}

fn part2(game: &str, recordings: &mut HashMap<u32, u32>) {
    let mut split_game = game.split(": ");
    let game_id =
        u32::from_str_radix(split_game.next().unwrap().split(' ').last().unwrap(), 10).unwrap();
    let mut card_deck = split_game.last().unwrap().trim().split("| ");
    let hash_set = HashSet::<u32>::from_iter(
        card_deck
            .next()
            .unwrap()
            .split(' ')
            .filter(|c| *c != "")
            .map(|v| u32::from_str_radix(v, 10).unwrap()),
    );

    let mut winner = 0;
    for round in card_deck.next().unwrap().split(' ') {
        let value = match u32::from_str_radix(round, 10) {
            Ok(s) => s,
            Err(_) => continue,
        };

        if hash_set.contains(&value) {
            winner += 1;
        }
    }

    *recordings.entry(game_id).or_insert(0) += 1;

    for i in 0..winner {
        *recordings.entry(game_id + 1 + i).or_insert(0) += *recordings.entry(game_id).or_insert(0);
    }
}
