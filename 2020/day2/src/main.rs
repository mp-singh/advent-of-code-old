use anyhow::Result;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new("([0-9]+)-([0-9]+) ([a-z]): ([a-z]*)").unwrap();
}
fn main() {
    let password_policies = read_input().unwrap();
    println!(
        "Total passwords valid at sled: {:?}",
        day2a(&password_policies)
    );
    println!(
        "Total passwords valid at toboggan: {:?}",
        day2b(&password_policies)
    );
}

pub fn day2a(password_policies: &[Option<PasswordPolicy>]) -> i32 {
    password_policies.iter().fold(0, |total, p| match p {
        Some(x) => {
            if x.is_valid_at_sled() {
                total + 1
            } else {
                total
            }
        }
        None => total,
    })
}

pub fn day2b(password_policies: &[Option<PasswordPolicy>]) -> i32 {
    password_policies.iter().fold(0, |total, p| match p {
        Some(x) => {
            if x.is_valid_at_toboggan() {
                total + 1
            } else {
                total
            }
        }
        None => total,
    })
}

#[derive(Debug)]
pub struct PasswordPolicy {
    min: usize,
    max: usize,
    letter: String,
    password: String,
}

impl PasswordPolicy {
    pub fn new(s: String) -> Option<Self> {
        let captures = RE.captures(&s)?;
        Some(Self {
            min: captures
                .get(1)
                .and_then(|s| s.as_str().parse::<usize>().ok())?,
            max: captures
                .get(2)
                .and_then(|s| s.as_str().parse::<usize>().ok())?,
            letter: captures.get(3)?.as_str().to_string(),
            password: captures.get(4)?.as_str().to_string(),
        })
    }
    pub fn is_valid_at_sled(&self) -> bool {
        let count = self.password.matches(&self.letter).count();
        count >= self.min && count <= self.max
    }
    pub fn is_valid_at_toboggan(&self) -> bool {
        if self.password.len() < self.max {
            return false;
        }

        let letter = self.letter.chars().nth(0).unwrap();
        let chars = self.password.chars().collect::<Vec<char>>();

        (chars[self.min - 1] == letter) ^ (chars[self.max - 1] == letter)
    }
}

fn read_input() -> Result<Vec<Option<PasswordPolicy>>> {
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.map_err(Into::into).map(PasswordPolicy::new))
        .collect()
}
