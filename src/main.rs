#![feature(str_split_once, result_flattening)]

pub mod day1;
pub mod day2;

use std::{
    env,
    path::PathBuf
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Where's the data directory arg?!?!")
    }
    let data_dir: PathBuf = (&args[1]).into();

    println!("Well hello there, let's do the advent of code");
    println!("Day 1 ...");
    println!("\t{}", day1::run(&data_dir));

    println!("Day 1, part 2...");
    println!("\t{}", day1::run_part2(&data_dir));

    println!("Day 2 ...");
    match day2::run(&data_dir) {
        Ok(s) => println!("\t{}", s),
        Err(e) => println!("\tERROR: {}", e)
    };
}
