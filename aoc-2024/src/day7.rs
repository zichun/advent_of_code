use crate::prelude::*;

struct Inp {
    target: i64,
    lst: Vec<i64>
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Inp> {
    input.lines().map(|l| {
        let mut tok = l.split(": ");
        let (target, lst) = (tok.next().unwrap().parse().unwrap(),
                             tok.next().unwrap().extract_tokens().collect());
        Inp { target, lst }
    }).collect()
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
    Cct,
}

impl Op {
    fn eval(&self, l: i64, r: i64) -> i64 {
        match self {
            Op::Add => l + r,
            Op::Mul => l * r,
            Op::Cct =>
                l * i64::pow(10, r.checked_ilog10().unwrap_or(0) + 1) + r,
        }
    }
}

fn solve(inp: &[Inp], ops: &[Op]) -> i64{
    inp.iter().filter_map(|Inp { target, lst }| {
        if repeat_n(ops, lst.len() - 1).multi_cartesian_product()
            .any(|ops| {
                let mut lst = lst.iter();
                let first = lst.next().unwrap();
                zip(lst, ops).fold(*first, |acc, (nxt, op)| {
                    op.eval(acc, *nxt)
                }) == *target
            }) {
                Some(*target)
            } else {
                None
            }
    }).sum()
}

#[aoc(day7, part1)]
fn part1(inp: &[Inp]) -> i64 {
    solve(inp, &[Op::Add, Op::Mul])
}

#[aoc(day7, part2)]
fn part2(inp: &[Inp]) -> i64 {
    solve(inp, &[Op::Add, Op::Mul, Op::Cct])
}
