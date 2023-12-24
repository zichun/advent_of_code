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
    let ec = grid.row(mr - 1).enumerate().find(|(c, ch)| **ch == '.').unwrap().0;

    let mut q = VecDeque::new();
    let mut visited = vec![vec![false; mc]; mr];
    q.push_back((0, sc));

    fn flood(grid: &Grid, visited: &mut Vec<Vec<bool>>, r: usize, c: usize, steps: usize, interesting: &mut Vec<(usize, usize, usize)>, prer: usize, prec: usize) {
        let first = prer == r && prec == c;
        let mut stop = false;
        if visited[r][c] && !first {
            stop = true;
        }
        visited[r][c] = true;

        let dirs = Direction::iter().filter(|dir| {
            if let Some((nr, nc)) = grid.coord_with_dir(r, c, *dir) {
                *grid.get(nr, nc) != '#' && (nr != prer || nc != prec)
            } else {
                false
            }
        }).collect::<Vec<_>>();

        if first || dirs.len() == 1 {
            dirs.into_iter().for_each(|dir| {
                let (nr, nc) = grid.coord_with_dir(r, c, dir).unwrap();
                if !stop {
                    flood(grid, visited, nr, nc, steps + 1, interesting, r, c);
                }
            });
        } else if dirs.len() > 1 || r + 1 == grid.dimensions().0 {
            interesting.push((r, c, steps));
        }
    }

    type GraphType = HashMap<usize, Vec<(usize, usize)>>;
    let mut graph: GraphType = HashMap::new();
    let mut vcnt = 0;
    let mut dest = 0;
    let mut vertices = HashMap::new();

    while let Some((r, c)) = q.pop_front() {
        vertices.entry((r, c)).or_insert_with(|| { vcnt += 1; vcnt - 1 });
        // inner flood
        let mut interesting = Vec::new();
        flood(&grid, &mut visited, r, c, 0, &mut interesting, r, c);
        interesting.into_iter().for_each(|(nr, nc, steps)| {
            vertices.entry((nr, nc)).or_insert_with(|| { vcnt += 1; vcnt - 1 });
            let (from, to) = (vertices[&(r, c)], vertices[&(nr, nc)]);
            if nr + 1 == mr {
                dest = to;
            }
            graph.entry(from).or_default().push((to, steps));
            graph.entry(to).or_default().push((from, steps));
            q.push_back((nr, nc));
        });
    }

    let mut cache = HashMap::new();
    fn dfs(grid: &Grid, graph: &GraphType, cache: &mut HashMap<usize, Vec<(usize, usize)>>, visited: usize, cur: usize, tsteps: usize, dest: usize) -> usize {
        let mut tr = 0;
        if cur == dest {
            return tsteps;
        }

        graph[&cur].iter().for_each(|(next, steps)| {
            if visited & (1 << *next) == 0 {
                tr = tr.max(dfs(grid, graph, cache, visited | (1 << *next), *next, tsteps + *steps, dest));
            }
        });
        tr
    }

    dfs(&grid, &graph, &mut cache, 1, 0, 0, dest)
}
