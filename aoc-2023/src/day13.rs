use std::{iter::zip, str::FromStr};

use crate::prelude::*;

#[aoc_generator(day13)]
fn parse(inp: &str) -> Vec<Grid<char>> {
    inp.split("\n\n").map(|inp| Grid::from_str(inp).unwrap()).collect()
}

fn reflection(g: &Grid<char>, smudge: usize) -> u32 {
    fn find_col_reflection(g: &Grid<char>, smudge: usize) -> u32 {
        let mc = g.dimensions().1;
        'c: for c in 1..mc {
            let mut diff = 0;
            for c2 in 0..mc / 2 {
                if c2 + 1 > c || c + c2 >= mc {
                    break;
                }
                diff += zip(g.col(c - c2 - 1), g.col(c + c2))
                    .filter(|(a, b)| a != b)
                    .count();
                if diff > smudge {
                    continue 'c;
                }
            }
            if diff == smudge {
                return c as u32;
            }
        }
        0
    }
    let c = find_col_reflection(g, smudge);
    if c == 0 {
        100 * find_col_reflection(&g.transpose(), smudge)
    } else {
        c
    }
}

#[aoc(day13, part1)]
fn part1(inp: &[Grid<char>]) -> u32 {
    inp.iter().map(|g| reflection(g, 0)).sum()
}

#[aoc(day13, part2)]
fn part2(inp: &[Grid<char>]) -> u32 {
    inp.iter().map(|g| reflection(g, 1)).sum()
}
