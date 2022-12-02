use std::io::Read;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day10;
mod day14;
mod day15;
mod day18;
mod day21;
mod day24;

fn main() {
    let mut inp = String::new();
    std::io::stdin().read_to_string(&mut inp).unwrap();
    println!("{}", day24::p1(&inp));
//    println!("{}", day18::p2(&inp));
}

