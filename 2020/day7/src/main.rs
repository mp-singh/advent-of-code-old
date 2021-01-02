use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CONTAINS_REGEX: Regex = Regex::new(r"^(.*) bags contain (.*)\.$").unwrap();
    static ref BAGS_REGEX: Regex = Regex::new(r"^\s*(\d*) (.*) bags?$").unwrap();
}

fn main() {
    let all_bags = read_input();
    println!("{:?}", day7a(&all_bags));
    println!("{:?}", day7b(&all_bags));
}

pub fn day7a(all_bags: &HashMap<String, Bag>) -> i32 {
    all_bags
        .values()
        .filter(|b| b.can_hold("shiny gold", &all_bags))
        .count() as i32
}

pub fn day7b(all_bags: &HashMap<String, Bag>) -> i32 {
    count_bags(all_bags.get("shiny gold").unwrap(), all_bags)
}

#[derive(Default, Debug)]
pub struct Bag {
    color: String,
    contains: Vec<(usize, String)>,
}

impl Bag {
    pub fn new(s: &str) -> Option<Self> {
        let caps = CONTAINS_REGEX.captures(&s)?;

        let mut bag = Bag {
            color: caps.get(1)?.as_str().to_string(),
            contains: Vec::new(),
        };

        let contains = caps.get(2)?.as_str();
        if matches!(contains, "no other bags") {
            return Some(bag);
        }

        for s in contains.split(", ") {
            let cap = BAGS_REGEX.captures(s)?;
            let n = cap.get(1)?.as_str().parse::<usize>().unwrap();
            let color = cap.get(2)?.as_str();
            bag.contains.push((n, color.to_string()))
        }

        Some(bag)
    }

    pub fn can_hold(&self, my_bag_color: &str, all_bags: &HashMap<String, Bag>) -> bool {
        if self.contains.is_empty() {
            return false;
        }
        self.contains.iter().any(|(_, b)| {
            b == my_bag_color || all_bags.get(b).unwrap().can_hold(my_bag_color, all_bags)
        })
    }
}

fn count_bags(bag: &Bag, all_bags: &HashMap<String, Bag>) -> i32 {
    if bag.contains.is_empty() {
        return 0;
    }
    let n = bag.contains.iter().fold(0, |total, (n, b)| {
        let inner_bag_count = count_bags(all_bags.get(b.as_str()).unwrap(), all_bags);
        total + n * (1 + inner_bag_count as usize)
    });
    n as i32
}

fn read_input() -> HashMap<String, Bag> {
    let mut result = HashMap::new();
    fs::read_to_string("input.txt")
        .expect("unable to read file")
        .split('\n')
        .filter_map(Bag::new)
        .for_each(|b| {
            result.insert(b.color.clone(), b);
        });
    result
}
