struct Input {
    digits: Vec<u8>,
    outputs: Vec<u8>,
}

fn main() {
    let inputs: Vec<Input> = include_str!("input.txt").lines().map(parse_line).collect();
    let outputs: Vec<u64> = inputs.iter().map(process).collect();
    println!("ans: {}", outputs.iter().sum::<u64>());
}

fn parse_line(line: &str) -> Input {
    let mut line_split = line.split(" | ");
    let digits_str = line_split.next().unwrap();
    let outputs_str = line_split.next().unwrap();
    let digits = digits_str.split_whitespace().map(parse_bitmap).collect();
    let outputs = outputs_str.split_whitespace().map(parse_bitmap).collect();
    Input { digits, outputs }
}

fn parse_bitmap(s: &str) -> u8 {
    s.as_bytes().iter().fold(0, |acc, &c| acc | 1 << c - b'a')
}

fn process(input: &Input) -> u64 {
    let mut representation_map = [0; 10];
    // find easy ones first
    for &representation in &input.digits {
        match representation.count_ones() {
            2 => representation_map[1] = representation,
            3 => representation_map[7] = representation,
            4 => representation_map[4] = representation,
            7 => representation_map[8] = representation,
            _ => (),
        }
    }
    for &representation in &input.digits {
        let and_four = (representation & representation_map[4]).count_ones();
        let and_seven = (representation & representation_map[7]).count_ones();
        let tot = representation.count_ones();
        match (and_four, and_seven, tot) {
            (3, 3, 6) => representation_map[0] = representation,
            (2, 2, 5) => representation_map[2] = representation,
            (3, 3, 5) => representation_map[3] = representation,
            (3, 2, 5) => representation_map[5] = representation,
            (3, 2, 6) => representation_map[6] = representation,
            (4, 3, 6) => representation_map[9] = representation,
            _ => (),
        }
    }

    input.outputs.iter().fold(0, |acc, &num| {
        for i in 0..10 {
            if representation_map[i] == num {
                return acc * 10 + i as u64;
            }
        }
        panic!(
            "representation {} not found, all representations: {:?}",
            num, representation_map
        );
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bitmap() {
        assert_eq!(parse_bitmap("abc"), 7);
        assert_eq!(parse_bitmap("agb"), 67);
        assert_eq!(parse_bitmap("cf"), 36);
    }
}
