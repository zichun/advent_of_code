use crate::prelude::*;

#[aoc_generator(day11)]
fn parse(inp: &str) -> Vec<(usize, usize)> {
    inp.lines()
        .enumerate()
        .flat_map(|(r, l)| {
            l.chars()
                .enumerate()
                .filter_map(move |(c, ch)| if ch == '#' { Some((r, c)) } else { None })
        })
        .collect()
}

fn add_1d<F: Fn((usize, usize)) -> usize>(inp: &mut Vec<(usize, usize)>, f: F, mul: u64) -> u64 {
    inp.sort_by(|a, b| f(*a).cmp(&f(*b)));
    let mut sum = 0;
    for i in 0..inp.len() {
        let mut acc = 0;
        for j in i+1..inp.len() {
            if f(inp[j]) > f(inp[j-1]) {
                acc += mul * ((f(inp[j]) - f(inp[j-1]) - 1) as u64) + 1;
            }
            sum += acc;
        }
    }
    sum
}


#[aoc(day11, part1)]
fn part1(inp: &Vec<(usize, usize)>) -> u64 {
    let mut inp = inp.to_owned();
    add_1d(&mut inp, |(r, _c)| r, 2) +
        add_1d(&mut inp, |(_r, c)| c, 2)
}

#[aoc(day11, part2)]
fn part2(inp: &Vec<(usize, usize)>) -> u64 {
    let mut inp = inp.to_owned();
    add_1d(&mut inp, |(r, _c)| r, 1000000) +
        add_1d(&mut inp, |(_r, c)| c, 1000000)
}
