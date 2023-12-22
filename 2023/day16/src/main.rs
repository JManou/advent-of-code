use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn next(&self, direction: &Direction, x_max: usize, y_max: usize) -> Option<Self> {
        match direction {
            Direction::Left => {
                if self.x > 0 {
                    return Some(Position {
                        x: self.x - 1,
                        y: self.y,
                    });
                } else {
                    return None;
                }
            }
            Direction::Right => {
                if self.x < x_max {
                    return Some(Position {
                        x: self.x + 1,
                        y: self.y,
                    });
                } else {
                    return None;
                }
            }
            Direction::Down => {
                if self.y < y_max {
                    return Some(Position {
                        x: self.x,
                        y: self.y + 1,
                    });
                } else {
                    return None;
                }
            }
            Direction::Up => {
                if self.y > 0 {
                    return Some(Position {
                        x: self.x,
                        y: self.y - 1,
                    });
                } else {
                    return None;
                }
            }
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn main() {
    println!("{}", std::env::current_dir().unwrap().display());
    let inputs = parse_input("input.txt");

    let result = part_1(&inputs);
    let result2 = part_2(&inputs);
    println!("{}", result);
    println!("{}", result2);
}

fn parse_input(filename: &str) -> Vec<Vec<char>> {
    let mut container: Vec<Vec<char>> = Vec::new();

    for line in fs::read_to_string(filename)
        .unwrap()
        .replace('\r', "")
        .split('\n')
    {
        container.push(line.chars().collect());
    }

    return container;
}

fn part_1(inputs: &Vec<Vec<char>>) -> usize {
    let mut set = HashMap::<Position, HashSet<Direction>>::new();
    beam_navigation(
        inputs,
        Some(Position { x: 0, y: 0 }),
        &mut set,
        &Direction::Right,
    );
    return set.len();
}

fn part_2(inputs: &Vec<Vec<char>>) -> usize {
    let mut beam = 0usize;
    let x_max = inputs[0].len();
    let y_max = inputs.len();
    for x in 0..x_max {
        let mut set = HashMap::<Position, HashSet<Direction>>::new();
        beam_navigation(
            inputs,
            Some(Position { x: x, y: 0 }),
            &mut set,
            &Direction::Down,
        );

        if set.len() > beam {
            beam = set.len();
        }
    }

    for x in 0..x_max {
        let mut set = HashMap::<Position, HashSet<Direction>>::new();
        beam_navigation(
            inputs,
            Some(Position { x: x, y: y_max - 1 }),
            &mut set,
            &Direction::Up,
        );

        if set.len() > beam {
            beam = set.len();
        }
    }

    for y in 0..y_max {
        let mut set = HashMap::<Position, HashSet<Direction>>::new();
        beam_navigation(
            inputs,
            Some(Position { x: 0, y: y }),
            &mut set,
            &Direction::Right,
        );

        if set.len() > beam {
            beam = set.len();
        }
    }

    for y in 0..y_max {
        let mut set = HashMap::<Position, HashSet<Direction>>::new();
        beam_navigation(
            inputs,
            Some(Position { x: x_max - 1, y: y }),
            &mut set,
            &Direction::Left,
        );

        if set.len() > beam {
            beam = set.len();
        }
    }

    return beam;
}

fn beam_navigation(
    inputs: &Vec<Vec<char>>,
    position: Option<Position>,
    encountered_tiles: &mut HashMap<Position, HashSet<Direction>>,
    direction: &Direction,
) {
    if position == None {
        return;
    }

    let set = encountered_tiles
        .entry(position.unwrap())
        .or_insert(HashSet::<Direction>::new());
    let inserted = set.insert(*direction);
    let points = inputs.len() * inputs[0].len();
    let x_max = inputs[0].len() - 1;
    let y_max = inputs.len() - 1;
    // not accurate, some point might only be accessible from 2 points
    if !inserted
        || is_fully_energized(position.unwrap(), set.len(), x_max, y_max)
        || encountered_tiles.len() >= points
    {
        return;
    }

    match inputs[position.unwrap().y][position.unwrap().x] {
        '.' => dot_navigation(inputs, position.unwrap(), encountered_tiles, direction),
        '/' => left_mirror(inputs, position.unwrap(), encountered_tiles, direction),
        '\\' => right_mirror(inputs, position.unwrap(), encountered_tiles, direction),
        '|' => vertical_splitter(inputs, position.unwrap(), encountered_tiles, direction),
        '-' => horizontal_splitter(inputs, position.unwrap(), encountered_tiles, direction),
        _ => return,
    }
}

fn is_fully_energized(position: Position, len: usize, x_max: usize, y_max: usize) -> bool {
    if (position.x == 0 || position.x == x_max) && (position.y == 0 || position.y == y_max) {
        return len >= 2;
    }

    if position.y == 0 || position.y == y_max || position.x == 0 || position.x == x_max {
        return len >= 3;
    }

    return len >= 4;
}

fn dot_navigation(
    inputs: &Vec<Vec<char>>,
    position: Position,
    encountered_tiles: &mut HashMap<Position, HashSet<Direction>>,
    direction: &Direction,
) {
    let x_max = inputs[0].len() - 1;
    let y_max = inputs.len() - 1;

    let next = position.next(direction, x_max, y_max);
    return beam_navigation(inputs, next, encountered_tiles, direction);
}

fn left_mirror(
    inputs: &Vec<Vec<char>>,
    position: Position,
    encountered_tiles: &mut HashMap<Position, HashSet<Direction>>,
    direction: &Direction,
) {
    let x_max = inputs[0].len() - 1;
    let y_max = inputs.len() - 1;
    match direction {
        Direction::Left => beam_navigation(
            inputs,
            position.next(&Direction::Down, x_max, y_max),
            encountered_tiles,
            &Direction::Down,
        ),
        Direction::Right => beam_navigation(
            inputs,
            position.next(&Direction::Up, x_max, y_max),
            encountered_tiles,
            &Direction::Up,
        ),
        Direction::Up => beam_navigation(
            inputs,
            position.next(&Direction::Right, x_max, y_max),
            encountered_tiles,
            &Direction::Right,
        ),
        Direction::Down => beam_navigation(
            inputs,
            position.next(&Direction::Left, x_max, y_max),
            encountered_tiles,
            &Direction::Left,
        ),
    }
}

fn right_mirror(
    inputs: &Vec<Vec<char>>,
    position: Position,
    encountered_tiles: &mut HashMap<Position, HashSet<Direction>>,
    direction: &Direction,
) {
    let x_max = inputs[0].len() - 1;
    let y_max = inputs.len() - 1;
    match direction {
        Direction::Left => beam_navigation(
            inputs,
            position.next(&Direction::Up, x_max, y_max),
            encountered_tiles,
            &Direction::Up,
        ),
        Direction::Right => beam_navigation(
            inputs,
            position.next(&Direction::Down, x_max, y_max),
            encountered_tiles,
            &Direction::Down,
        ),
        Direction::Up => beam_navigation(
            inputs,
            position.next(&Direction::Left, x_max, y_max),
            encountered_tiles,
            &Direction::Left,
        ),
        Direction::Down => beam_navigation(
            inputs,
            position.next(&Direction::Right, x_max, y_max),
            encountered_tiles,
            &Direction::Right,
        ),
    }
}

fn vertical_splitter(
    inputs: &Vec<Vec<char>>,
    position: Position,
    encountered_tiles: &mut HashMap<Position, HashSet<Direction>>,
    direction: &Direction,
) {
    let x_max = inputs[0].len() - 1;
    let y_max = inputs.len() - 1;
    match direction {
        Direction::Left => {
            beam_navigation(
                inputs,
                position.next(&Direction::Up, x_max, y_max),
                encountered_tiles,
                &Direction::Up,
            );

            beam_navigation(
                inputs,
                position.next(&Direction::Down, x_max, y_max),
                encountered_tiles,
                &Direction::Down,
            );
        }
        Direction::Right => {
            beam_navigation(
                inputs,
                position.next(&Direction::Up, x_max, y_max),
                encountered_tiles,
                &Direction::Up,
            );

            beam_navigation(
                inputs,
                position.next(&Direction::Down, x_max, y_max),
                encountered_tiles,
                &Direction::Down,
            );
        }
        Direction::Up => beam_navigation(
            inputs,
            position.next(direction, x_max, y_max),
            encountered_tiles,
            direction,
        ),
        Direction::Down => beam_navigation(
            inputs,
            position.next(direction, x_max, y_max),
            encountered_tiles,
            direction,
        ),
    }
}

fn horizontal_splitter(
    inputs: &Vec<Vec<char>>,
    position: Position,
    encountered_tiles: &mut HashMap<Position, HashSet<Direction>>,
    direction: &Direction,
) {
    let x_max = inputs[0].len() - 1;
    let y_max = inputs.len() - 1;
    match direction {
        Direction::Left => beam_navigation(
            inputs,
            position.next(direction, x_max, y_max),
            encountered_tiles,
            direction,
        ),
        Direction::Right => beam_navigation(
            inputs,
            position.next(direction, x_max, y_max),
            encountered_tiles,
            direction,
        ),
        Direction::Up => {
            beam_navigation(
                inputs,
                position.next(&Direction::Left, x_max, y_max),
                encountered_tiles,
                &Direction::Left,
            );

            beam_navigation(
                inputs,
                position.next(&Direction::Right, x_max, y_max),
                encountered_tiles,
                &Direction::Right,
            );
        }
        Direction::Down => {
            beam_navigation(
                inputs,
                position.next(&Direction::Left, x_max, y_max),
                encountered_tiles,
                &Direction::Left,
            );

            beam_navigation(
                inputs,
                position.next(&Direction::Right, x_max, y_max),
                encountered_tiles,
                &Direction::Right,
            );
        }
    }
}
