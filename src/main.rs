pub mod day1;

use std::{
    env,
    path::PathBuf
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let data_dir: PathBuf = (&args[1]).into();

    println!("Well hello there, let's do the advent of code");
    println!("Day 1 ...");
    println!("\t{}", day1::run(&data_dir));
}
