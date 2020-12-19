#![allow(dead_code)]
#![allow(unused_imports)]

use std::io::{self, Read};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day17_rewrite;
mod day18;
mod day19;

fn main() {
    day19();
}

fn day19() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day19::day19_1(&input));
    println!("{}", day19::day19_2(&input));
}

fn day18() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day18::day18_1(&input));
    println!("{}", day18::day18_2(&input));
}

fn day17() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day17::day17_1(&input));
    println!("{}", day17::day17_2(&input));

    println!("{}", day17_rewrite::day17_1(&input));
    println!("{}", day17_rewrite::day17_2(&input));
}

fn day16() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day16::day16_1(&input));
    println!("{}", day16::day16_2(&input));
}

fn day15() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day15::day15_1(&input));
    println!("{}", day15::day15_2(&input));
}

fn day14() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day14::day14_1(&input));
    println!("{}", day14::day14_2(&input));
}

fn day13() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day13::day13_1(&input));
    println!("{}", day13::day13_2(&input));
}

fn day12() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day12::day12_1(&input));
    println!("{}", day12::day12_2(&input));
}


fn day11() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day11::day11_1(&input));
    println!("{}", day11::day11_2(&input));
}

fn day10() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day10::day10_1(&input));
    println!("{}", day10::day10_2(&input));
}

fn day09() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day09::day09_1(&input, 25));
    println!("{}", day09::day09_2(&input, 25));
}

fn day08() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day08::day08_1(&input));
    println!("{}", day08::day08_2(&input));
}

fn day07() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day07::day07_1(&input));
    println!("{}", day07::day07_2(&input));
}

fn day06() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day06::day06_1(&input));
    println!("{}", day06::day06_2(&input));
}

fn day05() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day05::day05_1(&input));
    println!("{}", day05::day05_2(&input));
}

fn day04() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day04::day04_1(&input));
    println!("{}", day04::day04_2(&input));
}

fn day03() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day03::day03_1(&input));
    println!("{}", day03::day03_2(&input));
}

fn day02() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", day02::day02_1(&input));
    println!("{}", day02::day02_2(&input));
}

fn day01() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let input: Vec<u32> = input
        .split("\n")
        .filter_map(|x|
                    x.trim().parse().ok())
        .collect();

    println!("{}", day01::day01_1(&input));
    println!("{}", day01::day01_2(&input));
    println!("{}", day01::day01_2_linear(&input));
}
