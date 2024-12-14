use crate::prelude::*;

struct Robot {
    sr: isize,
    sc: isize,
    vr: isize,
    vc: isize,
}
#[aoc_generator(day14)]
fn parse(inp: &str) -> Vec<Robot> {
    inp.lines()
        .map(|l| {
            let mut tok = l
                .split(&['p', 'v', '=', ',', ' '])
                .filter(|s| !s.is_empty());
            let (sc, sr, vc, vr) = (
                tok.next().unwrap().parse::<isize>().unwrap(),
                tok.next().unwrap().parse::<isize>().unwrap(),
                tok.next().unwrap().parse::<isize>().unwrap(),
                tok.next().unwrap().parse::<isize>().unwrap(),
            );
            Robot { sr, sc, vr, vc }
        })
        .collect()
}

fn solve(inp: &[Robot], ticks: isize, rr: isize, cc: isize) -> isize {
    let mut quad = [[0; 2]; 2];

    inp.iter().for_each(|ro| {
        let (nr, nc) = (
            (ro.sr + ticks * ro.vr).rem_euclid(rr),
            (ro.sc + ticks * ro.vc).rem_euclid(cc),
        );

        if nr * 2 + 1 != rr && nc * 2 + 1 != cc {
            let rind = if nr < rr / 2 { 0 } else { 1 };
            let cind = if nc < cc / 2 { 0 } else { 1 };
            quad[rind][cind] += 1;
        }
    });
    quad[0][0] * quad[0][1] * quad[1][0] * quad[1][1]
}

#[aoc(day14, part1)]
fn part1(inp: &[Robot]) -> isize {
    solve(inp, 100, 103, 101)
}

fn simulate(inp: &[Robot], rr: isize, cc: isize) -> isize {
    for ticks in 1.. {
        let robots = inp
            .iter()
            .map(|ro| {
                (
                    (ro.sr + ticks * ro.vr).rem_euclid(rr),
                    (ro.sc + ticks * ro.vc).rem_euclid(cc),
                )
            })
            .collect::<HashSet<(isize, isize)>>();

        let cnt_diag = robots
            .iter()
            .filter(|(r, c)| {
                let (r, c) = (*r, *c);
                (robots.contains(&(r + 1, c + 1)) && robots.contains(&(r - 1, c - 1)))
                    || (robots.contains(&(r - 1, c + 1)) && robots.contains(&(r + 1, c - 1)))
            })
            .count();

        if cnt_diag > inp.len() / 3 {
            println!("{}: ", ticks);
            for r in 0..rr {
                for c in 0..cc {
                    print!("{}", if robots.contains(&(r, c)) { "*" } else { "." });
                }
                println!();
            }
            return ticks;
        }
    }
    unreachable!()
}

#[aoc(day14, part2)]
fn part2(inp: &[Robot]) -> isize {
    simulate(inp, 103, 101)
}
