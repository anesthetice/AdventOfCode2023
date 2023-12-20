use std::{ops::Range, collections::HashMap, char};

fn main() {
    let input: &'static str = include_str!("../input.txt");
    let map = parse_input(input);

    let characters: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| {
            line
            .chars()
            .collect()
        })
        .collect();

    let s_pos = input
        .trim()
        .lines()
        .enumerate()
        .filter(|(idx, line)| {
            line.contains("S")
        })
        .fold((0, 0), |acc, x| {(acc.0 + x.0, acc.1 + x.1.find('S').unwrap())});
    
    let mut direction: Direction = Direction::E;
    let mut position: Position = Position::new(s_pos.0, s_pos.1 - 1);
    let mut steps: usize = 1;

    let mut loop_: HashMap<Position, char> = HashMap::new();
    loop_.insert(position, characters[position.x][position.y]);

    while let Some((dir, pos)) = map.get(&(direction, position)) {
        direction = dir.clone();
        position = pos.clone();
        loop_.insert(position, characters[position.x][position.y]);
        steps += 1;
    }
    println!("{}", steps);

    let mut enclosed_tiles_amount: usize = 0;

    
    for x in 0..characters.len() {for y in 0..characters[0].len() {
        let mut count = 0;
        for new_y in 0..y {
            if loop_.get(&Position::new(x, new_y)).is_some() {
                count += 1;
            }
        }
        if count % 2 == 1 {
            enclosed_tiles_amount += 1;
        }
    }}

    println!("{}", enclosed_tiles_amount)
}

fn parse_input(input: &str) -> HashMap<(Direction, Position), (Direction, Position)> {
    type D = Direction;
    type P = Position;

    let input: Vec<&str> = input
        .trim()
        .lines()
        .collect();

    let mut output: HashMap<(Direction, Position), (Direction, Position)> = HashMap::new();

    let x_bound: Range<usize> = 0..input.len();
    let y_bound: Range<usize> = 0..input[0].len();

    for (x, line) in input.into_iter().enumerate() {
        for (y, chr) in line.chars().enumerate() {
            let pos = Position::new(x, y);
            match chr {
                '|' => {
                    output.insert((D::N, pos), (D::N, P::new(x+1, y)));
                    output.insert((D::S, pos), (D::S, P::new(x-1, y)));
                },
                '-' => {
                    output.insert((D::E, pos), (D::E, P::new(x, y-1)));
                    output.insert((D::W, pos), (D::W, P::new(x, y+1)));
                }
                'L' => {
                    output.insert((D::N, pos), (D::W, P::new(x, y+1)));
                    output.insert((D::E, pos), (D::S, P::new(x-1, y)));
                }
                'J' => {
                    output.insert((D::N, pos), (D::E, P::new(x, y-1)));
                    output.insert((D::W, pos), (D::S, P::new(x-1, y)));
                }
                '7' => {
                    output.insert((D::S, pos), (D::E, P::new(x, y-1)));
                    output.insert((D::W, pos), (D::N, P::new(x+1, y)));
                }
                'F' => {
                    output.insert((D::E, pos), (D::N, P::new(x+1, y)));
                    output.insert((D::S, pos), (D::W, P::new(x, y+1)));
                }
                _ => (),
            }
        }
    }
    output
        .into_iter()
        .filter(|(_, (_, pos))| {
            x_bound.contains(&pos.x) && y_bound.contains(&pos.y)
        })
        .collect::<HashMap<(Direction, Position), (Direction, Position)>>()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y:usize) -> Self {
        Self { x, y }
    }
}