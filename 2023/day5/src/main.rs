use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

struct Range {
    source_start: isize,
    source_end: isize,
    margin: isize,
}

impl Range {
    fn get_destination(&self, source: isize) -> isize {
        return source + self.margin;
    }
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        match self.cmp(&other) {
            Ordering::Equal => true,
            _ => false,
        }
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Eq for Range {}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        // Custom comparer, when searching for a range specify the value for which
        // you are searching a range twice
        if other.source_start == other.source_end
            && other.source_start >= self.source_start
            && other.source_start < self.source_end
        {
            return Ordering::Equal;
        }

        if self.source_start == self.source_end
            && self.source_start >= other.source_start
            && self.source_start < other.source_end
        {
            return Ordering::Equal;
        }

        // We are sure that they wont be any collision, union of ranges are empty
        return self.source_start.cmp(&other.source_start);
    }
}

fn main() {
    //println!("part1: {}", part1());
    println!("part2: {}", part2());
}

fn get_correspondence(value: isize, mapping: &Vec<Range>) -> isize {
    let search_value = Range {
        source_start: value,
        source_end: value,
        margin: 0,
    };

    return match mapping.binary_search(&search_value) {
        Ok(index) => mapping.get(index).unwrap().margin + value,
        Err(_) => value,
    };
}

fn read_seeds() -> Vec<isize> {
    println!("start reading seeds");

    let mut raw_line = String::new();
    let _ = std::io::stdin().read_line(&mut raw_line);
    let mut discard = String::new();
    let _ = std::io::stdin().read_line(&mut discard);

    return raw_line
        .trim()
        .split(": ")
        .last()
        .unwrap()
        .split(' ')
        .map(|d| d.parse::<isize>().unwrap())
        .collect();
}

fn read_mapping() -> Vec<Range> {
    println!("start reading mapping");

    let mut discard = String::new();
    let _ = std::io::stdin().read_line(&mut discard);

    println!("reading {}", &discard);
    let mut raw_line = String::new();

    let mut mapping = Vec::<Range>::new();

    loop {
        raw_line.clear();
        let _ = std::io::stdin().read_line(&mut raw_line);
        let line = raw_line.trim();
        if line.len() == 0 {
            break;
        }
        let mut sp = line.split(' ');
        let target = sp.next().unwrap().parse::<isize>().unwrap();
        let source = sp.next().unwrap().parse::<isize>().unwrap();
        let range = sp.next().unwrap().parse::<isize>().unwrap();

        let rg = Range {
            source_start: source,
            source_end: source + range,
            margin: target - source,
        };

        mapping.push(rg);
    }

    mapping.sort();
    return mapping;
}

fn part1() -> isize {
    let seeds = read_seeds();
    let seed_to_soil = read_mapping();
    let soil_to_fer = read_mapping();
    let fer_to_water = read_mapping();
    let water_to_light = read_mapping();
    let light_to_temp = read_mapping();
    let temp_to_hum = read_mapping();
    let hum_to_loc = read_mapping();

    let mut min = isize::max_value();

    for val in seeds {
        let mut correspondence = get_correspondence(val, &seed_to_soil);
        correspondence = get_correspondence(correspondence, &soil_to_fer);
        correspondence = get_correspondence(correspondence, &fer_to_water);
        correspondence = get_correspondence(correspondence, &water_to_light);
        correspondence = get_correspondence(correspondence, &light_to_temp);
        correspondence = get_correspondence(correspondence, &temp_to_hum);
        correspondence = get_correspondence(correspondence, &hum_to_loc);

        if correspondence < min {
            min = correspondence;
        }
    }

    return min;
}

fn part2() -> isize {
    let seeds = read_seeds();
    let seed_to_soil = read_mapping();
    let soil_to_fer = read_mapping();
    let fer_to_water = read_mapping();
    let water_to_light = read_mapping();
    let light_to_temp = read_mapping();
    let temp_to_hum = read_mapping();
    let hum_to_loc = read_mapping();

    let mut min = isize::max_value();

    let mut start: isize = 0;
    let mut is_range = false;
    let mut minRange = isize::max_value();
    let mut maxRange = isize::min_value();

    for val in seeds {
        if !is_range {
            start = val;
            is_range = true;
            continue;
        }

        println!("123354");

        is_range = false;
        for seed in start..start + val {
            if seed >= minRange && seed <= maxRange {
                continue;
            }

            if (seed >= maxRange) {
                maxRange = seed;
            }

            if (seed <= minRange) {
                minRange = seed;
            }

            let mut correspondence = get_correspondence(seed, &seed_to_soil);
            correspondence = get_correspondence(correspondence, &soil_to_fer);
            correspondence = get_correspondence(correspondence, &fer_to_water);
            correspondence = get_correspondence(correspondence, &water_to_light);
            correspondence = get_correspondence(correspondence, &light_to_temp);
            correspondence = get_correspondence(correspondence, &temp_to_hum);
            correspondence = get_correspondence(correspondence, &hum_to_loc);

            if correspondence < min {
                min = correspondence;
            }
        }
    }

    return min;
}
