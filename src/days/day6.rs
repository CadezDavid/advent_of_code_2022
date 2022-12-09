use crate::Solution;
pub fn solve(input: &str) -> Solution {

    fn aux(contents: &str, l: usize) -> isize {
        contents
            .chars()
            .enumerate()
            .try_fold(l + 1, |n, (i, c)| {
                match (contents[i + 1..i + n].chars().position(|x| x == c), n) {
                    (Some(_), _) => Ok(l + 1),
                    (None, 1) => Err(i),
                    (None, n) => Ok(n - 1),
                }
            })
            .unwrap_or_else(|x| x)
            .try_into()
            .unwrap()
    }
    Solution::Isize(aux(&input, 4), aux(&input, 14))
}
