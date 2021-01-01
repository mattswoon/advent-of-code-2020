use std::{
    fs::File,
    path::PathBuf,
    io::{BufReader, BufRead}
};

enum Square {
    Open,
    Tree
}

impl Square {
    fn from_char(c: char) -> Result<Square, &'static str> {
        match c {
            '.' => Ok(Square::Open),
            '#' => Ok(Square::Tree),
            _ => Err("Not a square character I understand")
        }
    }
}

struct Row {
    squares: Vec<Square>
}

impl Row {
    fn square(&self, n: usize) -> &Square {
        let n_squares = self.squares.len();
        let col = n % n_squares;
        self.squares.iter().nth(col).unwrap()
    }

    fn from_str(s: &str) -> Result<Row, &'static str> {
        let squares = s.chars()
            .map(Square::from_char)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Row { squares: squares })
    }
}

struct Hill {
    rows: Vec<Row>,
}

impl Hill {
    fn read_file(fname: PathBuf) -> Result<Hill, &'static str> {
        let file = File::open(fname)
            .map_err(|_| "Couldn't open the file")?;
        let rows = BufReader::new(file)
            .lines()
            .map(|r| r.map(|l| Row::from_str(&l)).map_err(|_| "Couldn't read line").flatten())
            .collect::<Result<Vec<Row>, _>>()?;
        Ok(Hill { rows: rows })
    }

    fn num_trees_hit(&self, down: usize, right: usize) -> usize {
        let mut col = 0;
        let mut count = 0;
        for row in self.rows.iter().skip(down).step_by(down) {
            col = (col + right) % row.squares.len();
            count += match row.square(col) {
                Square::Tree => 1,
                Square::Open => 0
            }
        }
        count
    }
}

pub fn run(data_dir: &PathBuf) -> Result<String, &'static str> {
    let hill = Hill::read_file(data_dir.join("day3-input.txt"))
        .map_err(|_| "Couldn't read file")?;
    Ok(format!("{}", hill.num_trees_hit(1, 3)))
}

pub fn run_part2(data_dir: &PathBuf) -> Result<String, &'static str> {
    let hill = Hill::read_file(data_dir.join("day3-input.txt"))
        .map_err(|_| "Couldn't read file")?;
    let trees_hit = vec![hill.num_trees_hit(1, 1),
                         hill.num_trees_hit(1, 3),
                         hill.num_trees_hit(1, 5),
                         hill.num_trees_hit(1, 7),
                         hill.num_trees_hit(2, 1)];
    let n: usize = trees_hit.iter().product();
    Ok(format!("{}", n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_trees_hit() {
        let hill = Hill { rows: vec![Row::from_str("..##.......").unwrap(),
                                     Row::from_str("#...#...#..").unwrap(),
                                     Row::from_str(".#....#..#.").unwrap(),
                                     Row::from_str("..#.#...#.#").unwrap(),
                                     Row::from_str(".#...##..#.").unwrap(),
                                     Row::from_str("..#.##.....").unwrap(),
                                     Row::from_str(".#.#.#....#").unwrap(),
                                     Row::from_str(".#........#").unwrap(),
                                     Row::from_str("#.##...#...").unwrap(),
                                     Row::from_str("#...##....#").unwrap(),
                                     Row::from_str(".#..#...#.#").unwrap()] };
        let n = hill.num_trees_hit(1, 3);
        assert_eq!(n, 7);

    }

    #[test]
    fn test_num_trees_hit_part2() {
        let hill = Hill { rows: vec![Row::from_str("..##.......").unwrap(),
                                     Row::from_str("#...#...#..").unwrap(),
                                     Row::from_str(".#....#..#.").unwrap(),
                                     Row::from_str("..#.#...#.#").unwrap(),
                                     Row::from_str(".#...##..#.").unwrap(),
                                     Row::from_str("..#.##.....").unwrap(),
                                     Row::from_str(".#.#.#....#").unwrap(),
                                     Row::from_str(".#........#").unwrap(),
                                     Row::from_str("#.##...#...").unwrap(),
                                     Row::from_str("#...##....#").unwrap(),
                                     Row::from_str(".#..#...#.#").unwrap()] };
        let trees_hit = vec![hill.num_trees_hit(1, 1),
                             hill.num_trees_hit(1, 3),
                             hill.num_trees_hit(1, 5),
                             hill.num_trees_hit(1, 7),
                             hill.num_trees_hit(2, 1)];
        let n: usize = trees_hit.iter().product();
        assert_eq!(n, 336);

    }
}
