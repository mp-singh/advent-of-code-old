use regex::Regex;
use std::fs;

fn main() {
    println!("Total passwords valid at sled: {:?}", day2a());
    println!("Total passwords valid at toboggan: {:?}", day2b());
}

pub fn day2a() -> i32 {
    read_input()
        .iter()
        .fold(0, |total, p| match p.is_valid_at_sled() {
            true => total + 1,
            false => total,
        })
}

pub fn day2b() -> i32 {
    read_input()
        .iter()
        .fold(0, |total, p| match p.is_valid_at_toboggan() {
            true => total + 1,
            false => total,
        })
}

const REGEX: &str = "([0-9]+)-([0-9]+) ([a-z]): ([a-z]*)";

#[derive(Debug)]
struct PasswordPolicy {
    min: usize,
    max: usize,
    letter: String,
    password: String,
}

impl PasswordPolicy {
    pub fn new(s: &str, re: &Regex) -> Option<Self> {
        let captures = re.captures(s)?;
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

        let letter = self.letter.as_bytes()[0] as char;
        let chars = self.password.chars().collect::<Vec<char>>();

        (&chars[self.min - 1] == &letter) ^ (&chars[self.max - 1] == &letter)
    }
}

fn read_input() -> Vec<PasswordPolicy> {
    let values = fs::read_to_string("input.txt").expect("Unable to read file");
    let re = Regex::new(REGEX).unwrap();
    values
        .split("\n")
        .filter_map(|s| PasswordPolicy::new(s, &re))
        .collect()
}
