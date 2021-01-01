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

pub fn run(data_dir: &PathBuf) -> String {
    let fname = data_dir.join("day1-input.txt");
    let numbers = read_data(&fname)
        .expect("Expected a vec of data");

    for (x,y) in numbers.iter().cartesian_product(numbers.iter()) {
        if x + y == 2020 {
            return format!("{}", x * y)
        }
    }

    "Oh shit, fuck, couldn't figure this one out 😬".to_string()
}
