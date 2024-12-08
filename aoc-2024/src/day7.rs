use crate::prelude::*;

struct Inp {
    target: i64,
    lst: Vec<i64>
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Inp> {
    input.lines().map(|l| {
        let mut tok = l.split(": ");
        let (target, mut lst) = (tok.next().unwrap().parse().unwrap(),
                             tok.next().unwrap().extract_tokens().collect::<Vec<i64>>());
        lst.reverse();
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
    fn undo(&self, last: i64, el: i64) -> Option<i64> {
        match self {
            Op::Add => Some(last - el),
            Op::Mul => if last % el == 0 { Some(last / el) } else { None },
            Op::Cct => {
                let (last_s, el_s) = (last.to_string(), el.to_string());
                if last_s.len() > el_s.len() && last_s.ends_with(&el_s) {
                    Some(last / i64::pow(10, el_s.len() as u32))
                } else {
                    None
                }
            },
        }
    }
}

fn solvable(target: i64, lst: &[i64], ops: &[Op]) -> bool {
    if lst.len() == 1  && target == lst[0] {
        return true;
    }
    ops.iter().any(|op| if let Some(nxt) = op.undo(target, lst[0]) {
        solvable(nxt, &lst[1..lst.len()], ops)
    } else {
        false
    })
}
fn solve(inp: &[Inp], ops: &[Op]) -> i64{
    inp.iter().filter_map(|Inp { target, lst }| {
        if solvable(*target, lst, ops) {
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
