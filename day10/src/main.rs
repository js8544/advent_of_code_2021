use core::panic;
use itertools::Itertools;

fn main() {
    let scores: Vec<u64> = include_str!("input.txt")
        .lines()
        .map(str::as_bytes)
        .map(compute_score)
        .filter(|&score| score > 0)
        .sorted()
        .collect();
    let ans = scores[scores.len() / 2];
    println!("ans: {}", ans);
}

fn compute_score(line: &[u8]) -> u64 {
    let mut stack: Vec<&u8> = vec![];
    for c in line {
        if is_open(c) {
            stack.push(c);
        } else {
            if !is_match(stack.last().unwrap(), c) {
                return 0;
            } else {
                stack.pop();
            }
        }
    }
    stack.iter().rev().fold(0, |acc, &cur| {
        return acc * 5
            + match cur {
                b'(' => 1,
                b'[' => 2,
                b'{' => 3,
                b'<' => 4,
                _ => panic!("unexpected char {}", cur),
            };
    })
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
