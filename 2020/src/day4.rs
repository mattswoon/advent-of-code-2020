use std::{
    fs::File,
    path::PathBuf,
    io::Read
};
use regex::Regex;

struct PassportChecker {
    check_values: bool 
}

impl PassportChecker {
    fn is_valid_passport(&self, fields: &Vec<Field>) -> bool {
        if self.check_values {
            PassportChecker::is_valid_check_values(fields)
        } else {
            PassportChecker::is_valid_fields_only(fields)
        }
    }

    fn is_valid_check_values(fields: &Vec<Field>) -> bool {
        let mut valid_byr = false;
        let mut valid_iyr = false;
        let mut valid_eyr = false;
        let mut valid_hgt = false;
        let mut valid_hcl = false;
        let mut valid_ecl = false;
        let mut valid_pid = false;

        for field in fields {
            match field {
                Field::Byr(_) => { valid_byr = field.is_valid(); },
                Field::Iyr(_) => { valid_iyr = field.is_valid(); },
                Field::Eyr(_) => { valid_eyr = field.is_valid(); },
                Field::Hgt(_) => { valid_hgt = field.is_valid(); },
                Field::Hcl(_) => { valid_hcl = field.is_valid(); },
                Field::Ecl(_) => { valid_ecl = field.is_valid(); },
                Field::Pid(_) => { valid_pid = field.is_valid(); },
                _ => { () }
            }
        }
        valid_byr & valid_iyr & valid_eyr & valid_hgt & valid_hcl & valid_ecl & valid_pid
    }

    fn is_valid_fields_only(fields: &Vec<Field>) -> bool {
        let mut valid_byr = false;
        let mut valid_iyr = false;
        let mut valid_eyr = false;
        let mut valid_hgt = false;
        let mut valid_hcl = false;
        let mut valid_ecl = false;
        let mut valid_pid = false;

        for field in fields {
            match field {
                Field::Byr(_) => { valid_byr = true; },
                Field::Iyr(_) => { valid_iyr = true; },
                Field::Eyr(_) => { valid_eyr = true; },
                Field::Hgt(_) => { valid_hgt = true; },
                Field::Hcl(_) => { valid_hcl = true; },
                Field::Ecl(_) => { valid_ecl = true; },
                Field::Pid(_) => { valid_pid = true; },
                _ => { () }
            }
        }
        valid_byr & valid_iyr & valid_eyr & valid_hgt & valid_hcl & valid_ecl & valid_pid
    }
}

#[derive(Debug, Clone)]
enum HeightValue {
    Cm(i32),
    In(i32)
}

#[derive(Debug, Clone)]
enum Field {
    Byr(i32),
    Iyr(i32),
    Eyr(i32),
    Hgt(HeightValue),
    Hcl(String),
    Ecl(String),
    Pid(String),
    Cid(String)
}

#[derive(Debug)]
pub enum PassportError {
    NotValid,
    ParseIntError(std::num::ParseIntError),
    ParseHeightError(&'static str),
    Other(&'static str)
}

impl std::fmt::Display for PassportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PassportError::NotValid => write!(f, "Not a valid passport"),
            PassportError::Other(s) => write!(f, "{}", s),
            PassportError::ParseIntError(e) => write!(f, "{}", e),
            PassportError::ParseHeightError(e) => write!(f, "{}", e)
        }
    }
}

impl Field {
    fn from_str(s: String) -> Result<Field, PassportError> {
        if let Some((code, value)) = s.split_once(':') {
            return match code {
                "byr" => {
                    let v = value.parse::<i32>()
                        .map_err(PassportError::ParseIntError)?;
                    Ok(Field::Byr(v))
                },
                "iyr" => {
                    let v = value.parse::<i32>()
                        .map_err(PassportError::ParseIntError)?;
                    Ok(Field::Iyr(v))
                },
                "eyr" => {
                    let v = value.parse::<i32>()
                        .map_err(PassportError::ParseIntError)?;
                    Ok(Field::Eyr(v))
                },
                "hgt" => {
                    let re = Regex::new(r"(\d+)(in|cm)").unwrap();
                    let cap = re.captures_iter(value).next()
                        .ok_or(PassportError::ParseHeightError("Regex didn't work"))?;
                    let num = cap[1].parse::<i32>()
                        .map_err(|e| PassportError::ParseIntError(e))?;
                    match &cap[2] {
                        "cm" => Ok(Field::Hgt(HeightValue::Cm(num))),
                        "in" => Ok(Field::Hgt(HeightValue::In(num))),
                        _ => Err(PassportError::ParseHeightError("Don't recognise measurement"))
                    }
                },
                "hcl" => Ok(Field::Hcl(value.to_string())),
                "ecl" => Ok(Field::Ecl(value.to_string())),
                "pid" => Ok(Field::Pid(value.to_string())),
                "cid" => Ok(Field::Cid(value.to_string())),
                _ => Err(PassportError::Other("Not a valid field variant")),
            }
        }
        Err(PassportError::Other("Not a valid field string"))
    }

    fn is_valid(&self) -> bool {
        match self {
            Field::Byr(v) => (1920..=2002).contains(v),
            Field::Iyr(v) => (2010..=2020).contains(v),
            Field::Eyr(v) => (2020..=2030).contains(v),
            _ => false
        }
    }
}

fn fields_from_string(s: String) -> Result<Vec<Vec<Field>>, PassportError> {
    s.split("\n\n")
        .into_iter()
        .map(|s| {
            s.split_whitespace()
                .map(|x| Field::from_str(x.to_string()))
                .collect::<Result<Vec<Field>, _>>()
        })
        .collect::<Result<Vec<Vec<Field>>, _>>()
}

pub fn run(data_dir: &PathBuf) -> Result<String, PassportError> {
    let mut file = File::open(data_dir.join("day4-input.txt"))
        .map_err(|_| PassportError::Other("Couldn't open file"))?;
    let mut buf: String = String::new();
    file.read_to_string(&mut buf)
        .map_err(|_| PassportError::Other("Couldn't read file"))?;
    
    let checker = PassportChecker { check_values: false };
    let field_batches = fields_from_string(buf)?;
    let num_passports = field_batches.into_iter()
        .filter(|fs| checker.is_valid_passport(fs))
        .count();
    Ok(format!("{}", num_passports))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passport_validation_as_field_vecs() {
        let batch = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        let checker = PassportChecker { check_values: false };
        let field_batches = fields_from_string(batch.to_string())
            .unwrap();
        let count = field_batches.into_iter()
            .filter(|fs| checker.is_valid_passport(fs))
            .count();
        assert_eq!(count, 2);
    }
}
