fn main() {
    let lines: Vec<&str> = include_str!("input.txt").lines().collect();
    part1(&lines);
}

fn part1(lines: &Vec<&str>) {
    let cnts = count_lines(&lines);
    println!("cnts: {:?}", cnts);

    let mut gamma_str = String::new();
    let mut epsilon_str = String::new();

    cnts.iter().for_each(|cnt| {
        if cnt[0] > cnt[1] {
            gamma_str.push('0');
            epsilon_str.push('1');
        } else {
            gamma_str.push('1');
            epsilon_str.push('0');
        }
    });
    let gamma = usize::from_str_radix(gamma_str.as_str(), 2).unwrap();
    let epsilon = usize::from_str_radix(epsilon_str.as_str(), 2).unwrap();
    println!(
        "gamma binary: {}, gamma: {}, epsilon binary: {}, epsilon: {}, product: {}",
        gamma_str,
        gamma,
        epsilon_str,
        epsilon,
        gamma * epsilon
    );
}

fn count_lines(lines: &Vec<&str>) -> Vec<[usize; 2]> {
    let line_len = lines.first().unwrap().len();
    lines.iter().for_each(|&line| {
        assert_eq!(line.len(), line_len);
    });
    let mut cnts: Vec<[usize; 2]> = vec![];
    cnts.resize(line_len, [0, 0]);
    lines.iter().for_each(|&line| {
        line.as_bytes().iter().enumerate().for_each(|(i, c)| {
            match c {
                b'0' => cnts[i][0] += 1,
                b'1' => cnts[i][1] += 1,
                _ => panic!("unexpected token {}", c),
            };
        });
    });
    cnts
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_lines() {
        let lines = vec!["11001", "00110", "11011", "10101"];
        let cnts = count_lines(&lines);
        assert_eq!(cnts.len(), 5);
        assert_eq!(cnts[0], [1, 3]);
        assert_eq!(cnts[1], [2, 2]);
        assert_eq!(cnts[2], [2, 2]);
        assert_eq!(cnts[3], [2, 2]);
        assert_eq!(cnts[4], [1, 3]);
    }
}
