use crate::Solution;

pub fn solve(input: &str) -> Solution {

    let mut elfs = input
        .split("\n\n")
        .map(|elf| elf.split('\n').map(|s| s.parse().unwrap_or(0)).sum())
        .collect::<Vec<isize>>();

    elfs.sort();

    let n = elfs.len();

    Solution::Isize(elfs[n - 1], elfs[n - 3..=n - 1].iter().sum())
}
