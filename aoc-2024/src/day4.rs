use crate::prelude::*;
const XMAS: &str = "XMAS";
const MAS: &str = "MAS";

fn cnt_xmas(g: &Grid<char>) -> usize {
    let (rr, cc) = g.dimensions();
    (0..rr)
        .map(|r| {
            (0..=cc-XMAS.len()).map(|c|
            // diagonal
            if r <= rr - XMAS.len() && XMAS.chars().enumerate().all(|(ind, char)|
                *g.get(r + ind, c + ind) == char
            ) { 1 } else { 0 } +
            // horizontal
            XMAS.chars().enumerate().all(|(ind, char)|
                *g.get(r, c + ind) == char
            ).then_some(1).unwrap_or_default()
                )
            .sum::<usize>()
        })
        .sum()
}

fn cnt_cross_mas(g: &Grid<char>) -> usize {
    let (rr, cc) = g.dimensions();
    (0..=rr - MAS.len())
        .map(|r| {
            (0..=cc - MAS.len())
                .map(|c| {
                    MAS.chars().enumerate().all(|(ind, char)| {
                        *g.get(r + ind, c + ind) == char
                            && *g.get(r + MAS.len() - ind - 1, c + ind) == char
                    }).then_some(1).unwrap_or_default()
                })
                .sum::<usize>()
        })
        .sum()
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    let g: Grid<char> = Grid::from_str(input).unwrap();
    (0..4).map(|rot| cnt_xmas(&g.rotate(rot))).sum()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    let g: Grid<char> = Grid::from_str(input).unwrap();
    (0..4).map(|rot| cnt_cross_mas(&g.rotate(rot))).sum()
}
