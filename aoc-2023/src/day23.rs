use std::{collections::BinaryHeap, sync::Arc};

use crate::prelude::*;

type Grid = crate::prelude::Grid<char>;

#[aoc_generator(day23)]
fn parse(inp: &str) -> Grid {
    Grid::from_str(inp).unwrap()
}

fn char_to_dir(c: char) -> Option<Direction> {
    match c {
        '>' => Some(Direction::Right),
        '<' => Some(Direction::Left),
        'v' => Some(Direction::Down),
        '^' => Some(Direction::Up),
        _ => None
    }
}

#[aoc(day23, part1)]
fn part1(grid: &Grid) -> usize {
    let (mr, mc) = grid.dimensions();

    let mut q = Vec::new();
    let mut visited = vec![vec![0; mc]; mr];

    let sc = grid.row(0).enumerate().find(|(_, ch)| **ch == '.').unwrap().0;
    q.push((0, sc));

    fn flood(grid: &Grid, q: &mut Vec<(usize, usize)>, visited: &mut Vec<Vec<usize>>, r: usize, c: usize, steps: usize) {
        if visited[r][c] > 0 {
            return;
        }
        visited[r][c] = steps;

        match char_to_dir(*grid.get(r, c)) {
            Some(dir) => {
                let (nr, nc) = grid.coord_with_dir(r, c, dir).unwrap();
                assert_eq!(*grid.get(nr, nc), '.');
                q.push((nr, nc));
            }
            None => {
                for dir in Direction::iter() {
                    if let Some((nr, nc)) = grid.coord_with_dir(r, c, dir) {
                        if *grid.get(nr, nc) == '#' {
                            continue;
                        }
                        flood(grid, q, visited, nr, nc, steps + 1);
                    }
                }
            }
        }
    }

    fn get_incomings(grid: &Grid, visited: &Vec<Vec<usize>>, r: usize, c: usize) -> Vec<Option<usize>> {
        Direction::iter().map(|dir| {
            match grid.coord_with_dir(r, c, dir) {
                Some((nr, nc)) => {
                    match char_to_dir(*grid.get(nr, nc)) {
                        Some(pdir) => if pdir == dir.opp() {
                            if visited[nr][nc] > 0 {
                                Some(visited[nr][nc])
                            } else {
                                None
                            }
                        } else {
                            Some(0)
                        },
                        None => Some(0),
                    }
                },
                None => Some(0),
            }
        }).collect::<Vec<_>>()
    }

    while !q.is_empty() {
        let found = q.iter().enumerate().find(|(ind, (r, c))| {
            let interesting_incoming = get_incomings(&grid, &visited, *r, *c);

            interesting_incoming.iter().find(|incoming| incoming.is_none()).is_none()
        }).map(|(ind, (r, c))| (ind, *r, *c));

        let (r, c) = match found {
            Some((ind, r, c)) => {
                q.remove(ind);
                (r, c)
            },
            None => q.pop().unwrap()
        };
        let interesting_incoming = get_incomings(&grid, &visited, r, c);
        let cur = match interesting_incoming.into_iter().filter_map(|inc| inc).max() {
            Some(m) => m + 1,
            None => 1,
        };
        flood(&grid, &mut q, &mut visited, r, c, cur);
    }

    let ec = grid.row(mr - 1).enumerate().find(|(c, ch)| **ch == '.').unwrap().0;
    let tr = visited[mr - 1][ec] - 1;
    tr
}

#[aoc(day23, part2)]
fn part2(grid: &Grid) -> usize {
    let (mr, mc) = grid.dimensions();
    let sc = grid.row(0).enumerate().find(|(_, ch)| **ch == '.').unwrap().0;

    let mut q = VecDeque::new();
    let mut visited = vec![vec![false; mc]; mr];
    q.push_back((0, sc));

    fn flood(grid: &Grid, visited: &mut Vec<Vec<bool>>, r: usize, c: usize, steps: usize, interesting: &mut Vec<(usize, usize, usize)>, first: bool) {
        if visited[r][c] && !first {
            return;
        }
        visited[r][c] = true;

        let dirs = Direction::iter().filter(|dir| {
            if let Some((nr, nc)) = grid.coord_with_dir(r, c, *dir) {
                *grid.get(nr, nc) != '#' && !visited[nr][nc]
            } else {
                false
            }
        }).collect::<Vec<_>>();

        if first || dirs.len() == 1 {
            dirs.into_iter().for_each(|dir| {
                let (nr, nc) = grid.coord_with_dir(r, c, dir).unwrap();
                flood(grid, visited, nr, nc, steps + 1, interesting, false);
            });
        } else if dirs.len() > 1 {
            interesting.push((r, c, steps));
        }
    }

    let mut graph: HashMap<(usize, usize), Vec<(usize, usize, usize)>> = HashMap::new();

    while let Some((r, c)) = q.pop_front() {
        // inner flood
        let mut interesting = Vec::new();
        println!("flooding {} {}",r, c);
        flood(&grid, &mut visited, r, c, 0, &mut interesting, true);
        interesting.into_iter().for_each(|(nr, nc, steps)| {
            graph.entry((r, c)).or_default().push((nr, nc, steps));
            graph.entry((nr, nc)).or_default().push((r, c, steps));
            q.push_back((nr, nc));
        });
    }
    println!("{:?}", graph);

    let ec = grid.row(mr - 1).enumerate().find(|(c, ch)| **ch == '.').unwrap().0;
    0
}
