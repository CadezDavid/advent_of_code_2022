use crate::Solution;
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashSet;
pub fn solve(input: &str) -> Solution {
    let area: HashSet<(isize, isize)> = HashSet::from_iter(
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

    let mut count1 = 0;
    let dir = [(1, 1), (-1, 1), (0, 1)].into_iter();
    let mut c = (500, 0);
    let mut todo = vec![c];
    let mut area1 = area.clone();
    while !todo.is_empty() && c.1 < floor {
        c = todo.pop().unwrap();
        todo.extend(
            dir.clone()
                .map(|(dx, dy)| (c.0 + dx, c.1 + dy))
                .filter(|p| !area1.contains(p)),
        );
        count1 += 1;
        area1.insert(c);
    }

    let mut count2 = 0;
    c = (500, 0);
    todo = vec![c];
    let mut area2 = area.clone();
    while !todo.is_empty() {
        c = todo.pop().unwrap();
        todo.extend(
            dir.clone()
                .map(|(dx, dy)| (c.0 + dx, c.1 + dy))
                .filter(|p| p.1 < floor && !area2.contains(p)),
        );
        count2 += 1;
        area2.insert(c);
    }
    Solution::Isize(count1 - floor - 1, count2)
}
