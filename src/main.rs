mod days;
mod etc;

use std::env;

use days::day01;
use etc::Solution;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Provide day(s) to run");
    }

    let days: Vec<u8> = args[1..]
        .iter()
        .map(|v| v.parse().expect("Expected a valid day"))
        .collect();

    for day in days {
        let solver = get_day_solver(day);

        let solution = solver();
        println!("\n=== Day {:02} ===", day);
        // println!("  · Part 1: {}", p1);
        // println!("  · Part 2: {}", p2);
        println!("  Solution {solution}");
    }
        
    println!("hello world!");
}

fn get_day_solver(day: u8) -> fn() -> Solution {
    match day {
        1 => day01::solve,
        _ => unimplemented!(),
    }
}
