use crate::Solution;
use cartesian::*;
use std::collections::VecDeque;
//use std::cmp::max;
pub fn solve(input: &str) -> Solution {
    #[derive(Copy, Clone, PartialEq)]
    enum Type {
        Water,
        Lava,
        Air,
    }
    use Type::{Air, Lava, Water};

    const M: usize = 24;
    let mut water = [[[Air; M]; M]; M];
    for line in input.lines() {
        let v: Vec<usize> = line.splitn(3, ',').map(|x| x.parse().unwrap()).collect();
        water[1 + v[0]][1 + v[1]][1 + v[2]] = Lava;
    }
    let mut sum1 = 0;
    let mut sum2 = 0;
    let directions: Vec<(i32, i32, i32)> = vec![
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    let mut todo: VecDeque<_> = VecDeque::new();
    for (s1, s2, s3) in cartesian!(0..M, 0..M, 0..M) {
        if water[s1][s2][s3] == Air {
            todo.push_back((s1, s2, s3));
            water[s1][s2][s3] = Water;
        }
        while let Some(c) = todo.pop_front() {
            for dir in &directions {
                let n = vec![
                    c.0 as i32 + dir.0,
                    c.1 as i32 + dir.1,
                    c.2 as i32 + dir.2,
                ];
                if n.iter().all(|x| 0 <= *x && *x < M as i32) {
                    let m: Vec<_> = n.iter().map(|x| *x as usize).collect();
                    let w = &mut water[m[0]][m[1]][m[2]];
                    match *w {
                        Air => {
                            todo.push_back((m[0], m[1], m[2]));
                            *w = Water;
                        }
                        Lava => sum1 += 1,
                        _ => (),
                    }
                }
            }
        }
        if sum2 == 0 {
            sum2 = sum1;
        }
    }

    Solution::Isize(sum1, sum2)
}
