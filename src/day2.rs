use std::fs;

pub(crate) fn day2() -> (isize, isize) {
    let file_path = "data/day2.in";
    let contents = fs::read_to_string(file_path).expect("Wheres the file??");

    fn eval1(s: &str) -> isize {
        ((s.chars().nth(2).unwrap() as isize - 'X' as isize - s.chars().nth(0).unwrap() as isize +
        'A' as isize + 4) % 3) * 3 + s.chars().nth(2).unwrap() as isize - 'X' as isize + 1
    }

    fn eval2(s: &str) -> isize {
        ((s.chars().nth(2).unwrap() as isize - 'X' as isize) % 3) * 3 + (s.chars().nth(2).unwrap()
        as isize - 'X' as isize + s.chars().nth(0).unwrap() as isize) % 3 + 1
    }

    let sum1: isize = contents.lines().map(|s| eval1(s)).sum();
    let sum2: isize = contents.lines().map(|s| eval2(s)).sum();

    (sum1, sum2)
}
