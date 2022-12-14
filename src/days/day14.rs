use crate::Solution;
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashSet;
pub fn solve(input: &str) -> Solution {
    let mut area: HashSet<(isize, isize)> = HashSet::from_iter(
        input
            .lines()
            .flat_map(|line| {
                line.split(" -> ")
                    .map(|x| x.split_once(',').unwrap())
                    .map(|(x, y)| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()))
                    .tuple_windows()
            })
            .flat_map(|((a1, a2), (b1, b2))| {
                (min(a1, b1)..=max(a1, b1)).cartesian_product(min(a2, b2)..=max(a2, b2))
            }),
    );

    let floor = area.iter().map(|(_, y)| y).max().unwrap() + 2;

    let mut count = 0;
    let dir = [(1, 1), (-1, 1), (0, 1)].into_iter();
    let mut c = (500, 0);
    let mut todo = vec![c];
    while !todo.is_empty() && c.1 < floor - 2 {
        c = todo.pop().unwrap();
        todo.extend(
            dir.clone()
                .map(|(dx, dy)| (c.0 + dx, c.1 + dy))
                .filter(|p| !area.contains(p)),
        );
        count += 1;
        area.insert(c);
    }
    let sum1 = count - floor + 1;
    while let Some(c) = todo.pop() {
        todo.extend(
            dir.clone()
                .map(|(dx, dy)| (c.0 + dx, c.1 + dy))
                .filter(|p| p.1 < floor && !area.contains(p)),
        );
        count += 1;
        area.insert(c);
    }
    Solution::Isize(sum1, count)
}
