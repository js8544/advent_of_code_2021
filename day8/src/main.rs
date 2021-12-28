struct Input {
    digits: Vec<&'static str>,
    outputs: Vec<&'static str>,
}

fn main() {
    let inputs: Vec<Input> = include_str!("input.txt").lines().map(parse_line).collect();
    let ans = inputs.iter().fold(0, |acc, input| {
        input.outputs.iter().fold(acc, |acc, &s| {
            if s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7 {
                acc + 1
            } else {
                acc
            }
        })
    });
    println!("ans: {}", ans);
}

fn parse_line(line: &'static str) -> Input {
    let mut line_split = line.split(" | ");
    let digits_str = line_split.next().unwrap();
    let outputs_str = line_split.next().unwrap();
    let digits = digits_str.split_whitespace().collect();
    let outputs = outputs_str.split_whitespace().collect();
    Input { digits, outputs }
}
