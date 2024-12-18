use std::cell::RefCell;

use crate::prelude::*;

#[aoc_generator(day18)]
fn parse(inp: &str) -> Vec<(usize, usize)> {
    inp.lines().map(|l| {
        let mut sp = l.split(',');
        let (x, y) = (sp.next_token(), sp.next_token());
        (y, x)
    }).collect()
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum El {
    Space,
    Wall
}

fn solve(inp: &[(usize, usize)], rr: usize, cc: usize, take: usize) -> Option<usize> {
    let mut g = Grid::<El>::new(rr + 1, cc + 1, El::Space);
    inp.iter().take(take).for_each(|(r, c)| {
        g.set(*r, *c, El::Wall);
    });

    let mut bfs = Bfs::<(usize, usize)>::new();
    bfs.visit((0, 0), 0);

    while let Some(((r, c), dist)) = bfs.pop() {
        if r == rr && c == cc {
            return Some(dist);
        }
        g.reachables(r, c, Direction::iter()).for_each(|(rr, cc)| {
            if *g.get(rr, cc) != El::Wall {
                bfs.visit((rr, cc), dist + 1);
            }
        });
    }
    None
}

#[aoc(day18, part1)]
fn part1(inp: &[(usize, usize)]) -> usize {
    solve(inp, 70, 70, 1024).unwrap()
}

#[aoc(day18, part2)]
fn part2(inp: &[(usize, usize)]) -> String {
    let ind = bsearch(0, inp.len() - 1, |take| solve(inp, 70, 70, take).is_some());
    format!("{},{}", inp[ind].1, inp[ind].0)
}
