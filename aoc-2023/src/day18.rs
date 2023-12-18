use crate::prelude::*;

#[derive(Clone, Debug)]
struct Instruction {
    dir: Direction,
    steps: usize,
    color: String,
}

#[aoc_generator(day18)]
fn parse(inp: &str) -> Vec<Instruction> {
    inp.lines()
        .map(|l| {
            let mut l = l.split_whitespace();
            Instruction {
                dir: l.next().unwrap().parse().unwrap(),
                steps: l.next_token(),
                color: l.next_token(),
            }
        })
        .collect()
}

#[aoc(day18, part1)]
fn part1(inp: &[Instruction]) -> usize {
    solve(inp)
}

fn solve(inp: &[Instruction]) -> usize {
    let (mut r, mut c) = (0, 0);
    let mut verts = Vec::new();
    let mut hors = Vec::new();
    inp.iter().for_each(|int| {
        let (dr, dc) = int.dir.as_delta();
        let (nr, nc) = (r + dr * int.steps as isize, c + dc * int.steps as isize);
        match int.dir {
            Direction::Down | Direction::Up => verts.push((c, r.min(nr), r.max(nr))),
            Direction::Left | Direction::Right => hors.push((r, c.min(nc), c.max(nc))),
        }
        (r, c) = (nr, nc);
    });

    verts.sort();
    hors.sort();

    let mut area = 0;
    let rows = hors.iter().map(|(r, _, _)| r).copied().unique().collect::<Vec<_>>();

    // calculate in betweens
    rows.iter().tuple_windows().for_each(|(r0, r1)| {
        if r0 != r1 {
            let mut outside = true;
            let mut left = isize::MIN;
            for (c0, top, bot) in verts.iter() {
                if top <= r0 && bot >= r1 {
                    if !outside {
                        area += (c0 - (left + 1)) * (r1 - r0 - 1);
                    }
                    outside = !outside;
                    left = *c0;
                }
            }
        }
    });

    #[derive(Debug, Eq, PartialEq)]
    enum Pt {
        Hor(isize, isize),
        Vert
    }

    rows.iter().for_each(|r| {
        let mut pts = hors.iter().filter(|(r1, _, _)| r1 == r)
            .map(|(_, c0, c1)| (c0, Pt::Hor(*c0, *c1)))
            .chain(
                verts.iter().filter(|(_, top, bot)| top <= r && r < bot)
                    .map(|(c, _, _)| (c, Pt::Vert)))
            .collect::<Vec<_>>();

        pts.sort_by(|a, b| {
            if a.0 == b.0 {
                if b.1 == Pt::Vert {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            } else {
                a.0.cmp(&b.0)
            }
        });
        let mut left = isize::MIN;
        let mut outside = true;
        pts.iter().for_each(|(c, pt)| {
            if !outside && **c > left{
                area += **c - left - 1;
            }
            match pt {
                Pt::Hor(_, c1) => {
                    left = *c1
                },
                Pt::Vert => {
                    left = **c;
                    outside = !outside;
                },
            }
        });
    });

    // add individual segments
    area += hors.iter().map(|(_, left, right)| *right + 1 - *left).sum::<isize>();
    area += verts.iter().map(|(_, top, bot)| *bot - *top - 1).sum::<isize>();

    area as usize
}

#[aoc(day18, part2)]
fn part2(inp: &[Instruction]) -> usize {
    let inp = inp
        .iter()
        .map(|int| Instruction {
            dir: Direction::from_ind(int.color.chars().nth(7).unwrap() as usize - '0' as usize + 1),
            steps: usize::from_str_radix(&int.color[2..=6], 16).unwrap(),
            color: String::new(),
        })
        .collect::<Vec<_>>();
    solve(&inp)
}
