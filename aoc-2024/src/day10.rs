use crate::prelude::*;

#[aoc(day10, part1)]
fn part1(inp: &str) -> usize {
    let g: Grid<char> = Grid::from_str(inp).unwrap();

    let mut pos_map = HashMap::new();
    for ((r, c), ch) in g.iter() {
        pos_map.entry(ch.to_digit(10).unwrap()).or_insert(Vec::new()).push((r, c));
    }

    let (rr, cc) = g.dimensions();
    let mut reachable = vec![vec![HashSet::new(); cc]; rr];
    (0..=9).rev().for_each(|height| {
        pos_map.entry(height).or_insert(Vec::new()).iter().for_each(|(r, c)| {
            if height == 9 {
                reachable[*r][*c].insert((*r, *c));
            } else {
                g.reachables(*r, *c, Direction::iter())
                    .for_each(|(nr, nc)| {
                        if g.get(nr, nc).to_digit(10).unwrap() == height + 1 {
                            reachable[nr][nc].iter().cloned().collect::<Vec<_>>()
                                .into_iter().for_each(|(nr, nc)| {
                                    reachable[*r][*c].insert((nr, nc));
                                });
                        }
                    });
            }
        });
    });

    pos_map.get(&0).unwrap().iter().map(|(r, c)| reachable[*r][*c].len()).sum()
}

#[aoc(day10, part2)]
fn part2(inp: &str) -> usize {
    let g: Grid<char> = Grid::from_str(inp).unwrap();

    let mut pos_map = HashMap::new();
    for ((r, c), ch) in g.iter() {
        pos_map.entry(ch.to_digit(10).unwrap()).or_insert(Vec::new()).push((r, c));
    }

    let (rr, cc) = g.dimensions();
    let mut reachable = vec![vec![0; cc]; rr];
    (0..=9).rev().map(|height| {
        pos_map.entry(height).or_insert(Vec::new()).iter().map(|(r, c)| {
            if height == 9 {
                reachable[*r][*c] = 1;
            } else {
                g.reachables(*r, *c, Direction::iter())
                    .for_each(|(nr, nc)| {
                        if g.get(nr, nc).to_digit(10).unwrap() == height + 1 {
                        reachable[*r][*c] += reachable[nr][nc];
                    }
                });
            }
            reachable[*r][*c]
        }).sum()
    }).last().unwrap()

}
