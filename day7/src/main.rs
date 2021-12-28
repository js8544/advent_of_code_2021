#![feature(int_abs_diff)]
fn main() {
    let mut nums: Vec<u64> = include_str!("input.txt")
        .trim()
        .split(",")
        .map(|str| str.parse().unwrap())
        .collect();

    nums.sort();
    let mid = nums[nums.len() / 2];

    let ans = nums
        .iter()
        .fold(0, |acc, &num| acc + u64::abs_diff(num, mid));
    println!("ans: {}", ans);
}
