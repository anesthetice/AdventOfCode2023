
fn main() {
    main_part1();
    main_part2();
}

fn main_part1() {
    let input: &'static str = include_str!("../input.txt");
    let input: Vec<&str> = input.trim().lines().collect();

    let times: Vec<u64> = input[0][9..]
        .split_whitespace()
        .flat_map(|substring| {substring.parse::<u64>().ok()})
        .collect();

    let distances: Vec<u64> = input[1][9..]
        .split_whitespace()
        .flat_map(|substring| {substring.parse::<u64>().ok()})
        .collect();

    let times_to_distances: Vec<(u64, u64)> = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| { (time, distance) })
        .collect();

    println!("{}", process(times_to_distances));
}

fn main_part2() {
    let input: &'static str = include_str!("../input.txt");
    let input: Vec<&str> = input.trim().lines().collect();

    let times: Vec<u64> = vec![(input[0][9..]
        .replace(" ", "")
        .parse::<u64>()
        .unwrap())];

    let distances: Vec<u64> = vec![(input[1][9..]
        .replace(" ", "")
        .parse::<u64>()
        .unwrap())];

    let times_to_distances: Vec<(u64, u64)> = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| { (time, distance) })
        .collect();

    println!("{}", process(times_to_distances));
}

fn process(times_to_distances: Vec<(u64, u64)>) -> u64 {
    times_to_distances
        .into_iter()
        .map(|(time, comp_distance)| {
            let mut output: u64 = 0;
            for speed in 1..time {
                // the value of our speed is equal to the value of the initial time lost
                let time_left: u64 = time - speed;
                if time_left * speed > comp_distance {
                    output += 1;
                }
            }
            output
        })
        .fold(1, |acc, x| {acc*x})
}