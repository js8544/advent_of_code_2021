fn main() {
    let arr: Vec<usize> = include_str!("input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let ans = arr
        .windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|&window| window[0] < window[1])
        .count();
    println!("ans: {}", ans);
}
