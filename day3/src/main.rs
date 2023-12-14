use core::num;
use std::{io::Read, fmt::Debug};

fn main() {
    let mut input: String = String::new();
    std::fs::OpenOptions::new()
        .create(false)
        .read(true)
        .open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let numbers: Vec<Number> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(index, line)| {
            parse_line_for_numbers(line, index)
        })
        .collect();

    let symbols: Vec<Symbol> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(index, line)| {
            parse_line_for_symbols(line, index)
        })
        .collect();

    let mut sum: u32 = 0;
    for symbol in symbols.iter() {
        let adjacent_positions: Vec<(usize, usize)> = vec![
            (symbol.line_idx-1, symbol.pos_idx-1), (symbol.line_idx-1, symbol.pos_idx), (symbol.line_idx-1, symbol.pos_idx+1),
            (symbol.line_idx, symbol.pos_idx-1), (symbol.line_idx, symbol.pos_idx), (symbol.line_idx, symbol.pos_idx+1),
            (symbol.line_idx+1, symbol.pos_idx-1), (symbol.line_idx+1, symbol.pos_idx), (symbol.line_idx+1, symbol.pos_idx+1),
        ];

        for number in numbers.iter() {
            'main :
            for sub_idx in number.start_idx..(number.start_idx+number.length) {
                if adjacent_positions.contains(&(number.line_idx, sub_idx)) {
                    sum += number.value;
                    break 'main;
                }
            }
        }
    }
    println!("{}", sum);

    // part 2
    let mut sum: u32 = 0;
    for symbol in symbols.iter() {

        let adjacent_positions: Vec<(usize, usize)> = vec![
            (symbol.line_idx-1, symbol.pos_idx-1), (symbol.line_idx-1, symbol.pos_idx), (symbol.line_idx-1, symbol.pos_idx+1),
            (symbol.line_idx, symbol.pos_idx-1), (symbol.line_idx, symbol.pos_idx), (symbol.line_idx, symbol.pos_idx+1),
            (symbol.line_idx+1, symbol.pos_idx-1), (symbol.line_idx+1, symbol.pos_idx), (symbol.line_idx+1, symbol.pos_idx+1),
        ];
    
        let adjacent_numbers: Vec<Number> = numbers.iter().filter_map(|number| {
            for sub_idx in number.start_idx..(number.start_idx+number.length) {
                if adjacent_positions.contains(&(number.line_idx, sub_idx)) {
                    return Some(number.clone());
                }
            }
            return None
        }).collect();

        if adjacent_numbers.len() == 2 {
            sum += adjacent_numbers[0].value * adjacent_numbers[1].value;
        }
    }
    println!("{}", sum);
}

#[derive(Debug, Clone)]
struct Number {
    value: u32,
    line_idx: usize,
    start_idx: usize,
    length: usize,
}

impl Number {
    fn new(value: u32, line_idx: usize, start_idx: usize, length: usize) -> Self {
        Self { value, line_idx, start_idx, length}
    }
    /// this method is meant to be continously called in a iterator that spans a string until it returns false
    /// it adds the provided character to the back of the number and returns true if the character is a valid digit
    /// otherwises returns an error indicating that the number has finished being 'built'
    fn cycle(&mut self, input: char) -> Result<(), ()> {
        match input.to_digit(10) {
            Some(num) => {
                self.value = (self.value * 10) + num;
                self.length += 1;
                Ok(())
            }
            None => {
                Err(())
            }
        }
    }
}

fn parse_line_for_numbers(input: &str, line_idx: usize) -> Vec<Number> {
    let mut output: Vec<Number> = Vec::new();
    let mut output_part: Number = Number::new(0, line_idx, 0, 0);
    for (idx, chr) in input.char_indices() {
        if output_part.length == 0 {
            output_part.start_idx = idx;
        }
        match output_part.cycle(chr) {
            Ok(()) => (),
            Err(()) => {
                if output_part.length != 0 {
                    output.push(output_part.clone());
                    output_part.value = 0;
                    output_part.length = 0;
                    output_part.start_idx = idx;
                }
            },
        }
        // edge case, if the number is situated at the edge of the grid it will not be pushed out
        if idx == input.chars().count()-1 {
            output.push(output_part.clone());
        }
    }
    output
}


#[derive(Debug)]
struct Symbol {
    line_idx: usize,
    pos_idx: usize,
}

impl Symbol {
    fn new(line_idx: usize, pos_idx: usize) -> Self {
        Symbol { line_idx, pos_idx }
    }
}

fn parse_line_for_symbols(input: &str, line_idx: usize) -> Vec<Symbol> {
    let mut output: Vec<Symbol> = Vec::new();
    for (idx, chr) in input.chars().enumerate() {
        if chr != '.' && !chr.is_digit(10) {
            output.push(Symbol::new(line_idx, idx))
        }
    }
    output
}