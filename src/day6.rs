use std::cmp::max;
use std::fs;

pub(crate) fn day6() -> (isize, isize) {
    let file_path = "data/day6.in";
    let contents = fs::read_to_string(file_path).expect("Wheres the file??");

    let mut l = 4;
    let (_, sum1) = contents
        .chars()
        .try_fold((l, 0), |(n, pos), c| {
            match (contents[max(l, pos) - l..pos].chars().rev().position(|x| x == c), n) {
                (Some(m), n) => Ok((max(l, n + m - 1) - m, pos + 1)),
                (None, 1) => Err((0, pos)),
                (None, n) => Ok((max(1, n) - 1, pos + 1)),
            }
        })
        .unwrap_or_else(|x| x);

    l = 14;
    let (_, sum2) = contents
        .chars()
        .try_fold((l, 0), |(n, pos), c| {
            match (
                contents[max(l, pos) - l..pos].chars().rev().position(|x| x == c), n) {
                (Some(m), n) => Ok((max(l, n + m - 1) - m, pos + 1)),
                (None, 1) => Err((0, pos)),
                (None, n) => Ok((max(1, n) - 1, pos + 1)),
            }
        })
        .unwrap_or_else(|x| x);

    (sum1.try_into().unwrap(), sum2.try_into().unwrap())
}
