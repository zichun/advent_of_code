use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Inp {
    a: (isize, isize),
    b: (isize, isize),
    target: (isize, isize),
}

#[aoc_generator(day13)]
fn parse(inp: &str) -> Vec<Inp> {
    fn parse_inner(l: &str, token: &str) -> (isize, isize) {
        let mut l = l.split(": ").nth(1).unwrap().split(", ");
        let x = l
            .next()
            .unwrap()
            .split(token)
            .nth(1)
            .unwrap()
            .parse::<isize>()
            .unwrap();
        let y = l
            .next()
            .unwrap()
            .split(token)
            .nth(1)
            .unwrap()
            .parse::<isize>()
            .unwrap();
        (x, y)
    }
    inp.split("\n\n")
        .map(|l| {
            let mut lines = l.lines();
            let a = parse_inner(lines.next().unwrap(), "+");
            let b = parse_inner(lines.next().unwrap(), "+");
            let target = parse_inner(lines.next().unwrap(), "=");
            Inp { a, b, target }
        })
        .collect()
}

#[aoc(day13, part1)]
fn part1(inp: &[Inp]) -> isize {
    inp.iter().map(|inp| solve(inp, 0)).sum()
}

#[aoc(day13, part2)]
fn part2(inp: &[Inp]) -> isize {
    inp.iter().map(|inp| solve(inp, 10000000000000)).sum()
}

fn solve(inp: &Inp, offset: isize) -> isize {
    let (target_x, target_y) = (inp.target.0 + offset, inp.target.1 + offset);
    let (top, bot) = (
        target_y * inp.a.0 - target_x * inp.a.1,
        inp.b.1 * inp.a.0 - inp.b.0 * inp.a.1,
    );
    if bot == 0 || top % bot != 0 {
        0
    } else {
        let b = (target_y * inp.a.0 - target_x * inp.a.1) / (inp.b.1 * inp.a.0 - inp.b.0 * inp.a.1);
        if (target_x - b * inp.b.0) % inp.a.0 != 0 {
            0
        } else {
            3 * (target_x - b * inp.b.0) / inp.a.0 + b
        }
    }
}
