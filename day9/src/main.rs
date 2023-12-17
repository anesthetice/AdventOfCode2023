fn main() {
    let input: &'static str = include_str!("../input.txt");
    part_1(input);
}

fn part_1(input: &str) {

    let output = input
        .lines()
        .map(|line| {
            let mut input: Vec<i64> = line
                .split_whitespace()
                .map(|substring| {substring.parse::<i64>().unwrap()})
                .collect::<Vec<i64>>();

            // this only stores the last digit of every "step" until the line of zeroes
            let mut output: Vec<i64> = Vec::new();
            'main:
            loop {
                for index in 0..input.len()-1 {
                    input[index] = input[index+1] - input[index]
                }
                output.push(input.pop().unwrap());
                if input.iter().filter(|x| **x == 0).count() == input.len() {
                    break 'main;
                }
            }
            output.into_iter().fold(0_i64, |acc, x| {acc + x})
        })
        .fold(0, |acc, x| {acc + x});

    println!("{}", output);
}