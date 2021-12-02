fn main() {
    let ops: Vec<Op> = include_str!("input.txt").lines().map(parse_line).collect();
    let pos = run(&ops);
    println!(
        "horizontal: {}, depth: {}, ans: {}",
        pos.horizontal,
        pos.depth,
        pos.horizontal * pos.depth
    )
}

struct Pos {
    horizontal: usize,
    depth: usize,
    aim: usize,
}

enum Op {
    Forward(usize),
    Up(usize),
    Down(usize),
}

fn parse_line(line: &str) -> Op {
    let mut split = line.split(" ");
    match split.next().unwrap() {
        "forward" => Op::Forward(split.next().unwrap().parse().unwrap()),
        "up" => Op::Up(split.next().unwrap().parse().unwrap()),
        "down" => Op::Down(split.next().unwrap().parse().unwrap()),
        str => panic!("unexpected token: {}", str),
    }
}

fn run(ops: &Vec<Op>) -> Pos {
    ops.iter().fold(
        Pos {
            horizontal: 0,
            depth: 0,
            aim: 0,
        },
        |Pos {
             horizontal,
             depth,
             aim,
         },
         op| match op {
            Op::Forward(n) => Pos {
                horizontal: horizontal + n,
                depth: depth + aim * n,
                aim: aim,
            },
            Op::Up(n) => Pos {
                horizontal: horizontal,
                depth: depth,
                aim: aim - n,
            },
            Op::Down(n) => Pos {
                horizontal: horizontal,
                depth: depth,
                aim: aim + n,
            },
        },
    )
}
