use std::fs;

pub(crate) fn day1() -> (isize, isize) {
    let file_path = "data/day1.in";
    let contents = fs::read_to_string(file_path).expect("Wheres the file??");

    let mut elfs = contents
        .split("\n\n")
        .map(|elf| elf.split('\n').map(|s| s.parse().unwrap_or(0)).sum())
        .collect::<Vec<isize>>();

    elfs.sort();

    let n = elfs.len();

    (elfs[n - 1], elfs[n - 3..=n - 1].iter().sum())
}
