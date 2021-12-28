#![feature(int_abs_diff)]
fn main() {
    let nums: Vec<u64> = include_str!("input.txt")
        .trim()
        .split(",")
        .map(|str| str.parse().unwrap())
        .collect();

    let min = nums.iter().min().unwrap().to_owned();
    let max = nums.iter().max().unwrap().to_owned();

    let ans = (min..max)
        .map(|mid| {
            nums.iter().fold(0, |acc, &num| {
                let diff = u64::abs_diff(num, mid);
                acc + diff * (diff + 1) / 2
            })
        })
        .min()
        .unwrap();

    println!("ans: {}", ans);
}
