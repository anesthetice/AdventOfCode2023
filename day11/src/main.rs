fn main() {
    let input: &str = include_str!("../input.txt");

    let galaxies_1: Vec<Vector> = process_image(input, 1);
    let mut sum: usize = 0;
    for i in 0..galaxies_1.len() {
        for j in i+1..galaxies_1.len() {
            sum += galaxies_1[i].dist(&galaxies_1[j]);
        }
    }
    println!("{}", sum);

    let galaxies_2: Vec<Vector> = process_image(input, 999999);
    let mut sum: usize = 0;
    for i in 0..galaxies_2.len() {
        for j in i+1..galaxies_2.len() {
            sum += galaxies_2[i].dist(&galaxies_2[j]);
        }
    }
    println!("{}", sum);
}

fn process_image(input: &str, expansion: usize) -> Vec<Vector> {
    let input: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect();
    let row_length: usize = input[0].len();
    let col_length: usize = input.len();

    let row_indexes_to_expand: Vec<usize> = (0..col_length)
        .into_iter()
        .flat_map(|row_index| {
            if input[row_index].iter().filter(|chr| **chr=='.').count() == row_length {
                Some(row_index)
            }
            else {None}
        })
        .collect();

    let col_indexes_to_expand: Vec<usize> = (0..row_length)
        .into_iter()
        .flat_map(|col_index| {
            let mut sum: usize = 0;
            for row_index in 0..col_length {
                if input[row_index][col_index] == '.' {
                    sum += 1;
                }
            }
            if sum == col_length {
                Some(col_index)
            }
            else {None}
        })
        .collect();

    let galaxies = input
        .into_iter()
        .enumerate()
        .flat_map(|(row_idx, line)| {
            line
                .into_iter()
                .enumerate()
                .flat_map(|(col_idx, chr)| {
                    if chr == '#' {
                        Some(Vector(row_idx, col_idx))
                    }
                    else {None}
                })
                .collect::<Vec<Vector>>()
        })
        .collect::<Vec<Vector>>();

    galaxies
        .into_iter()
        .map(|mut vect| {
            let vect_ = vect.clone();
            for row_idx in row_indexes_to_expand.iter() {
                if *row_idx < vect_.0 {
                    vect.0 += expansion
                }
            }
            for col_idx in col_indexes_to_expand.iter() {
                if *col_idx < vect_.1 {
                    vect.1 += expansion;
                }
            }
            vect
        })
        .collect::<Vec<Vector>>()
}

/// represents a vector belonging to N²
#[derive(Debug, Clone)]
pub struct Vector(usize, usize);

impl Vector {
    pub fn dist(&self, other: &Self) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}
