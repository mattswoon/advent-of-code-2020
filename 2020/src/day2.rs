use std::{
    path::PathBuf,
    fs::File,
    io::{BufReader, BufRead}
};
use regex::Regex;

struct Policy {
    pub lb: i32,
    pub ub: i32,
    pub letter: char
}

impl Policy {
    fn from_str(s: &str) -> Result<Policy, &'static str> {
        let re = Regex::new(r"(\d+)-(\d+) ([a-z])").unwrap();
        let cap = re.captures_iter(s).next()
            .ok_or("Regex didn't work")?;
        let lb: i32 = cap[1].parse::<i32>()
            .map_err(|_| "Couldn't parse lower bound")?;
        let ub: i32 = cap[2].parse::<i32>()
            .map_err(|_| "Couldn't parse upper bound")?;
        let letter: char = cap[3].chars().nth(0)
            .ok_or("Couldn't get the letter")?;
        Ok(Policy { lb: lb, ub: ub, letter: letter })
    }

    fn is_valid(&self, password: &str) -> bool {
        let letter_count: i32 = password.chars()
            .map(|c| if c == self.letter { 1 } else { 0 })
            .sum();
        (self.lb..=self.ub).contains(&letter_count)
    }

    fn is_valid_part2(&self, password: &str) -> bool {
        if let Some(l) = password.chars().nth((self.lb - 1) as usize) {
            if let Some(u) = password.chars().nth((self.ub - 1) as usize) {
                return match (l == self.letter, u == self.letter) {
                    (true, true) => false,
                    (true, false) => true,
                    (false, true) => true,
                    (false, false) => false
                }
            }
        }
        false
    }
}

fn valid_password(line: String) -> Result<bool, &'static str> {
    if let Some((policy_str, password)) = line.split_once(':') {
        let policy = Policy::from_str(&policy_str[..]);
        let password = password.trim_start();
        policy.map(|p| p.is_valid(password))
    } else {
        Err("Failed to interpret line")
    }
}

fn valid_password_part2(line: String) -> Result<bool, &'static str> {
    if let Some((policy_str, password)) = line.split_once(':') {
        let policy = Policy::from_str(&policy_str[..]);
        let password = password.trim_start();
        policy.map(|p| p.is_valid_part2(password))
    } else {
        Err("Failed to interpret line")
    }
}

pub fn run(data_dir: &PathBuf) -> Result<String, &'static str> {
    let file = File::open(data_dir.join("day2-input.txt"))
        .map_err(|_| "Couldn't open the file")?;
    BufReader::new(file)
        .lines()
        .map(|x| x.map_err(|_| "Couldn't read line").map(valid_password).flatten())
        .collect::<Result<Vec<_>, _>>()
        .map(|v| v.into_iter().filter(|&x| x).count())
        .map(|x| format!("{}", x))
}

pub fn run_part2(data_dir: &PathBuf) -> Result<String, &'static str> {
    let file = File::open(data_dir.join("day2-input.txt"))
        .map_err(|_| "Couldn't open the file")?;
    BufReader::new(file)
        .lines()
        .map(|x| x.map_err(|_| "Couldn't read line").map(valid_password_part2).flatten())
        .collect::<Result<Vec<_>, _>>()
        .map(|v| v.into_iter().filter(|&x| x).count())
        .map(|x| format!("{}", x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_policy() {
        let p = Policy::from_str("3-4 l").unwrap();
        assert_eq!(p.lb, 3);
        assert_eq!(p.ub, 4);
        assert_eq!(p.letter, 'l');
        
        let p = Policy::from_str("10-20 x").unwrap();
        assert_eq!(p.lb, 10);
        assert_eq!(p.ub, 20);
        assert_eq!(p.letter, 'x');
    }

    #[test]
    fn test_password_validity() {
        let p = Policy::from_str("3-4 l").unwrap();
        assert!(p.is_valid("absdfhlasdflasdfl"));
    }

    #[test]
    fn test_password_validity_2() {
        let p = Policy::from_str("3-4 l").unwrap();
        assert!(p.is_valid_part2("abldefghed"));
    }
}
