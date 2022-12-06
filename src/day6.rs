use std::cmp;
use std::collections::VecDeque;
use std::fs;

pub(crate) fn day6() -> (isize, isize) {
    let file_path = "data/day6.in";
    let contents = fs::read_to_string(file_path).expect("Wheres the file??");

    let mut len = 4;
    let (_, _, sum1) = contents
        .chars()
        .try_fold((VecDeque::new(), 0, 0), |(mut q, n, pos), c| {
            if pos > len - 1 {
                q.pop_back();
            };
            q.push_front(c);
            match (q.iter().skip(1).position(|&x| x == c), n, pos > len - 1) {
                (None, 1, true) => Err((q, n, pos + 1)),
                (Some(m), _, _) => Ok((q, cmp::max(len - m, n) - 1, pos + 1)),
                (None, _, _) => Ok((q, cmp::max(1, n) - 1, pos + 1)),
            }
        })
        .unwrap_or_else(|v| v);

    len = 14;
    let (_, _, sum2) = contents
        .chars()
        .try_fold((VecDeque::new(), 0, 0), |(mut q, n, pos), c| {
            if pos > len - 1 {
                q.pop_back();
            };
            q.push_front(c);
            match (q.iter().skip(1).position(|&x| x == c), n, pos > len - 1) {
                (None, 1, true) => Err((q, n, pos + 1)),
                (Some(m), _, _) => Ok((q, cmp::max(len - m, n) - 1, pos + 1)),
                (None, _, _) => Ok((q, cmp::max(1, n) - 1, pos + 1)),
            }
        })
        .unwrap_or_else(|v| v);

    (sum1.try_into().unwrap(), sum2.try_into().unwrap())
}
