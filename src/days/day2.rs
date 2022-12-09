use crate::Solution;

pub fn solve(input: &str) -> Solution {

    fn eval1(s: &str) -> isize {
        ((s.chars().nth(2).unwrap() as isize - 'X' as isize - s.chars().nth(0).unwrap() as isize +
        'A' as isize + 4) % 3) * 3 + s.chars().nth(2).unwrap() as isize - 'X' as isize + 1
    }

    fn eval2(s: &str) -> isize {
        ((s.chars().nth(2).unwrap() as isize - 'X' as isize) % 3) * 3 + (s.chars().nth(2).unwrap()
        as isize - 'X' as isize + s.chars().nth(0).unwrap() as isize) % 3 + 1
    }

    let sum1: isize = input.lines().map(|s| eval1(s)).sum();
    let sum2: isize = input.lines().map(|s| eval2(s)).sum();

    Solution::Isize(sum1, sum2)
}
