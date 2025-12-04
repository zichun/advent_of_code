use crate::prelude::*;

#[aoc_generator(day4)]
fn parse(input: &str) -> Grid<char> {
    Grid::from_str(input).unwrap()
}

#[aoc(day4, part1)]
fn part1(g: &Grid<char>) -> usize {
    g.iter().filter(|((r, c), ch)| {
        **ch == '@' &&
            g.reachables(*r, *c, DirectionWithDiag::iter())
                .filter(|(r, c)| *g.get(*r, *c) == '@')
                .count() < 4
    })
    .count()

}
#[aoc(day4, part2)]
fn part2(g: &Grid<char>) -> usize {
    let mut g = g.clone();
    let mut tr = 0;
    loop {
        let rem = g.iter()
            .filter(|((r, c), ch)| {
                **ch == '@' &&
                    g.reachables(*r, *c, DirectionWithDiag::iter())
                        .filter(|(r, c)| *g.get(*r, *c) == '@')
                        .count() < 4
            })
            .map(|((r, c), _ch)| (r, c))
            .collect::<Vec<(usize, usize)>>();
        if rem.is_empty() {
            break;
        }
        tr += rem.len();
        rem.iter().for_each(|(r, c)| g.set(*r, *c, '.'));
    }
    tr
}
