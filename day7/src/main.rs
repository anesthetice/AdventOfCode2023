mod part1;
mod part2;

fn main() {
    let input: &str = include_str!("../input.txt");
    part1_main(input);
    part2_main(input);
}

fn part1_main(input: &str) {
    use crate::part1::*;

    
    let mut hands: Vec<Hand> = Vec::new();
    for line in input.trim().lines() {
        Hand::from(line).incorporate(&mut hands);
    }

    let winnings: u64 = hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (idx, hand)| {acc + hand.bid * (idx as u64+1)});
    println!("{}", winnings);
}

fn part2_main(input: &str) {
    use crate::part2::*;

    let mut hands: Vec<Hand> = Vec::new();
    for line in input.trim().lines() {
        Hand::from(line).incorporate(&mut hands);
    }

    let winnings: u64 = hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (idx, hand)| {acc + hand.bid * (idx as u64+1)});
    println!("{}", winnings);
}