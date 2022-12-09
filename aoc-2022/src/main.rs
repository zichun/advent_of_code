#![allow(dead_code)]
#![allow(unused_imports)]

use std::io::{self, Read};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", day09::part1(&input));
    println!("{}", day09::part2(&input));
}
