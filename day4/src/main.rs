


fn main() {
    let input = include_str!("../input.txt");

    // part 1
    let mut sum: u32 = 0;
    for line in input.trim().lines() {
        sum += parse_line_part_1(line);
    }
    println!("{}", sum);

    // part 2
    let mut sum: u32 = 0;
    let full_input: Vec<&str> = input.trim().lines().collect();
    for (index, line) in input.trim().lines().enumerate() {
        parse_line_part_2(line, index, &full_input, &mut sum);
    }
    println!("{}", sum);
}

fn parse_numbers(input: &str, separator: char) -> Vec<u32> {
    input.split(separator).filter_map(|substring| {
        substring.parse::<u32>().ok()
    }).collect::<Vec<u32>>()
}

/// takes a line as an argument and outputs the final score for that line
fn parse_line_part_1(input: &str) -> u32 {
    // cleaning up the input first
    let input = &input[input.find(':').unwrap()+1..];

    // splitting the input into input_0 (the winning numbers) and input_1 (the elf's numbers)
    let input: Vec<&str> = input.split('|').collect();
    let input_0: &str = input[0];
    let input_1: &str = input[1];

    let input_0: Vec<u32> = parse_numbers(input_0.trim(), ' ');
    let input_1: Vec<u32> = parse_numbers(input_1.trim(), ' ');

    let mut output: u32 =  0;

    for elf_num in input_1.iter() {
        if input_0.contains(elf_num) {
            if output == 0 {
                output += 1;
            } else {
                output *= 2;
            }
        }
    }
    output
}

// recursively parses a line
fn parse_line_part_2(input: &str, index: usize, full_input: &Vec<&str>, sum: &mut u32) {
        // cleaning up the input first
        let input = &input[input.find(':').unwrap()+1..];

        // splitting the input into input_0 (the winning numbers) and input_1 (the elf's numbers)
        let input: Vec<&str> = input.split('|').collect();
        let input_0: &str = input[0];
        let input_1: &str = input[1];
    
        let input_0: Vec<u32> = parse_numbers(input_0.trim(), ' ');
        let input_1: Vec<u32> = parse_numbers(input_1.trim(), ' ');

        // adds 1 to the sum now (because we have 1 card)
        *sum += 1;

        // recursive bs
        let mut index_var: usize = 1;
        for elf_num in input_1.iter() {
            if input_0.contains(elf_num) {
                parse_line_part_2(full_input[index+index_var], index+index_var, full_input, sum);
                index_var += 1;
            }
        }
}


#[cfg(test)]
mod tests {
    #[test]
    fn parse_numbers_test() {
        assert_eq!(
            crate::parse_numbers("1 2 34 72", ' '),
            vec![1_u32, 2_u32, 34_u32, 72_u32]
        )
    }
    #[test]
    fn parse_line_part_1_test() {
        assert_eq!(
            crate::parse_line_part_1("Card   1: 74  8  2 86 40 25 93 17 61 32 | 65 25 73 55 75 94 54 99 53 17 89  4 44 13 15 32 57 92  8 21 74 64  5 87 24"),
            16_u32
        )
    }
}   