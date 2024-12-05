mod days;
mod etc;

use std::env;

use days::day01;
use etc::Solution;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Provide 0/1 for test input and day(s) to run");
    }

    let run_test: u8 = args[1].parse().expect("Expected 0/1 for test option");

    let days: Vec<u8> = args[2..]
        .iter()
        .map(|v| v.parse().expect("Expected a valid day"))
        .collect();

    for day in days {
        let solver = get_day_solver(day);

        let solution = solver(run_test == 1);
        println!("\n=== Day {:02} ===", day);
        // println!("  · Part 1: {}", p1);
        // println!("  · Part 2: {}", p2);
        println!("  Solution {solution}");
    }
}

fn get_day_solver(day: u8) -> fn(test: bool) -> Solution {
    match day {
        1 => day01::solve,
        _ => unimplemented!(),
    }
}
