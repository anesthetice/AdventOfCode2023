fn main() {
    let input: &str = include_str!("../input.txt");
    let input: String = process_image(input);

    let galaxies = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(row_idx, line)| {
            line
                .char_indices()
                .flat_map(|(col_idx, chr)| {
                    if chr == '#' {
                        Some(Vector(row_idx, col_idx))
                    }
                    else {None}
                })
                .collect::<Vec<Vector>>()
        })
        .collect::<Vec<Vector>>();

    let mut sum: usize = 0;

    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            sum += galaxies[i].dist(&galaxies[j]);
        }
    }

    println!("{}", sum);
}


fn process_image(input: &str) -> String {
    let mut input: Vec<Vec<char>> = input
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

    for (x, row_index) in row_indexes_to_expand.into_iter().enumerate() {
        input.insert(row_index+x, vec!['.'; row_length])
    }
    let col_length: usize = input.len();

    for (x, col_index) in col_indexes_to_expand.into_iter().enumerate() {
        for row_index in 0..col_length {
            input[row_index].insert(col_index+x, '.')
        }
    }  

    input
        .into_iter()
        .map(|line| {
            format!("{}\n", line.into_iter().collect::<String>())
        })
        .collect::<String>()
}

/// represents a vector belonging to N²
#[derive(Debug)]
pub struct Vector(usize, usize);

impl Vector {
    pub fn dist(&self, other: &Self) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    #[test]
    pub fn process_image_test() {
        let input: &'static str = indoc! {
            "...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#....."};

        let output: &'static str = indoc! {
            "....#........
            .........#...
            #............
            .............
            .............
            ........#....
            .#...........
            ............#
            .............
            .............
            .........#...
            #....#.......
            "
        };

        assert_eq!(
            output,
            crate::process_image(input).as_str()
        )
    }
}