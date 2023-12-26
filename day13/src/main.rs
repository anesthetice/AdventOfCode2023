
fn main() {
    let a: Vec<String> = vec!["123".to_string(),"456".to_string(),"789".to_string()];
    let b = transpose(a.as_slice());

    println!("{:#?}\n\n{:#?}", a, b);
}

fn parse_input(input: &str) -> Vec<Vec<String>>{
    input
        .split("\n\n")
        .map(|input| {
            input.lines().map(|line| line.to_string()).collect()
        })
        .collect::<Vec<Vec<String>>>()
}

fn transpose(input: &[String]) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for _ in 0..input[0].len() {
        output.push(String::new());
    }

    for (i, string) in input.iter().enumerate() {
    for (j, chr) in string.char_indices() {
        output[j].push(chr)
    }
    }

    output
}