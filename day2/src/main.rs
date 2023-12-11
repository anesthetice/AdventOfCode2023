use std::io::Read;

#[derive(Debug)]
struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new_red(value: u8) -> Self {
        Self { red: value, green: 0, blue: 0 }
    }
    pub fn new_green(value: u8) -> Self {
        Self { red: 0, green: value, blue: 0 }
    }
    pub fn new_blue(value: u8) -> Self {
        Self { red: 0, green: 0, blue: value }
    }
    pub fn new_null() -> Self {
        Self { red: 0, green: 0, blue: 0 }
    }
    pub fn is_possible(&self, other: &Color) -> bool {
        other.red>=self.red && other.green>=self.green && other.blue>=self.blue
    }
}

impl std::ops::Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.red = self.red + rhs.red;
        self.blue = self.blue + rhs.blue;
        self.green = self.green + rhs.green;
    }
}


/// input should look like this : "4 green" 
fn parse_color(input: &str) -> Color {
    let input: Vec<&str> = input.split(" ").collect();
    let value: u8 = input[0].parse().unwrap();
    match input[1] {
        "red" => Color::new_red(value),
        "green" => Color::new_green(value),
        "blue" => Color::new_blue(value),
        _ => panic!("failed to parse '{}'", input[1])
    }
}

/// set should look like this : "7 red, 7 blue, 9 green"
fn parse_set(input: &str) -> Color {
    let mut output: Color = Color::new_null();
    input.split(", ").for_each(|substring| {
        output += parse_color(substring);
    });
    output
}

/// ignores the tag just retuns the sum of the sets
fn parse_line(input: &str) -> Color {
    let input = &input[input.find(':').unwrap()+2..];
    let mut output: Color = Color::new_null();
    input.split("; ").for_each(|substring| {
        output += parse_set(substring);
    });

    output
}

fn main() {
    let mut input: String = String::new();
    std::fs::OpenOptions::new()
        .create(false)
        .read(true)
        .open("./input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let bag: Color = Color { red: 12, green: 13, blue: 14 };

    let mut output: usize = 0;
    let mut counter: usize = 1;
    for line in input.trim().lines() {
        println!("line: '{}'\nparsed: {:?}", line, parse_line(line));
        if parse_line(line).is_possible(&bag) {
            println!("possible, adding {}", counter);
            output += counter
        }
        counter += 1;
        println!("");
    }

    println!("{}", output);
}