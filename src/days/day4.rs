use crate::Solution;
pub fn solve(input: &str) -> Solution {
    let sum1 = input
        .lines()
        .map(|s| {
            s.split(['-', ','].as_ref())
                .map(|s| s.parse::<isize>().unwrap())
                .collect()
        })
        .map(|v: Vec<_>| (v[2] - v[0]) * (v[3] - v[1]))
        .filter(|t| t <= &0)
        .count();

    let sum2 = input
        .lines()
        .map(|s| {
            s.split(['-', ','].as_ref())
                .map(|s| s.parse::<isize>().unwrap())
                .collect()
        })
        .map(|v: Vec<_>| (v[2] - v[1]) * (v[3] - v[0]))
        .filter(|t| t <= &0)
        .count();

    Solution::Isize(sum1.try_into().unwrap(), sum2.try_into().unwrap())
}
