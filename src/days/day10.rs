use crate::Solution;
pub fn solve(input: &str) -> Solution {
    let cmds = input
        .lines()
        .flat_map(|line| match line.split_once(' ') {
            Some((_, val)) => vec![0, val.parse().unwrap()],
            None => vec![0],
        })
        .enumerate();

    let (sum1, _) = &cmds.clone().fold((0, 1), |(sum, x), (i, n)| {
        if (i + 21) % 40 == 0 && (i + 21) / 40 <= 6 {
            (sum + (i as isize + 1) * x, x + n)
        } else {
            (sum, x + n)
        }
    });

    let _ = &cmds.clone().fold(1, |sprite, (i, n)| {
        if (sprite - (i as isize) % 40).abs() <= 1 {
            print!("#");
        } else {
            print!(".");
        }
        if (i + 1) % 40 == 0 {
            print!("\n");
        }
        sprite + n
    });

    Solution::Isize(*sum1, 0)
}
