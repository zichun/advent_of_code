use crate::prelude::*;
use itertools::FoldWhile::{Continue, Done};
use num::integer::lcm;

struct Input {
    pattern: Vec<char>,
    map: HashMap<String, (String, String)>,
}
impl Input {
    fn traverse<'a, F: Fn(&str) -> bool>(&'a self, start: &'a str, done: F) -> (u32, &'a str) {
        self.pattern
            .iter()
            .cycle()
            .fold_while((0, start), |(steps, loc), el| {
                if steps > 0 && done(loc) {
                    Done((steps, loc))
                } else {
                    Continue((
                        steps + 1,
                        match *el {
                            'L' => &self.map[loc].0,
                            'R' => &self.map[loc].1,
                            _ => unreachable!(),
                        }
                    ))
                }
            })
            .into_inner()
    }
}
#[aoc_generator(day8)]
fn parse(inp: &str) -> Input {
    let mut inp = inp.lines();
    let pattern = inp.next_token::<String>().chars().collect();
    let map = inp
        .skip(1)
        .map(|el| {
            let mut el = el.split(" = ");
            let start = el.next_token();
            let s = el.next_token::<String>();
            let mut el = s.split(", ");
            (
                start,
                (
                    el.next_token::<String>()[1..=3].to_owned(),
                    el.next_token::<String>()[0..=2].to_owned(),
                ),
            )
        })
        .collect();
    Input { pattern, map }
}
#[aoc(day8, part1)]
fn part1(input: &Input) -> u32 {
    input.traverse("AAA", |node| node == "ZZZ").0
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> u64 {
    input
        .map
        .keys()
        .filter(|k| k.ends_with("A"))
        .collect::<Vec<&String>>()
        .iter()
        .map(|node| input.traverse(node, |node| node.ends_with("Z")).0 as u64)
        .fold(1, |acc, el| lcm(acc, el))
}
