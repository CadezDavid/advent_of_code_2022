use std::fs;

pub(crate) fn day4() -> (isize, isize) {
    let file_path = "data/day4.in";
    let contents = fs::read_to_string(file_path).expect("Wheres the file??");

    let sum1 = contents
        .lines()
        .map(|s| {
            s.split(['-', ','].as_ref())
                .map(|s| s.parse::<isize>().unwrap())
                .collect()
        })
        .map(|v: Vec<_>| (v[2] - v[0]) * (v[3] - v[1]))
        .filter(|t| t <= &0)
        .count();

    let sum2 = contents
        .lines()
        .map(|s| {
            s.split(['-', ','].as_ref())
                .map(|s| s.parse::<isize>().unwrap())
                .collect()
        })
        .map(|v: Vec<_>| (v[2] - v[1]) * (v[3] - v[0]))
        .filter(|t| t <= &0)
        .count();

    (sum1.try_into().unwrap(), sum2.try_into().unwrap())
}
