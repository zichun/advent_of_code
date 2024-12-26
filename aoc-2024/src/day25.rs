use crate::prelude::*;

#[aoc(day25, part1)]
fn part1(inp: &str) -> usize {
    let (mut keys, mut locks) = (Vec::new(), Vec::new());
    inp.split("\n\n").for_each(|d| {
        let g = Grid::<char>::from_str(d).unwrap();
        if *g.get(0, 0) == '.' {
            keys.push(g.cols().map(|col| {
                col.enumerate().find(|(_r, ch)| {
                    **ch == '#'
                }).unwrap().0
            }).collect::<Vec<_>>());
        } else {
            locks.push(g.cols().map(|col| {
                col.enumerate().find(|(_r, ch)| {
                    **ch == '.'
                }).unwrap().0
            }).collect::<Vec<_>>());
        }
    });

    keys.iter().map(|k| {
        locks.iter().filter(|l| zip(*l, k).all(|(l, k)| k >= l)).count()
    }).sum()
}
