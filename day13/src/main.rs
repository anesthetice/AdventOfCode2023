fn main() {
    let input: &'static str = include_str!("../input.txt");
    let input = parse_input(input);
    
    let answer = input
        .into_iter()
        .map(|block| {
            let block_t = transpose(block.as_slice());
            op(block.as_slice()) * 100 + op(block_t.as_slice())
        })
        .fold(0_usize,  |acc, x| {acc+x});
    println!("{}", answer);
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
    for string in input.iter() {
    for (j, chr) in string.char_indices() {
        output[j].push(chr)
    }
    }
    output
}

fn op(input: &[String]) -> usize {
    let idx_step = |curr_idxs: Option<(usize, usize)>, init_idx: usize, length: usize| -> Option<(usize, usize)> {
        match curr_idxs {
            Some((a, b)) => {
                if a > 0 && b < length-1 {
                    Some((a-1, b+1))
                }
                else {
                    None
                }
            },
            None => {Some((init_idx-1, init_idx))}
        }
    };
    let mut sum: usize = 0;
    for idx in 1..input.len() {
        let mut valid: bool = true;
        let mut curr_idxs: Option<(usize, usize)> = None;
        while let Some((a, b)) = idx_step(curr_idxs, idx, input.len()) {
            if input[a] != input[b] {valid=false; break;}
            curr_idxs = Some((a, b));
        }
        if valid {sum+=idx}
    }
    sum
}
