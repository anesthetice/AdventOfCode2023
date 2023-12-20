use std::collections::HashMap;

fn main() {
    let input: &'static str = include_str!("../input.txt");
    main_part_1(input);
    main_part_2(input);
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

fn main_part_2(input: &str) {
    let (instructions, network) = input
        .split_once("\n\n")
        .unwrap();

    let mut hashmap: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut starting_nodes: Vec<&str> = Vec::new();

    for line in network.lines() {
        if &line[2..3] == "A" {
            starting_nodes.push(&line[0..3]);
        }
        parse_line(line, &mut hashmap);
    }

    let compute = |initial_key: &str, hashmap: &HashMap<&str, (&str, &str)>, instructions: &str| -> usize {
        let mut key: String = initial_key.to_string();
        let mut counter: usize = 0;
        loop {
            for instruction in instructions.chars() {
                match instruction {
                    'L' => key = hashmap.get(key.as_str()).unwrap().0.to_string(),
                    'R' => key = hashmap.get(key.as_str()).unwrap().1.to_string(),
                    _ => unreachable!(),
                }
                counter += 1;
                if key.ends_with('Z') {
                    return counter
                }
            }
        }
    };

    let GCD = |mut a: usize, mut b: usize| -> usize {
        if b > a {
            let temp = b;
            b = a;
            a = temp;
        }
        if b == 0 {
            return a;
        }
        loop {
            let res = a % b;
            if res == 0 {
                return b;
            }
            a = b;
            b = res;
        }
    };

    let LCM = |a: usize, b: usize| -> usize {
        if a == 0 {
            return b;
        } else if b == 0 {
            return a;
        }
        a * b / GCD(a, b)
    };

    let steps: usize = starting_nodes
        .iter()
        .map(|key| {
            compute(key, &hashmap, instructions)
        })
        .fold(0, |acc, x| {LCM(acc, x)});

    println!("{}", steps);
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





