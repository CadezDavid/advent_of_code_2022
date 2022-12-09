mod days;
use days::{
    day1, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day2, day20, day21,
    day22, day23, day24, day25, day3, day4, day5, day6, day7, day8, day9,
};

use std::env;
use std::fmt;
use std::fs;

pub enum Solution {
    Isize(isize, isize),
    Str(String, String),
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Solution::Isize(part1, part2) => write!(f, "part1: {part1}, part2: {part2}"),
            Solution::Str(part1, part2) => write!(f, "part1: {part1}, part2: {part2}"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: isize = args[1]
        .trim()
        .parse()
        .expect("First argument should be a number.");

    let file_path = String::from("data/day") + &day.to_string() + ".in";
    let contents = fs::read_to_string(file_path).expect("Wheres the file??");

    let solution: Solution;

    match day {
        1 => solution = day1::solve(&contents),
        2 => solution = day2::solve(&contents),
        3 => solution = day3::solve(&contents),
        4 => solution = day4::solve(&contents),
        5 => solution = day5::solve(&contents),
        6 => solution = day6::solve(&contents),
        7 => solution = day7::solve(&contents),
        8 => solution = day8::solve(&contents),
        9 => solution = day9::solve(&contents),
        10 => solution = day10::solve(&contents),
        11 => solution = day11::solve(&contents),
        12 => solution = day12::solve(&contents),
        13 => solution = day13::solve(&contents),
        14 => solution = day14::solve(&contents),
        15 => solution = day15::solve(&contents),
        16 => solution = day16::solve(&contents),
        17 => solution = day17::solve(&contents),
        18 => solution = day18::solve(&contents),
        19 => solution = day19::solve(&contents),
        20 => solution = day20::solve(&contents),
        21 => solution = day21::solve(&contents),
        22 => solution = day22::solve(&contents),
        23 => solution = day23::solve(&contents),
        24 => solution = day24::solve(&contents),
        25 => solution = day25::solve(&contents),
        _ => unimplemented!(),
    }

    println!("Solutions for day {}:\n{}", day, solution)
}
