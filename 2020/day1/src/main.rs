use itertools::Itertools;
use std::fs;

fn main() {
    println!("2 Sum Problem: {:?}", day1a());
    println!("3 Sum Problem: {:?}", day1b());
}

pub fn day1a() -> i32 {
    match read_input()
        .iter()
        .combinations(2)
        .find(|v| v[0] + v[1] == 2020)
        .map(|v| v.into_iter().product::<usize>())
    {
        Some(r) => r as i32,
        None => 0,
    }
}

pub fn day1b() -> i32 {
    match read_input()
        .iter()
        .combinations(3)
        .find(|v| v[0] + v[1] + v[2] == 2020)
        .map(|v| v.into_iter().product::<usize>())
    {
        Some(r) => r as i32,
        None => 0,
    }
}

fn read_input() -> Vec<usize> {
    let values = fs::read_to_string("input.txt").expect("Unable to read file");
    values
        .split("\n")
        .filter_map(|s| s.parse::<usize>().ok())
        .collect()
}
