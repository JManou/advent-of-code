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
    .split('\n') {
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

fn part_1(inputs: &Vec<Vec<Vec<char>>>) -> usize{
    let mut result = 0;
    for pattern in inputs {
        result += 100 * check_horizontal_matching(pattern);
        result += check_vertical_matching(pattern);
    }

    return result;
}

fn part_2(inputs: &Vec<Vec<Vec<char>>>) -> usize{
    return 0;
}

fn check_vertical_matching(inputs: &Vec<Vec<char>>) -> usize {
    let mut i = 0;
    let max = inputs[0].len() - 1;

    loop {
        if i >= max {
            break;
        }

        if compare_column(inputs, i, i+1) && (i == max - 1 || i == 0 || check_vertical_reflection(inputs, i)) {
            return i+1;
        }

        i+= 1;
    }

    return 0;
}

fn check_horizontal_matching(inputs: &Vec<Vec<char>>) -> usize {
    let mut i = 0;

    loop {
        if i >= inputs.len() - 1 {
            break;
        }

        if inputs.get(i) == inputs.get(i+1) && (i == 0 || inputs.len() - 2 == i || check_horizontal_reflection(inputs, i)) {      
            return i+1;
        }

        i+= 1;
    }

    return 0;
}

fn check_horizontal_reflection(inputs: &Vec<Vec<char>>, i: usize) -> bool {
    let mut lc = i-1;
    let mut rc = i+2;

    loop {
        let c = inputs.get(lc) == inputs.get(rc);

        if !c {
            return false;
        }
        
        if lc == 0 || rc == inputs.len() - 1 {
            break;
        }

        lc -= 1;
        rc += 1; 
    }

    return true;
}

fn check_vertical_reflection(inputs: &Vec<Vec<char>>, i: usize) -> bool {
    let mut lc = i-1;
    let mut rc = i+2;
    let columns_len = inputs[0].len();

    loop {
        let c = compare_column(inputs, lc, rc);

        if !c {
            return false;
        }
        
        if lc == 0 || rc == columns_len - 1 {
            break;
        }

        lc -= 1;
        rc += 1; 
    }

    return true;
}

fn compare_column(inputs: &Vec<Vec<char>>, i: usize, j: usize) -> bool
{
    for r in 0..inputs.len() {
        if inputs.get(r).unwrap().get(i) != inputs.get(r).unwrap().get(j) {
            return false;
        }
    }

    return true;
}
