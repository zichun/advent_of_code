use crate::prelude::*;

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<(u64, u64)> {
    input.lines().map(|l| {
        let mut tok = l.split(",");
        (tok.next_token(), tok.next_token())
    }).collect()
}

fn area(a: &(u64, u64), b: &(u64, u64)) -> u64 {
    (a.0.abs_diff(b.0) + 1) *
        (a.1.abs_diff(b.1) + 1)
}

#[aoc(day9, part1)]
fn part1(inp: &[(u64, u64)]) -> u64 {
    inp.iter().combinations(2).map(|a| area(a[0], a[1])).max().unwrap()
}

fn normalize(a: &(u64, u64), b: &(u64, u64)) -> ((u64, u64), (u64, u64)) {
    ((a.0.min(b.0), a.1.min(b.1)),
     (a.0.max(b.0), a.1.max(b.1)))
}

fn valid(inp: &[(u64, u64)], a: &(u64, u64), b: &(u64, u64)) -> bool {
    let (a, b) = normalize(a, b);
    inp.iter().tuple_windows().all(|(red0, red1)| {
        let (red0, red1) = normalize(red0, red1);
        !(a.0 < red1.0 && b.0 > red0.0 && a.1 < red1.1 && b.1 > red0.1)
    })
}

#[aoc(day9, part2)]
fn part2(inp: &[(u64, u64)]) -> u64 {
    let mut loopinp = inp.to_vec();
    loopinp.push(loopinp[0]);

    inp.iter().combinations(2).filter_map(|a| {
        if valid(&loopinp, a[0], a[1]) {
            Some(area(a[0], a[1]))
        } else {
            None
        }
    }).max().unwrap()
}
