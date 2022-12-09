use crate::Solution;

pub fn solve(input: &str) -> Solution {

    fn value(c: &char) -> isize {
        if (*c as isize) < ('a' as isize) {
            *c as isize - 'A' as isize + 27
        } else {
            *c as isize - 'a' as isize + 1
        }
    }

    let sum1 = input
        .lines()
        .map(|s| {
            let (a, b) = s.split_at(s.len() / 3);
            value(&(*a).chars().find(|c| b.contains(*c)).unwrap())
        })
        .sum();

    let lines: Vec<&str> = input.lines().collect();
    let mut n = 0;
    let mut groups = Vec::new();
    while (n + 2) < lines.len() {
        groups.push((lines[n], lines[n + 1], lines[n + 2]));
        n += 3;
    }

    let sum2 = groups
        .iter()
        .map(|s| {
            let (a, b, c) = s;
            value(
                &(*a)
                    .chars()
                    .find(|d| b.contains(*d) && c.contains(*d))
                    .unwrap(),
            )
        })
        .sum();

    Solution::Isize(sum1, sum2)
}
