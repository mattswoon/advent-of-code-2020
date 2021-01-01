#![feature(str_split_once, result_flattening)]

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

use std::{
    env,
    path::PathBuf
};

fn print_result<T: std::fmt::Display, E: std::fmt::Display>(result: Result<T, E>) {
    match result {
        Ok(x) => println!("\t{}", x),
        Err(e) => println!("\tERROR: {}", e)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Where's the data directory arg?!?!")
    }
    let data_dir: PathBuf = (&args[1]).into();

    println!("Well hello there, let's do the advent of code");
    println!("Day 1 ...");
    print_result(day1::run(&data_dir));

    println!("Day 1, part 2...");
    print_result(day1::run_part2(&data_dir));

    println!("Day 2 ...");
    print_result(day2::run(&data_dir));
    
    println!("Day 2, part 2 ...");
    print_result(day2::run_part2(&data_dir));
    
    println!("Day 3 ...");
    print_result(day3::run(&data_dir));

    println!("Day 3, part 2 ...");
    print_result(day3::run_part2(&data_dir));
    
    println!("Day 4 ...");
    print_result(day4::run(&data_dir));
}
