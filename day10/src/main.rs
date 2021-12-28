fn main() {
    let ans: u64 = include_str!("input.txt")
        .lines()
        .map(str::as_bytes)
        .map(compute_score)
        .sum();
    println!("ans: {}", ans);
}

fn compute_score(line: &[u8]) -> u64 {
    let mut stack: Vec<&u8> = vec![];
    for c in line {
        if is_open(c) {
            stack.push(c);
        } else {
            if !is_match(stack.last().unwrap(), c) {
                return match &c {
                    b')' => 3,
                    b']' => 57,
                    b'>' => 25137,
                    b'}' => 1197,
                    _ => panic!(),
                };
            } else {
                stack.pop();
            }
        }
    }
    0
}

fn is_open(c: &u8) -> bool {
    match c {
        b'(' | b'[' | b'<' | b'{' => true,
        _ => false,
    }
}

fn is_match(a: &u8, b: &u8) -> bool {
    match (a, b) {
        (b'(', b')') | (b'[', b']') | (b'<', b'>') | (b'{', b'}') => true,
        _ => false,
    }
}
