use crate::prelude::*;

struct Balls {
    red: u32,
    green: u32,
    blue: u32,
}
type Game = Vec<Balls>;

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Game> {
    input.split("\n")
        .map(|game| {
            game.split(": ").skip(1).next().unwrap()
                .split("; ")
                .map(|round| {
                    let (mut red, mut green, mut blue) = (0, 0, 0);
                    round.split(", ").for_each(|t| {
                        let mut tokens = t.split(" ");
                        let cnt = tokens.next().unwrap().parse::<u32>().unwrap();
                        match tokens.next().unwrap() {
                            "red" => red = cnt,
                            "green" => green = cnt,
                            "blue" => blue = cnt,
                            _ => panic!("unrecognized"),
                        }
                    });
                    Balls { red, green, blue }
                }).collect()
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Game]) -> u32 {
    input.iter().enumerate().filter_map(|(ind, l)| {
        if l.iter().filter(|b| {
            b.red > 12 || b.green > 13 || b.blue > 14
        }).count() > 0 {
            None
        } else {
            Some(ind + 1)
        }
    }).sum::<usize>() as u32
}

#[aoc(day2, part2)]
fn part2(input: &[Game]) -> u32 {
    input.iter().map(|game| {
        let min = game.iter().fold((0, 0, 0), |acc, round| {
            (acc.0.max(round.red),
             acc.1.max(round.blue),
             acc.2.max(round.green))
        });
        min.0 * min.1 * min.2
    }).sum()
}
