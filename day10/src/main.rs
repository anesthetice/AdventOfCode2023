use std::{ops::Range, collections::HashMap};
fn main() {
    let input: &'static str = include_str!("../input.txt");

    let map = parse_input(input);

    let S_pos = input
        .trim()
        .lines()
        .enumerate()
        .filter(|(idx, line)| {
            line.contains("S")
        })
        .fold((0, 0), |acc, x| {(acc.0 + x.0, acc.1 + x.1.find('S').unwrap())});
    
    let S_pos_1 = Position::new(S_pos.0, S_pos.1-1);
    
    let mut direction: Direction = Direction::E;
    let mut position: Position = Position::new(S_pos.0, S_pos.1 - 1);
    let mut trace: String = String::new();
    let mut steps: usize = 1;

    while let Some((dir, pos)) = map.get(&(direction, position)) {
        direction = dir.clone();
        position = pos.clone();
        trace = format!("{}{:?}\n", trace, position);
        steps += 1;
    }

    println!("{}", steps);

    use std::io::Write;
    let mut f = std::fs::OpenOptions::new().create(true).write(true).truncate(true).open("stdout.txt").unwrap().write_all(trace.as_bytes()).unwrap();

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


/*
struct Runner {
    position: (usize, usize),
    in_direction: Direction,
    out_direction: Direction,
    steps: usize
}

impl Runner {
    // assume a runner is created from the position of a 'S' character
    pub fn new(position: (usize, usize), out_direction: Direction) -> Self {
        Self { position, in_direction: out_direction.opposite(), out_direction, steps: 0 }
    }
    pub fn step(mut self, grid: &[&str], x_bound: &Range<usize>, y_bound: &Range<usize>) -> Result<Self, usize> {
        self.out_direction.step_position(&mut self.position);
        if !x_bound.contains(&self.position.0) || !y_bound.contains(&self.position.1) {
            return Err(self.steps);
        }

        match grid[self.position.0].chars().nth(self.position.1).unwrap() {
            'S' => return Err(self.steps)
        }

        Ok(self)
    }
}



impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East
        }
    }
    fn step_position(&self, position: &mut (usize, usize)) {
        match self {
            Direction::North => position.0 -= 1,
            Direction::East => position.1 += 1,
            Direction::South => position.0 += 1,
            Direction::West => position.1 -= 1,
        }
    }
}
*/