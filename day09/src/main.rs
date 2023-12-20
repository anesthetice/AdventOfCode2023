fn main() {
    let input: &'static str = include_str!("../input.txt");

    let output: (i64, i64) = input
        .lines()
        .map(|line| {
            let mut input: Vec<i64> = line
                .split_whitespace()
                .map(|substring| {substring.parse::<i64>().unwrap()})
                .collect::<Vec<i64>>();

            // this only stores the last digit of every 'step' until we reach a line of zeroes
            let mut output_1: Vec<i64> = Vec::new();
            // this only stores the first digit of every 'step' until we reach a line of zeroes
            let mut output_2: Vec<i64> = Vec::new();
            'main:
            loop {
                output_2.push(input[0].clone());
                for index in 0..input.len()-1 {
                    input[index] = input[index+1] - input[index]
                }
                output_1.push(input.pop().unwrap());
                if input.iter().filter(|x| **x == 0).count() == input.len() {
                    break 'main;
                }
            }
            (
                output_1.into_iter().fold(0_i64, |acc, x| {acc + x}),
                output_2.into_iter().enumerate().fold(0_i64, |acc, (idx, x)| {acc + x * i64::pow(-1, idx as u32)})
            )
        })
        .fold((0, 0), |acc, (x1, x2)| {(acc.0 + x1, acc.1 + x2)});

    println!("{}\n{}", output.0, output.1);
}