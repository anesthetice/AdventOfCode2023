use std::io::Read;
use std::path::Path;
use std::fs::OpenOptions;

fn main() {
    println!("1 : {}", trebuchet_one("calibration_document.txt"));
    println!("2 : {}", trebuchet_two("calibration_document.txt"));
}

fn trebuchet_one<Q: AsRef<Path>>(filepath: Q) -> u32 {
    let mut data: String = String::new();
    let mut file = OpenOptions::new().read(true).create(false).open(filepath).unwrap();
    file.read_to_string(&mut data).unwrap();
    
    let mut sum: u32 = 0;
    data.trim().split('\n').for_each(|substring| {
        let first_number: u32 = substring.chars().find(|chr| {chr.is_numeric()}).unwrap().to_digit(10).unwrap() * 10;
        let second_number: u32 = substring.chars().rfind(|chr| {chr.is_numeric()}).unwrap().to_digit(10).unwrap();
        sum += first_number + second_number;
    });
    sum
}

fn trebuchet_two<Q: AsRef<Path>>(filepath: Q) -> u32 {
    let alphabetical_numbers: [&'static str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut data: String = String::new();
    let mut file = OpenOptions::new().read(true).create(false).open(filepath).unwrap();
    file.read_to_string(&mut data).unwrap();

    let mut sum: u32 = 0;
    data.trim().split('\n').for_each(|substring| {
        // store the 'numeric' numbers alongside their index
        let (mut idx_1, num_1) = substring.chars().enumerate().find(|(idx, chr)| {chr.is_numeric()}).unwrap();
        let mut num_1 = num_1.to_digit(10).unwrap() * 10;
        let (mut idx_2, num_2) = substring.chars().rev().enumerate().find(|(idx, chr)| {chr.is_numeric()}).unwrap();
        let mut num_2 = num_2.to_digit(10).unwrap();
        // have to reverse the index again because .enumerate doesn't care if I reversed the iterator just before, sucks that I can't use rfind
        idx_2 = substring.len()-(1+idx_2);

        // check if 'alphabetical' numbers exist and if their starting index is lower than the current idx (higher for the second number), then replace
        let mut value: u32 = 1;
        alphabetical_numbers.iter().for_each(|alphabetical_number| {
            if let Some(idx) = substring.find(alphabetical_number) {
                if idx < idx_1 {
                    num_1=value*10;
                    idx_1=idx;
                }
            }
            if let Some(idx) = substring.rfind(alphabetical_number) {
                if idx > idx_2 {
                    num_2=value;
                    idx_2=idx;
                }
            }
            value += 1;
        });
        sum += num_1 + num_2;
    });
    sum
}