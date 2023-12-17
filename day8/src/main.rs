use std::collections::HashMap;

fn main() {
    let input: &'static str = include_str!("../input.txt");
    main_part_1(input);

}

fn main_part_1(input: &str) {
    let (instructions, network) = input
        .split_once("\n\n")
        .unwrap();

    let mut hashmap: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in network.lines() {
        parse_line(line, &mut hashmap);
    }
    let mut counter: usize = 0;
    let mut key: String = String::from("AAA");
    while key.as_str() != "ZZZ" {
        for instruction in instructions.chars() {
            match instruction {
                'L' => key = hashmap.get(key.as_str()).unwrap().0.to_string(),
                'R' => key = hashmap.get(key.as_str()).unwrap().1.to_string(),
                _ => unreachable!(),
            }
            counter += 1;

            if key.as_str() == "ZZZ" {
                break;
            }
        }
    }

    println!("{}", counter);
}

fn parse_line<'a>(input: &'a str, output: &mut HashMap<&'a str, (&'a str, &'a str)>) {
    let (key, value) = input
        .split_once(" = ")
        .unwrap();

    let value = value[1..value.len()-1]
        .split_once(", ")
        .unwrap();

    output.insert(key, value);
}



