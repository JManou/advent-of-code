use aho_corasick::AhoCorasick;
use std::collections::HashMap;

fn main() {
    let mut result_part_1: u32 = 0;
    let mut result_part_2: u32 = 0;
    let mut raw_line = String::new();

    println!("start");
    let digit_finder = DigitFinder::new();
    loop {
        raw_line.clear();
        let _ = std::io::stdin().read_line(&mut raw_line);
        let line = raw_line.trim();
        if line.len() == 0 {
            println!("part1: {}", result_part_1);
            println!("part2: {}", result_part_2);
            return;
        }

        result_part_1 += part1(line);
        result_part_2 += digit_finder.find_last_two_digits(line);
    }
}

fn part1(line: &str) -> u32 {
    let mut line_values = Vec::<char>::with_capacity(2);

    for _char in line.chars() {
        if !_char.is_ascii_digit() {
            continue;
        }

        if line_values.len() == 2 {
            line_values.remove(1);
        }

        line_values.push(_char)
    }

    if line_values.len() < 2 && line_values.len() > 0 {
        line_values.push(line_values[0])
    }

    return u32::from_str_radix(String::from_iter(line_values.iter()).as_str(), 10).unwrap();
}

struct DigitFinder {
    regex: AhoCorasick,
    value_mapper: HashMap<&'static str, char>,
    pattern_array: Vec<&'static str>,
}

impl DigitFinder {
    fn new() -> DigitFinder {
        DigitFinder {
            regex: AhoCorasick::new([
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
                "six", "seven", "eight", "nine",
            ])
            .unwrap(),
            value_mapper: HashMap::<&str, char>::from([
                ("one", '1'),
                ("two", '2'),
                ("three", '3'),
                ("four", '4'),
                ("five", '5'),
                ("six", '6'),
                ("seven", '7'),
                ("eight", '8'),
                ("nine", '9'),
            ]),
            pattern_array: [
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
                "six", "seven", "eight", "nine",
            ]
            .to_vec(),
        }
    }

    fn find_last_two_digits(&self, line: &str) -> u32 {
        let mut str_number = String::new();
        let mut matches = self.regex.find_overlapping_iter(line);
        let first = self
            .pattern_array
            .get(matches.next().unwrap().pattern().as_usize())
            .unwrap();
        let second = match matches.last() {
            Some(m) => self.pattern_array.get(m.pattern().as_usize()).unwrap(),
            None => first,
        };

        str_number.push(self.get_digit_char(first));
        str_number.push(self.get_digit_char(second));

        return u32::from_str_radix(str_number.as_str(), 10).unwrap();
    }

    fn get_digit_char(&self, match_str: &str) -> char {
        if self.value_mapper.contains_key(match_str) {
            return self.value_mapper[match_str];
        }

        return match_str.chars().next().unwrap();
    }
}
