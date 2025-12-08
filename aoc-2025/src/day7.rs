use crate::prelude::*;

#[aoc_generator(day7)]
fn parse(inp: &str) -> Grid<char> {
    Grid::from_str(inp).unwrap()
}

#[aoc(day7, part1)]
fn part1(g: &Grid<char>) -> usize {
    let mut drops = vec![false; g.dimensions().1];
    drops[g.row(0)
        .enumerate()
        .find(|&(_ind, ch)| *ch == 'S')
        .map(|(ind, _)| ind).unwrap()
    ] = true;

    let mut tr = 0;
    g.iter().for_each(|((_r, c), &ch)| {
        if ch == '^' && drops[c] {
                drops[c - 1] = true; // assume splitter is never at edges
                drops[c + 1] = true;
                drops[c] = false;
                tr += 1;
            }
    });

    tr
}

#[aoc(day7, part2)]
fn part2(g: &Grid<char>) -> u64 {
    let mut drops = vec![0u64; g.dimensions().1];
    drops[g.row(0)
        .enumerate()
        .find(|&(_ind, ch)| *ch == 'S')
        .map(|(ind, _)| ind).unwrap()
    ] = 1;

    let mut tr = 0;
    g.iter().for_each(|((_r, c), &ch)| {
        if ch == '^' && drops[c] > 0 {
                drops[c - 1] += drops[c]; // assume splitter is never at edges
                drops[c + 1] += drops[c];
                drops[c] = 0;
                tr += 1;
            }
    });

    drops.iter().sum()
}
