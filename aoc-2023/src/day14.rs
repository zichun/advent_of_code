use crate::prelude::*;

type Grid = crate::prelude::Grid<char>;

#[aoc_generator(day14)]
fn parse(inp: &str) -> Grid {
    Grid::from_str(inp).unwrap()
}

fn tilt(inp: &mut Grid, dir: usize) -> Grid {
    let (mr, mc) = inp.dimensions_with_rot(dir);
    for c in 0..mc {
        let mut dotr = 0;
        for r in 0..mr {
            match inp.get_with_rot(r, c, dir) {
                'O' => {
                    if dotr == r {
                        dotr += 1;
                    } else {
                        while *inp.get_with_rot(dotr, c, dir) != '.' {
                            dotr += 1;
                        }
                        inp.set_with_rot(dotr, c, dir, 'O');
                        inp.set_with_rot(r, c, dir, '.');
                    }
                }
                '#' => {
                    dotr = r + 1;
                }
                _ => (),
            }
        }
    }
    inp.clone()
}

fn north_weight(inp: &Grid) -> u32 {
    let mr = inp.dimensions().0;
    inp.cols()
        .map(|col| {
            col.enumerate()
                .map(|(ind, c)| if *c == 'O' { (mr - ind) as u32 } else { 0 })
                .sum::<u32>()
        })
        .sum::<u32>()
}

#[aoc(day14, part2)]
fn part2(inp: &Grid) -> u32 {
    fn findcycle(weights: &[u32]) -> Option<usize> {
        if weights.len() > 4 {
            for cy in 2..weights.len() / 2 {
                if weights[weights.len() - cy..weights.len()]
                    == weights[weights.len() - cy - cy..weights.len() - cy]
                {
                    return Some(cy);
                }
            }
        }
        None
    }
    let mut weights = Vec::new();
    let mut inp = inp.clone();
    let cy = loop {
        inp = (0..4).fold(inp, |mut acc, el| tilt(&mut acc, el));
        weights.push(north_weight(&inp));
        match findcycle(&weights) {
            Some(cy) => break cy,
            None => (),
        }
    };
    let t = weights.len() - cy - cy + 1;
    weights[(1000000000 - t) % cy + t - 1]
}

#[aoc(day14, part1)]
fn part1(inp: &Grid) -> u32 {
    let mut inp = inp.clone();
    north_weight(&tilt(&mut inp, 0))
}
