use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let inputs = parse_input();
    // let result = part_1(&inputs, "AAA", "ZZZ");

    let start: Vec<String> = inputs
        .nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|f| f.clone())
        .collect();
    println!("{}", start.len());
    let result2 = part_2(&inputs, start, &'Z');

    // println!("{}", result);
    println!("{}", result2);
}

struct Input {
    nodes: HashMap<String, Node>,
    directions: Vec<char>,
}

#[derive(Clone)]
struct Node {
    value: String,
    left: String,
    right: String,
    path: String,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match self.cmp(&other) {
            Ordering::Equal => true,
            _ => false,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.path.len().cmp(&other.path.len());
    }
}

fn parse_input() -> Input {
    let mut inputs = HashMap::<String, Node>::new();

    println!("start");
    let mut raw_line = String::new();
    let _ = std::io::stdin().read_line(&mut raw_line);
    let directions: Vec<char> = raw_line.trim().chars().collect();
    let _ = std::io::stdin().read_line(&mut raw_line);
    loop {
        raw_line.clear();
        let _ = std::io::stdin().read_line(&mut raw_line);
        let line = raw_line.trim();
        if line.len() == 0 {
            break;
        }

        let mut value_paths = line.split(" = ");
        let value = value_paths.next().unwrap().to_string();
        let mut paths = value_paths
            .next()
            .unwrap()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(", ");

        let left = paths.next().unwrap().trim().to_string();
        let right = paths.next().unwrap().trim().to_string();

        let node = Node {
            value: value.clone(),
            left: left,
            right: right,
            path: String::new(),
        };

        inputs.entry(value).or_insert(node);
    }

    return Input {
        nodes: inputs,
        directions,
    };
}

fn part_1(inputs: &Input, start: &str, end: &char) -> usize {
    let mut steps = 0;
    let mut next = start.to_string();
    for direction in &inputs.directions {
        steps += 1;

        if *direction == 'L' {
            next = inputs.nodes.get(&next).unwrap().left.clone();
        } else {
            next = inputs.nodes.get(&next).unwrap().right.clone();
        }

        if next.ends_with(*end) {
            return steps;
        }
    }

    return steps + part_1(&inputs, &next, end);
}

fn part_2(inputs: &Input, start: Vec<String>, end: &char) -> usize {
    let mut next = start;
    let mut results = Vec::<usize>::new();

    for s in next {
        results.push(part_1(inputs, &s, end))
    }

    return results
        .iter()
        .map(|f| *f)
        .fold(1usize, num_integer::lcm::<usize>);
}

// RL

// AAA = (BBB, CCC)
// BBB = (DDD, EEE)
// CCC = (ZZZ, GGG)
// DDD = (DDD, DDD)
// EEE = (EEE, EEE)
// GGG = (GGG, GGG)
// ZZZ = (ZZZ, ZZZ)
