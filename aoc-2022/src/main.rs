#![allow(dead_code)]
#![allow(unused_imports)]

use std::io::{self, Read};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", day05::part1(&input));
    println!("{}", day05::part2(&input));
}
