use crate::Solution;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Solution {
    fn aux(len: usize, input: &str) -> isize {
        let mut rope = vec![(0, 0); len];
        let mut visited = HashSet::new();
        let directions =
            HashMap::from([('D', (-1, 0)), ('U', (1, 0)), ('L', (0, -1)), ('R', (0, 1))]);
        for cmd in input.lines() {
            let dir = directions.get(&cmd.chars().nth(0).unwrap()).unwrap();
            let count = cmd[2..].parse().unwrap();
            for _ in 0..count {
                rope[0].0 += dir.0;
                rope[0].1 += dir.1;
                for i in 1..rope.len() {
                    let diff: (isize, isize) =
                        (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1);
                    if diff.0.abs() > 1 || diff.1.abs() > 1 {
                        rope[i] = (rope[i].0 + diff.0.signum(), rope[i].1 + diff.1.signum());
                    }
                }
                visited.insert(rope[rope.len() - 1]);
            }
        }
        visited.len() as isize
    }
    Solution::Isize(aux(2, input), aux(10, input))
}
