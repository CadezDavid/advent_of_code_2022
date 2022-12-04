mod day1;
mod day2;
mod day3;
mod day4;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = args[1]
        .trim()
        .parse()
        .expect("First argument should be a number.");

    let results: (isize, isize);

    match day {
        1 => {
            results = day1::day1();
        }
        2 => {
            results = day2::day2();
        }
        3 => {
            results = day3::day3();
        }
        4 => {
            results = day4::day4();
        }
        _ => {
            println!("Not solved yet.");
            std::process::exit(0)
        }
    }

    println!(
        "Results for day {}:\nPart 1: {}\nPart 2: {}",
        args[1], results.0, results.1
    )
}
