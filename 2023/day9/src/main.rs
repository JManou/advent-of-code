use std::fs;


fn main() {
    println!("{}", std::env::current_dir().unwrap().display());
    let inputs = parse_input("input.txt");
    let result = part_1(&inputs);
    let result2 = part_2(&inputs);
    println!("{}", result);
    println!("{}", result2);
}

fn parse_input(filename: &str) -> Vec<Vec<isize>> {
    return fs::read_to_string(filename)
    .unwrap()
    .replace('\r', "")
    .split('\n')
    .map(|f| f.split(' ').map(|c| c.parse::<isize>().unwrap()).collect::<Vec<isize>>())
    .collect();
}

fn part_1(inputs: &Vec<Vec<isize>>) -> isize {
    let mut result = 0;

    for line in inputs {
        result += part_1_line(line);
    }

    return result;
}

fn part_1_line(line: &Vec<isize>) -> isize {

    if line.iter().all(|f| *f == 0) {
        return 0;
    }

    let mut current = line.get(0).unwrap();
    let mut next_line = Vec::<isize>::new();
    for next in line.iter().skip(1) {
        next_line.push(next - current);
        current = next;
    }

    return line.last().unwrap() + part_1_line(&next_line);
}

fn part_2(inputs: &Vec<Vec<isize>>) -> isize {
    let mut result = 0;

    for line in inputs {
        result += part_2_line(line);
    }

    return result;
}

fn part_2_line(line: &Vec<isize>) -> isize {

    if line.iter().all(|f| *f == 0) {
        return 0;
    }

    let mut current = line.get(0).unwrap();
    let mut next_line = Vec::<isize>::new();
    for next in line.iter().skip(1) {
        next_line.push(next - current);
        current = next;
    }

    return line.first().unwrap() - part_2_line(&next_line);
}