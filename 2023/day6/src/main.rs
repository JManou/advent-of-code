fn main() {
    let mut raw_line = String::new();
    let _ = std::io::stdin().read_line(&mut raw_line);

    let times: Vec<usize> = raw_line
        .trim()
        .split(": ")
        .last()
        .unwrap()
        .split(' ')
        .filter(|p| *p != "")
        .map(|d| d.parse::<usize>().unwrap())
        .collect();
    let mut raw_line = String::new();

    let _ = std::io::stdin().read_line(&mut raw_line);

    let distances: Vec<usize> = raw_line
        .trim()
        .split(": ")
        .last()
        .unwrap()
        .trim()
        .split(' ')
        .filter(|p| *p != "")
        .map(|d| d.parse::<usize>().unwrap())
        .collect();

    let result = part1(&times, &distances);
    let result2 = part2(&times, &distances);

    println!("part1 {}", result);
    println!("part2 {}", result2)
}

fn part1(times: &Vec<usize>, distances: &Vec<usize>) -> usize {
    let mut result = 1;
    for i in 0..times.len() {
        let t_time = *times.get(i).unwrap();
        let distance = *distances.get(i).unwrap();
        let mut solution = 0;
        let mut found = false;

        for t in 1..t_time {
            let travelled = (t_time - t) * t;

            if travelled > distance {
                solution += 1;
                found = true
            } else if found {
                break;
            }
        }

        result *= solution;
    }

    return result;
}

fn part2(times: &Vec<usize>, distances: &Vec<usize>) -> usize {
    let mut result = 1;

    let t = String::from_iter(times.iter().map(|f| f.to_string()))
        .parse::<usize>()
        .unwrap();
    let d = String::from_iter(distances.iter().map(|f| f.to_string()))
        .parse::<usize>()
        .unwrap();

    let mut tt = Vec::<usize>::new();
    tt.push(t);

    let mut dd = Vec::<usize>::new();
    dd.push(d);

    return part1(&tt, &dd);
}
