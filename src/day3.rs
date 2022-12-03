//use std::cmp;
use std::fs;

pub(crate) fn day3() -> (isize, isize) {
    let file_path = "data/day3.in";
    let contents = fs::read_to_string(file_path).expect("Wheres the file??");

    fn value(c: &char) -> isize {
        if (*c as isize) < ('a' as isize) {
            *c as isize - 'A' as isize + 27
        } else {
            *c as isize - 'a' as isize + 1
        }
    }

    let sum1 = contents
        .lines()
        .map(|s| {
            let (a, b) = s.split_at(s.len() / 2);
            value(&(*a).chars().find(|c| b.contains(*c)).unwrap())
        })
        .sum();

    let lines: Vec<&str> = contents.lines().collect();
    let mut n = 0;
    let mut groups = Vec::new();
    while (n + 2) < lines.len() {
        groups.push((lines[n], lines[n+1], lines[n+2]));
        n += 3;
    }

    let sum2 = groups
        .iter()
        .map(|s| {
            let (a, b, c) = s;
            value(&(*a).chars().find(|d| b.contains(*d) && c.contains(*d)).unwrap())
        })
        .sum();

    (sum1, sum2)
}
