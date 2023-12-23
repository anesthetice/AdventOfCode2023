/// it's pretty clear I am just going to brute force this one ):<
/// too complicated for my smooth brain
use rayon::{iter::{self, ParallelIterator}, str::ParallelString};

fn main() {
    let input: &str = include_str!("../input.txt");

    let one: usize = input
        .par_lines()
        .map(|line| {
            let (input, checklist) = parse_line(line);
            enumerate(input)
                .into_iter()
                .filter(move |string| {
                    check(string, &checklist)
                })
                .count()
        })
        .fold(|| 0_usize, |acc, x| {acc + x})
        .sum::<usize>();

    println!("{}", one);
}

fn parse_line(input: &str) -> (&str, Vec<u8>) {
    let (l_input, r_input) = input.split_once(' ').unwrap();
    (
        l_input,
        r_input.split(',').map(|substr| {substr.parse::<u8>().unwrap()}).collect(),
    )
}

fn check(input: &str, checklist: &[u8]) -> bool {
    checklist == input
        .split('.')
        .filter(|substr| {substr.len() > 0})
        .map(|substr| {
            substr.len() as u8
        })
        .collect::<Vec<u8>>()
}

fn enumerate(input: &str) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    _enumerate_step(String::new(), input, &mut output);
    output
}

fn _enumerate_step(current: String, remaining: &str, output: &mut Vec<String>) {
    if let Some(subslice) = remaining.get(0..1) {
        if subslice == "?" {
            let mut alt_current_1: String = current.clone();
            alt_current_1.push('#');
            _enumerate_step(alt_current_1, &remaining[1..], output);

            let mut alt_current_2: String = current.clone();
            alt_current_2.push('.');
            _enumerate_step(alt_current_2, &remaining[1..], output);
        }
        else {
            let mut new_current: String = current.clone();
            new_current.push_str(subslice);
            _enumerate_step(new_current, &remaining[1..], output);
        }
    }
    else {
        output.push(current);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_line_test() {
        let input: &'static str = "???.### 1,1,3";
        let output: (&'static str, Vec<u8>) = ("???.###", vec![1,1,3]);
        assert_eq!(
            output,
            crate::parse_line(input),
        )
    }
    #[test]
    fn check_test_1() {
        let input: &'static str = "#.#.###";
        let checklist: &[u8] = &[1,1,3];
        assert!(crate::check(input, checklist))
    }
    #[test]
    fn check_test_2() {
        let input: &'static str = "..##.###";
        let checklist: &[u8] = &[1,1,3];
        assert!(!crate::check(input, checklist))
    }
    #[test]
    fn enumerate_test() {
        let input: &'static str = ".??.";
        assert_eq!(
            vec![
                ".##.".to_string(),
                ".#..".to_string(),
                "..#.".to_string(),
                "....".to_string(),
            ],
            crate::enumerate(input)
        )
    }
}