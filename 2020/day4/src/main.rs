use regex::Regex;
use std::fs;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref HCL_REGEX: Regex = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
    static ref PID_REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    static ref HGT_REGEX: Regex = Regex::new(r"^([1-9][0-9]{1,2})(cm|in)$").unwrap();
}

fn main() {
    let passports = read_input();
    println!(
        "Passports with all required fields: {:?}",
        day4a(&passports)
    );
    println!("Passports with all valid feilds: {:?}", day4b(&passports));
}

pub fn day4a(passports: &[Passport]) -> u32 {
    passports
        .iter()
        .fold(0u32, |total, p| match p.is_valid_passport() {
            true => total + 1,
            false => total,
        })
}

pub fn day4b(passports: &[Passport]) -> u32 {
    passports
        .iter()
        .fold(0u32, |total, p| match p.is_valid_passport_part2() {
            true => total + 1,
            false => total,
        })
}

#[derive(Default, Debug)]
pub struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl Passport {
    pub fn new(s: &str) -> Option<Self> {
        let mut passport = Passport::default();
        let attributes = s.split_whitespace().collect::<Vec<&str>>();
        for attribute in attributes {
            let parts = attribute.split(':').collect::<Vec<&str>>();
            if parts.len() == 2 {
                match parts[0] {
                    "byr" => passport.byr = parts[1].to_string(),
                    "iyr" => passport.iyr = parts[1].to_string(),
                    "eyr" => passport.eyr = parts[1].to_string(),
                    "hgt" => passport.hgt = parts[1].to_string(),
                    "hcl" => passport.hcl = parts[1].to_string(),
                    "ecl" => passport.ecl = parts[1].to_string(),
                    "pid" => passport.pid = parts[1].to_string(),
                    "cid" => passport.cid = Some(parts[1].to_string()),
                    _ => {}
                }
            }
        }
        Some(passport)
    }

    pub fn is_valid_passport(&self) -> bool {
        !self.byr.is_empty()
            && !self.iyr.is_empty()
            && !self.eyr.is_empty()
            && !self.hgt.is_empty()
            && !self.hcl.is_empty()
            && !self.ecl.is_empty()
            && !self.pid.is_empty()
    }

    pub fn is_valid_passport_part2(&self) -> bool {
        self.validate_byr()
            && self.validate_iyr()
            && self.validate_eyr()
            && self.validate_hgt()
            && self.validate_hcl()
            && self.validate_ecl()
            && self.validate_pid()
    }

    fn validate_byr(&self) -> bool {
        match self.byr.parse::<u32>() {
            Ok(x) => 1920 <= x && x <= 2002,
            _ => false,
        }
    }

    fn validate_iyr(&self) -> bool {
        match self.iyr.parse::<u32>() {
            Ok(x) => 2010 <= x && x <= 2020,
            _ => false,
        }
    }

    fn validate_eyr(&self) -> bool {
        match self.eyr.parse::<u32>() {
            Ok(x) => 2020 <= x && x <= 2030,
            _ => false,
        }
    }

    fn validate_hgt(&self) -> bool {
        match HGT_REGEX.captures(&self.hgt) {
            Some(cap) => {
                let num = cap[1].parse::<i32>().unwrap_or(0);
                match &cap[2] {
                    "cm" => 150 <= num && num <= 193,
                    "in" => 59 <= num && num <= 76,
                    _ => false,
                }
            }
            None => false,
        }
    }

    fn validate_hcl(&self) -> bool {
        HCL_REGEX.is_match(&self.hcl)
    }

    fn validate_ecl(&self) -> bool {
        matches!(
            self.ecl.as_str(),
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
        )
    }

    fn validate_pid(&self) -> bool {
        PID_REGEX.is_match(&self.pid)
    }
}

fn read_input() -> Vec<Passport> {
    fs::read_to_string("input.txt")
        .expect("unable to read file")
        .split("\n\n")
        .filter_map(Passport::new)
        .collect()
}
