use crate::prelude::*;
const XMAS: &str = "XMAS";
const MAS: &str = "MAS";

fn cnt_xmas(g: &Grid<char>) -> usize {
    let (rr, cc) = g.dimensions();
    // Horizontal
    let hor_cnt: usize = g.rows().map(|r| {
        let row = r.collect::<String>();
        row.split(XMAS).count() - 1
    }).sum();

    // Diagonal
    let diag_cnt: usize = (0..=rr-XMAS.len()).map(|r|
        (0..=cc-XMAS.len()).map(|c|
            if XMAS.chars().enumerate().all(|(ind, char)| {
                *g.get(r + ind, c + ind) == char
            }) { 1 } else { 0 })
            .sum::<usize>()).sum();
    hor_cnt + diag_cnt
}

fn cnt_cross_mas(g: &Grid<char>) -> usize {
    let (rr, cc) = g.dimensions();
    (0..=rr-MAS.len()).map(|r|
        (0..=cc-MAS.len()).map(|c|
            if MAS.chars().enumerate().all(|(ind, char)| {
                *g.get(r + ind, c + ind) == char &&
                    *g.get(r + MAS.len() - ind - 1, c + ind) == char
            }) { 1 } else { 0 })
            .sum::<usize>()).sum()
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    let g: Grid<char> = Grid::from_str(input).unwrap();
    (0..4).map(|rot| {
        cnt_xmas(&g.rotate(rot))
    })
    .sum()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    let g: Grid<char> = Grid::from_str(input).unwrap();
    (0..4).map(|rot| {
        cnt_cross_mas(&g.rotate(rot))
    })
    .sum()
}
