use std::collections::HashMap;

fn main() {
    let mut raw_line = String::new();
    let mut input_array = Vec::<Vec<char>>::new();

    println!("start");
    loop {
        raw_line.clear();
        let _ = std::io::stdin().read_line(&mut raw_line);
        let line = raw_line.trim();
        if line.len() == 0 {
            println!("read all input");
            break;
        }

        input_array.push(line.chars().collect());
    }

    println!("part 1 {}", part_1(&input_array));
    println!("part 2 {}", part_2(&input_array));
}

fn part_1(input: &Vec<Vec<char>>) -> u32 {
    let mut valid_numbers = Vec::<u32>::new();
    let x_len = input[0].len();
    let y_len = input.len();
    for y in 0..y_len {
        let line = input.get(y).unwrap();
        let mut number = String::new();
        let mut is_valid = false;
        for x in 0..x_len {
            let car = *line.get(x).unwrap();
            if !car.is_digit(10) {
                if is_valid {
                    // println!("{:?}", valid_numbers);
                    valid_numbers.push(u32::from_str_radix(number.as_str(), 10).unwrap())
                }

                is_valid = false;
                number.clear();
                continue;
            }

            number.push(car);

            if is_valid {
                continue;
            }

            // check above and under if possible
            {
                is_valid = check_above(&input, x, y) || check_under(&input, x, y);
            }

            if number.len() == 1 {
                is_valid |= check_left(line, x)
                    || check_left_above(&input, x, y)
                    || check_left_under(&input, x, y);
            }

            if x + 1 < x_len && !(*line.get(x + 1).unwrap()).is_ascii_digit() {
                is_valid |= check_right(line, x)
                    || check_right_above(&input, x, y)
                    || check_right_under(&input, x, y);
            }
        }

        if is_valid {
            // println!("{:?}", valid_numbers);
            valid_numbers.push(u32::from_str_radix(number.as_str(), 10).unwrap())
        }
    }

    return valid_numbers.iter().sum();
}

fn part_2(input: &Vec<Vec<char>>) -> u32 {
    let mut valid_numbers = HashMap::<(usize, usize), Vec<u32>>::new();
    let x_len = input[0].len();
    let y_len = input.len();
    for y in 0..y_len {
        let line = input.get(y).unwrap();
        let mut number = String::new();
        let mut is_valid = false;
        let mut x_point = 0;
        let mut y_point = 0;
        for x in 0..x_len {
            let car = *line.get(x).unwrap();
            if !car.is_digit(10) {
                if is_valid {
                    // println!("{:?}", valid_numbers);
                    let number = u32::from_str_radix(number.as_str(), 10).unwrap();
                    valid_numbers
                        .entry((x_point, y_point))
                        .or_insert(Vec::new())
                        .push(number);
                }

                is_valid = false;
                number.clear();
                continue;
            }

            number.push(car);

            if is_valid {
                continue;
            }

            // check above and under if possible
            if check_above(&input, x, y) {
                x_point = x;
                y_point = y - 1;
                is_valid = true
            }

            if check_under(&input, x, y) {
                x_point = x;
                y_point = y + 1;
                is_valid = true
            }

            if check_left(line, x) {
                x_point = x - 1;
                y_point = y;
                is_valid = true
            }

            if check_right(line, x) {
                x_point = x + 1;
                y_point = y;
                is_valid = true
            }

            if check_left_above(&input, x, y) {
                x_point = x - 1;
                y_point = y - 1;
                is_valid = true
            }

            if check_left_under(&input, x, y) {
                x_point = x - 1;
                y_point = y + 1;
                is_valid = true
            }

            if check_right_above(&input, x, y) {
                x_point = x + 1;
                y_point = y - 1;
                is_valid = true
            }

            if check_right_under(&input, x, y) {
                x_point = x + 1;
                y_point = y + 1;
                is_valid = true
            }
        }

        if is_valid {
            // println!("{:?}", valid_numbers);
            let number = u32::from_str_radix(number.as_str(), 10).unwrap();
            valid_numbers
                .entry((x_point, y_point))
                .or_insert(Vec::new())
                .push(number);
        }
    }

    return valid_numbers
        .values()
        .filter(|x| x.len() == 2)
        .map(|x| x.iter().product::<u32>())
        .sum();
}

fn is_valid_symbol(car: char) -> bool {
    return car != '.' && !car.is_ascii_digit();
}

fn check_left(input: &Vec<char>, x: usize) -> bool {
    if x == 0 {
        return false;
    }

    match input.get(x - 1) {
        Some(val) => is_valid_symbol(*val),
        None => false,
    }
}

fn check_right(input: &Vec<char>, x: usize) -> bool {
    match input.get(x + 1) {
        Some(val) => is_valid_symbol(*val),
        None => false,
    }
}

fn check_above(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if y == 0 {
        return false;
    }

    match input.get(y - 1) {
        Some(val) => is_valid_symbol(*val.get(x).unwrap()),
        None => false,
    }
}

fn check_under(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    match input.get(y + 1) {
        Some(val) => is_valid_symbol(*val.get(x).unwrap()),
        None => false,
    }
}

fn check_right_above(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if y == 0 {
        return false;
    }

    match input.get(y - 1) {
        Some(val) => check_right(val, x),
        None => false,
    }
}

fn check_right_under(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    match input.get(y + 1) {
        Some(val) => check_right(val, x),
        None => false,
    }
}

fn check_left_above(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if y == 0 {
        return false;
    }

    match input.get(y - 1) {
        Some(val) => check_left(val, x),
        None => false,
    }
}

fn check_left_under(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    match input.get(y + 1) {
        Some(val) => check_left(val, x),
        None => false,
    }
}
