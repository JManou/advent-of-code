use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

fn main() {
    let inputs = parse_input();
    let result = best_first_search(&inputs, "AAA", "ZZZ");

    println!("{}", result);
    println!("{}", result.len());
}

enum Direction {
    Left,
    Right,
}

#[derive(Clone)]
struct Node {
    value: String,
    left: String,
    right: String,
    path: String,
}

impl Node {
    fn from_parent(&mut self, parent_path: String, direction: Direction) {
        self.path.push_str(&parent_path);
        self.path.push(match direction {
            Direction::Left => 'L',
            Direction::Right => 'R',
        });
    }
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

fn parse_input() -> HashMap<String, Node> {
    let mut inputs = HashMap::<String, Node>::new();

    println!("start");
    let mut raw_line = String::new();
    let _ = std::io::stdin().read_line(&mut raw_line);
    let is_lr = raw_line.starts_with('L');
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

        let first = paths.next().unwrap().trim().to_string();
        let second = paths.next().unwrap().trim().to_string();
        let left: String;
        let right: String;

        if is_lr {
            left = first;
            right = second;
        } else {
            right = first;
            left = second;
        }

        let node = Node {
            value: value.clone(),
            left: left,
            right: right,
            path: String::new(),
        };

        inputs.entry(value).or_insert(node);
    }

    return inputs;
}

fn best_first_search(inputs: &HashMap<String, Node>, start: &str, end: &str) -> String {
    let mut encountered = HashSet::<String>::new();
    let mut p_queue = BinaryHeap::<Node>::new();

    let parent = inputs.get(start).unwrap();

    let mut left_node = inputs.get(&parent.left).unwrap().clone();
    left_node.from_parent(parent.path.clone(), Direction::Left);
    let mut right_node = inputs.get(&parent.right).unwrap().clone();
    right_node.from_parent(parent.path.clone(), Direction::Right);

    p_queue.push(left_node);
    p_queue.push(right_node);

    while let Some(node) = p_queue.pop() {
        // Alternatively we could have continued to find all shortest paths
        if node.value == end {
            return node.path;
        }

        // Important as we may have already found a better way
        if !encountered.insert(node.value) {
            continue;
        }

        let mut l_node = inputs.get(&node.left).unwrap().clone();
        l_node.from_parent(node.path.clone(), Direction::Left);
        let mut r_node = inputs.get(&node.right).unwrap().clone();
        r_node.from_parent(node.path.clone(), Direction::Right);

        p_queue.push(l_node);
        p_queue.push(r_node);
    }

    return String::new();
}

// RL

// AAA = (BBB, CCC)
// BBB = (DDD, EEE)
// CCC = (ZZZ, GGG)
// DDD = (DDD, DDD)
// EEE = (EEE, EEE)
// GGG = (GGG, GGG)
// ZZZ = (ZZZ, ZZZ)
