use anyhow::Result;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let expense = read_input().unwrap();
    println!("2 Sum Problem: {:?}", day1a(&expense));
    println!("3 Sum Problem: {:?}", day1b(&expense));
    Ok(())
}

pub fn day1a(expenses: &[i32]) -> i32 {
    match expenses
        .iter()
        .combinations(2)
        .find(|v| v[0] + v[1] == 2020)
        .map(|v| v.into_iter().product::<i32>())
    {
        Some(r) => r,
        None => 0,
    }
}

pub fn day1b(expenses: &[i32]) -> i32 {
    match expenses
        .iter()
        .combinations(3)
        .find(|v| v[0] + v[1] + v[2] == 2020)
        .map(|v| v.into_iter().product::<i32>())
    {
        Some(r) => r,
        None => 0,
    }
}

fn read_input() -> Result<Vec<i32>> {
    BufReader::new(File::open("input.txt")?)
        .lines()
        .map(|line| {
            line.map_err(Into::into)
                .and_then(|s| s.parse::<i32>().map_err(Into::into))
        })
        .collect()
}
