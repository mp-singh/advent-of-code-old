use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SEAT_REGEX: Regex = Regex::new(r"^([F|B]{7})([R|L]{3})$").unwrap();
}

fn main() {
    let seats = read_input().unwrap();
    println!("Highest seat ID: {:?}", day5a(&seats));
    println!("My seat ID is: {:?}", day5b(&seats));
}

pub fn day5a(seats: &[Seat]) -> i32 {
    match seats.iter().max_by_key(|&seat| seat.id) {
        Some(max) => max.id,
        None => {
            panic!("unable to find max :(");
        }
    }
}

pub fn day5b(seats: &[Seat]) -> i32 {
    find_my_seat(seats)
}

#[derive(Default, Debug)]
pub struct Seat {
    id: i32,
    row: i32,
    column: i32,
}

impl Seat {
    pub fn new(s: String) -> Self {
        let seat = Seat::default();
        let captures = SEAT_REGEX.captures(&s).unwrap();

        let row = seat.find_row(match captures.get(1) {
            Some(x) => x.as_str(),
            None => panic!("invalid row entry"),
        });

        let column = seat.find_col(match captures.get(2) {
            Some(y) => y.as_str(),
            None => panic!("invalid column entry"),
        });

        let id = row * 8 + column;

        Seat { id, row, column }
    }
    pub fn find_row(&self, s: &str) -> i32 {
        self.find_index(128, 'F', 'B', s)
    }
    pub fn find_col(&self, s: &str) -> i32 {
        self.find_index(8, 'L', 'R', s)
    }
    pub fn find_index(&self, len: i32, bottom: char, top: char, code: &str) -> i32 {
        let (low, _, _) =
            code.chars()
                .fold((0, len - 1, len), |(mut low, mut high, mut rem), c| {
                    rem /= 2;
                    match c {
                        c if c == bottom => high -= rem,
                        c if c == top => low += rem,
                        _ => panic!("woops"),
                    }
                    (low, high, rem)
                });
        low
    }
}

fn find_my_seat(seats: &[Seat]) -> i32 {
    let mut result = HashMap::new();
    for i in 0..128 * 8 {
        result.insert(i, false);
    }

    seats.iter().for_each(|seat| {
        result.insert(seat.id, true);
    });

    for i in 1..128 * 8 - 1 {
        // its okay to unwrap here as i'm making sure all values exist above. I still don't like this
        // and I might need to investigate further for a cleaner way of doing this.
        if !result[&i] && *result.get(&(i - 1)).unwrap() && *result.get(&(i + 1)).unwrap() {
            return i;
        }
    }

    panic!("Someone messed up, and it wasn't mee!")
}

fn read_input() -> Result<Vec<Seat>> {
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.map_err(Into::into).map(Seat::new))
        .collect()
}
