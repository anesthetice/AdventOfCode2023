use rayon::{iter::{self, ParallelIterator, IntoParallelIterator}, str::ParallelString};

fn main() {
    let input: &'static str = include_str!("../input.txt");

    
    let answer_1: usize = input
        .par_lines()
        .map(|line| {
            let (input, checklist) = parse_line_part_1(line);
            count(&input, &checklist)
        })
        .fold(|| 0_usize, |acc, x| {acc+x})
        .sum::<usize>();

    println!("{}", answer_1);

    let answer_2: usize = input
        .par_lines()
        .map(|line| {
            let (input, checklist) = parse_line_part_2(line);
            count(&input, &checklist)
        })
        .fold(|| 0_usize, |acc, x| {acc+x})
        .sum::<usize>();
    
    println!("{}", answer_2);

    /*
    let input: &str = "?#.??????#??#?#?#?#? 1,1,15";
    let (input, checklist) = parse_line_part_1(input);
    println!(
        "{}",
        count(&input, &checklist)
    )  
    */

}

fn count(input: &str, checklist: &[usize]) -> usize {
    if input.len() == 0 {
        if checklist.len() == 0 {return 1}
        else {return 0}
    }

    if checklist.len() == 0 {
        if input.contains('#') {return 0}
        else {return 1}
    }

    let mut result: usize = 0;

    if ".?".contains(&input[0..1]) {
        result += count(&input[1..], &checklist[..]);
    }

    if "#?".contains(&input[0..1]) {
        // fist check that we even have the required length for a block of broken springs
        if checklist[0] <= input.len()
        // makes sure there aren't any working springs inside our block
        && !input[0..checklist[0]].contains('.') 
        // asserts that we've either reached the end of our input or that there isn't another broken spring at the end of our block as that would invalidate the block
        && (input.len() == checklist[0] || &input[checklist[0]..checklist[0]+1] != "#")
        {
            result += count(&input[checklist[0]..], &checklist[1..])
        }
    }
    result
}

fn parse_line_part_1(input: &str) -> (&str, Vec<usize>) {
    let (l_input, r_input) = input.split_once(' ').unwrap();
    (
        l_input,
        r_input.split(',').map(|substr| {substr.parse::<usize>().unwrap()}).collect(),
    )
}

fn parse_line_part_2(input: &str) -> (String, Vec<usize>) {
    let (l_input, r_input) = input.split_once(' ').unwrap();
    let mut r_input: Vec<usize> = r_input.split(',').map(|substr| {substr.parse::<usize>().unwrap()}).collect();
    let size = r_input.len();
    r_input.extend_from_within(0..);
    r_input.extend_from_within(0..);
    r_input.extend_from_within(0..size);
    (
        format!("{}?{}?{}?{}?{}", l_input, l_input, l_input, l_input, l_input),
        r_input,
    )
}


#[cfg(test)]
mod tests {
    #[test]
    fn parse_line_part_1_test() {
        let input: &'static str = "???.### 1,1,3";
        let output: (&'static str, Vec<usize>) = ("???.###", vec![1,1,3]);
        assert_eq!(
            output,
            crate::parse_line_part_1(input),
        )
    }
    #[test]
    fn parse_line_part_2_test() {
        let input: &'static str = ".# 1";
        let output: (String, Vec<usize>) = (".#?.#?.#?.#?.#".to_string(), vec![1,1,1,1,1]);
        assert_eq!(
            output,
            crate::parse_line_part_2(input),
        )
    }
}