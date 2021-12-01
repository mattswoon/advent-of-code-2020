use std::{
    io::{BufReader, BufRead},
    fs::File,
    path::PathBuf
};
use itertools::Itertools;

fn read_data(fname: &PathBuf) -> Result<Vec<i32>, &'static str> {
    let file = File::open(fname)
        .map_err(|_| "Couldn't open the file")?;
    BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "Trouble reading lines")?
        .into_iter()
        .map(|l| l.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "Trouble parsing the lines into ints")
}

pub fn run(data_dir: &PathBuf) -> Result<String, &'static str> {
    let fname = data_dir.join("day1-input.txt");
    let numbers = read_data(&fname)
        .map_err(|_| "Expected a vec of data")?;

    for (x,y) in numbers.iter().cartesian_product(numbers.iter()) {
        if x + y == 2020 {
            return Ok(format!("{}", x * y))
        }
    }

    Err("Oh shit, fuck, couldn't figure this one out 😬")
}

pub fn run_part2(data_dir: &PathBuf) -> Result<String, &'static str> {
    let fname = data_dir.join("day1-input.txt");
    let numbers = read_data(&fname)
        .map_err(|_| "Expected a vec of data")?;

    let triplets = numbers.iter()
        .cartesian_product(numbers.iter()
                           .cartesian_product(numbers.iter()));
    for (x,(y,z)) in triplets {
        if x + y + z == 2020 {
            return Ok(format!("{}", x * y * z))
        }
    }

    Err("Oh shit, fuck, couldn't figure this one out 😬")
}
