use std::{
    fs::File,
    path::PathBuf,
    io::Read
};

#[allow(dead_code)]
#[derive(Debug)]
struct Passport {
    byr: Field,
    iyr: Field,
    eyr: Field,
    hgt: Field,
    hcl: Field,
    ecl: Field,
    pid: Field,
    cid: Option<Field>
}

impl Passport {
    fn from_fields(fields: Vec<Field>) -> Result<Passport, PassportError> {
        let byr = field(&fields, "byr").ok_or(PassportError::NotValid)?;
        let iyr = field(&fields, "iyr").ok_or(PassportError::NotValid)?;
        let eyr = field(&fields, "eyr").ok_or(PassportError::NotValid)?;
        let hgt = field(&fields, "hgt").ok_or(PassportError::NotValid)?;
        let hcl = field(&fields, "hcl").ok_or(PassportError::NotValid)?;
        let ecl = field(&fields, "ecl").ok_or(PassportError::NotValid)?;
        let pid = field(&fields, "pid").ok_or(PassportError::NotValid)?;
        let cid = field(&fields, "cid");
        Ok(Passport {
            byr: byr,
            iyr: iyr,
            eyr: eyr,
            hgt: hgt,
            hcl: hcl,
            ecl: ecl,
            pid: pid,
            cid: cid
        })
    }
}

fn field(fields: &Vec<Field>, code: &str) -> Option<Field> {
    fields.iter().filter(|f| match f {
        Field::Byr(_) => "byr" == code,
        Field::Iyr(_) => "iyr" == code,
        Field::Eyr(_) => "eyr" == code,
        Field::Hgt(_) => "hgt" == code,
        Field::Hcl(_) => "hcl" == code,
        Field::Ecl(_) => "ecl" == code,
        Field::Pid(_) => "pid" == code,
        Field::Cid(_) => "cid" == code,
    }).next().map(|f| f.clone())
}

#[derive(Debug, Clone)]
enum Field {
    Byr(String),
    Iyr(String),
    Eyr(String),
    Hgt(String),
    Hcl(String),
    Ecl(String),
    Pid(String),
    Cid(String)
}

#[derive(Debug)]
pub enum PassportError {
    NotValid,
    Other(&'static str)
}

impl std::fmt::Display for PassportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PassportError::NotValid => write!(f, "Not a valid passport"),
            PassportError::Other(s) => write!(f, "{}", s)
        }
    }
}

impl Field {
    fn from_str(s: String) -> Result<Field, PassportError> {
        if let Some((code, value)) = s.split_once(':') {
            return match code {
                "byr" => Ok(Field::Byr(value.to_string())),
                "iyr" => Ok(Field::Iyr(value.to_string())),
                "eyr" => Ok(Field::Eyr(value.to_string())),
                "hgt" => Ok(Field::Hgt(value.to_string())),
                "hcl" => Ok(Field::Hcl(value.to_string())),
                "ecl" => Ok(Field::Ecl(value.to_string())),
                "pid" => Ok(Field::Pid(value.to_string())),
                "cid" => Ok(Field::Cid(value.to_string())),
                _ => Err(PassportError::Other("Not a valid field variant")),
            }
        }
        Err(PassportError::Other("Not a valid field string"))
    }
}

fn passports_from_string(s: String) -> Result<Vec<Result<Passport, PassportError>>, PassportError> {
    let passports = s.split("\n\n")
        .into_iter()
        .map(|s| {
            s.split_whitespace()
                .map(|s| Field::from_str(s.to_string()))
                .collect::<Result<Vec<Field>, _>>()
        })
        .collect::<Result<Vec<Vec<Field>>, _>>()?
        .into_iter()
        .map(|v| Passport::from_fields(v))
        .collect::<Vec<Result<Passport, PassportError>>>();
    Ok(passports)
}

pub fn run(data_dir: &PathBuf) -> Result<String, PassportError> {
    let mut file = File::open(data_dir.join("day4-input.txt"))
        .map_err(|_| PassportError::Other("Couldn't open file"))?;
    let mut buf: String = String::new();
    file.read_to_string(&mut buf)
        .map_err(|_| PassportError::Other("Couldn't read file"))?;
    
    let passports = passports_from_string(buf)?;
    let num_passports = passports.into_iter()
        .filter(|x| x.is_ok())
        .count();
    Ok(format!("{}", num_passports))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passport_validation() {
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
        let passports = passports_from_string(batch.to_string())
            .unwrap();
        let count = passports.into_iter()
            .filter(|x| x.is_ok())
            .count();
        assert_eq!(count, 2);
    }
}
