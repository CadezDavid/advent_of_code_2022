use crate::Solution;
pub fn solve(input: &str) -> Solution {
    let forest: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .collect()
        })
        .collect();
    let mut sum = 2 * (forest.len() + forest[0].len() - 2);
    for x in 1..(forest.len() - 1) {
        for y in 1..(forest[0].len() - 1) {
            let m1 = forest[x][..y].iter().max().unwrap();
            let m2 = forest[x][y + 1..].iter().max().unwrap();
            let m3 = (0..x).map(|i| forest[i][y]).max().unwrap();
            let m4 = (x + 1..forest.len()).map(|i| forest[i][y]).max().unwrap();
            if forest[x][y] > *m1 || forest[x][y] > *m2 || forest[x][y] > m3 || forest[x][y] > m4 {
                sum += 1;
            }
        }
    }

    let mut max = 0;
    for x in 1..(forest.len() - 1) {
        for y in 1..(forest[0].len() - 1) {
            let tree = forest[x][y];
            let m1 = forest[x][..y]
                .iter()
                .rev()
                .try_fold(0, |n, t| if tree <= *t { Err(n + 1) } else { Ok(n + 1) })
                .unwrap_or_else(|x| x);
            let m2 = forest[x][y + 1..]
                .iter()
                .try_fold(0, |n, t| if tree <= *t { Err(n + 1) } else { Ok(n + 1) })
                .unwrap_or_else(|x| x);
            let m3 = (0..x)
                .rev()
                .map(|i| forest[i][y])
                .try_fold(0, |n, t| {
                    if tree <= t as isize {
                        Err(n + 1)
                    } else {
                        Ok(n + 1)
                    }
                })
                .unwrap_or_else(|x| x);
            let m4 = (x + 1..forest.len())
                .map(|i| forest[i][y])
                .try_fold(0, |n, t| {
                    if tree <= t as isize {
                        Err(n + 1)
                    } else {
                        Ok(n + 1)
                    }
                })
                .unwrap_or_else(|x| x);
            max = max.max(m1 * m2 * m3 * m4);
        }
    }
    Solution::Isize(sum as isize, max)
}
