use std::fs;

fn main() {
    println!("{}", std::env::current_dir().unwrap().display());
    let inputs = parse_input("input.txt");

    let result = part_1(&inputs);
    let result2 = part_2(&inputs);
    println!("{}", result);
    println!("{}", result2);
}

fn parse_input(filename: &str) -> Vec<Vec<Vec<char>>> {
    let mut input: Vec<Vec<Vec<char>>> = Vec::new();

    let mut container: Vec<Vec<char>> = Vec::new();

    for line in fs::read_to_string(filename)
        .unwrap()
        .replace('\r', "")
        .split('\n')
    {
        if line == "" {
            input.push(container);
            container = Vec::new();
            continue;
        }

        container.push(line.chars().collect());
    }

    input.push(container);

    return input;
}

fn part_1(inputs: &Vec<Vec<Vec<char>>>) -> usize {
    let mut result = 0;
    for pattern in inputs {
        result += 100 * check_horizontal_matching(pattern, 0);
        result += check_vertical_matching(pattern, 0);
    }

    return result;
}

fn part_2(inputs: &Vec<Vec<Vec<char>>>) -> usize {
    let mut result = 0;
    for pattern in inputs {
        result += 100 * check_horizontal_matching(pattern, 1);
        result += check_vertical_matching(pattern, 1);
    }

    return result;
}

fn check_vertical_matching(inputs: &Vec<Vec<char>>, diff: usize) -> usize {
    let mut i = 0;
    let max = inputs[0].len() - 1;

    loop {
        if i >= max {
            break;
        }

        let c = compare_column(inputs, i, i + 1);

        if c <= diff && check_vertical_reflection(inputs, i, diff - c) {
            return i + 1;
        }

        i += 1;
    }

    return 0;
}

fn check_horizontal_matching(inputs: &Vec<Vec<char>>, diff: usize) -> usize {
    let mut i = 0;

    loop {
        if i >= inputs.len() - 1 {
            break;
        }

        let c = compare_line(inputs, i, i + 1);

        if c <= diff && check_horizontal_reflection(inputs, i, diff - c) {
            return i + 1;
        }

        i += 1;
    }

    return 0;
}

fn check_horizontal_reflection(inputs: &Vec<Vec<char>>, i: usize, diff: usize) -> bool {
    let mut rc = i + 2;
    let mut cm = diff > 0;

    if i == 0 || rc > inputs.len() - 1 {
        return !cm;
    }

    let mut lc = i - 1;

    loop {
        let c = compare_line(inputs, lc, rc);

        if c != 0 && (!cm || c != 1) {
            return false;
        }

        if cm && c == 1 {
            cm = false;
        }

        if lc == 0 || rc == inputs.len() - 1 {
            break;
        }

        lc -= 1;
        rc += 1;
    }

    if cm {
        return false;
    }

    return true;
}

fn check_vertical_reflection(inputs: &Vec<Vec<char>>, i: usize, diff: usize) -> bool {
    let mut cm = diff > 0;
    let mut rc = i + 2;
    let columns_len = inputs[0].len();

    if i == 0 || rc > columns_len - 1 {
        return !cm;
    }

    let mut lc = i - 1;

    loop {
        let c = compare_column(inputs, lc, rc);

        if c != 0 && (!cm || c != 1) {
            return false;
        }

        if cm && c == 1 {
            cm = false;
        }

        if lc == 0 || rc == columns_len - 1 {
            break;
        }

        lc -= 1;
        rc += 1;
    }

    if cm {
        return false;
    }

    return true;
}

fn compare_column(inputs: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let mut nm: usize = 0;
    for r in 0..inputs.len() {
        if inputs.get(r).unwrap().get(i) != inputs.get(r).unwrap().get(j) {
            nm += 1;
        }
    }

    return nm;
}

fn compare_line(inputs: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let mut nm: usize = 0;
    for r in 0..inputs[0].len() {
        if inputs.get(i).unwrap().get(r) != inputs.get(j).unwrap().get(r) {
            nm += 1;
        }
    }

    return nm;
}
