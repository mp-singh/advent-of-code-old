use regex::Regex;
use std::collections::HashSet;
use std::fs;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref GROUP_REGEX: Regex = Regex::new(r"(^[a-zA-Z]+)$").unwrap();
}

fn main() {
    let groups = read_input();
    println!("All Sum: {:?}", day6a(&groups));
    println!("Everyone Sum: {:?}", day6b(&groups));
}

pub fn day6a(groups: &[Group]) -> i32 {
    groups.iter().map(|g| g.all_yes_sum).sum()
}

pub fn day6b(groups: &[Group]) -> i32 {
    groups.iter().map(|g| g.everyone_yes_sum).sum()
}

#[derive(Default, Debug)]
pub struct Group {
    answers: String,
    all_yes_sum: i32,
    everyone_yes_sum: i32,
}

impl Group {
    pub fn new(s: &str) -> Self {
        if GROUP_REGEX.is_match(&s.replace('\n', "")) {
            Group {
                answers: s.to_string(),
                all_yes_sum: Group::find_all_yes_sum(s),
                everyone_yes_sum: Group::find_everyone_yes_sum(s),
            }
        } else {
            panic!("Bad input!")
        }
    }

    fn find_all_yes_sum(s: &str) -> i32 {
        let mut result = HashSet::new();
        s.chars().filter(|c| *c != '\n').for_each(|c| {
            result.insert(c);
        });
        result.len() as i32
    }

    fn find_everyone_yes_sum(s: &str) -> i32 {
        let mut result = [0usize; 26];
        let group_size = s.split('\n').filter(|&s| !s.is_empty()).count();
        s.chars().filter(|c| *c != '\n').for_each(|c| {
            let index = c as usize - 97;
            result[index] += 1
        });
        result.iter().filter(|&&size| size == group_size).count() as i32
    }
}

fn read_input() -> Vec<Group> {
    fs::read_to_string("input.txt")
        .expect("unable to read file")
        .split("\n\n")
        .map(Group::new)
        .collect()
}
