use std::fs;

pub(crate) fn day4() -> (isize, isize) {
    let file_path = "data/day4.in";
    let contents = fs::read_to_string(file_path).expect("Wheres the file??");

    let sum1 = contents
        .lines()
        .map(|s| s.split_once(',').unwrap())
        .map(|(a, b)| (a.split_once('-').unwrap(), b.split_once('-').unwrap()))
        .map(|((a1, a2), (b1, b2))| {
            (b1.parse::<isize>().unwrap() - a1.parse::<isize>().unwrap())
                * (b2.parse::<isize>().unwrap() - a2.parse::<isize>().unwrap())
        })
        .filter(|t| t <= &0)
        .count();

    let sum2 = contents
        .lines()
        .map(|s| s.split_once(',').unwrap())
        .map(|(a, b)| (a.split_once('-').unwrap(), b.split_once('-').unwrap()))
        .map(|((a1, a2), (b1, b2))| {
            (b1.parse::<isize>().unwrap() - a2.parse::<isize>().unwrap())
                * (b2.parse::<isize>().unwrap() - a1.parse::<isize>().unwrap())
        })
        .filter(|t| t <= &0)
        .count();

    (sum1.try_into().unwrap(), sum2.try_into().unwrap())
}
