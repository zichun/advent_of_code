use std::str::FromStr;

use crate::prelude::*;

type Grid = crate::grid::Grid<char>;

#[aoc_generator(day16)]
fn parse(inp: &str) -> Grid {
    Grid::from_str(inp).unwrap()
}

fn energized(g: &Grid, r: usize, c: usize, dir: Direction) -> usize {
    let (mr, mc) = g.dimensions();

    let mut q = VecDeque::new();
    let mut visited = vec![vec![vec![false; 4]; mc]; mr];
    let mut visited_g = vec![vec![false; mc]; mr];

    visited[r][c][dir.ind()] = true;
    q.push_back((r, c, dir));

    fn add(
        g: &Grid,
        q: &mut VecDeque<(usize, usize, Direction)>,
        visited: &mut Vec<Vec<Vec<bool>>>,
        r: usize,
        c: usize,
        dir: Direction,
    ) {
        if let Some((r, c)) = g.coord_with_dir(r, c, dir) {
            if !visited[r][c][dir.ind()] {
                visited[r][c][dir.ind()] = true;
                q.push_back((r, c, dir));
            }
        }
    }
    let mut cnt = 0;
    while !q.is_empty() {
        let (r, c, dir) = q.pop_front().unwrap();
        if !visited_g[r][c] {
            cnt += 1;
        }
        visited_g[r][c] = true;

        match g.get(r, c) {
            '.' => add(g, &mut q, &mut visited, r, c, dir),
            '|' => {
                if dir.is_updown() {
                    add(g, &mut q, &mut visited, r, c, dir)
                } else {
                    add(g, &mut q, &mut visited, r, c, dir.next());
                    add(g, &mut q, &mut visited, r, c, dir.prev());
                }
            }
            '-' => {
                if dir.is_leftright() {
                    add(g, &mut q, &mut visited, r, c, dir)
                } else {
                    add(g, &mut q, &mut visited, r, c, dir.next());
                    add(g, &mut q, &mut visited, r, c, dir.prev());
                }
            }
            '/' => {
                let next_dir = match dir {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                };
                add(g, &mut q, &mut visited, r, c, next_dir);
            }
            '\\' => {
                let next_dir = match dir {
                    Direction::Up => Direction::Left,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                };
                add(g, &mut q, &mut visited, r, c, next_dir);
            }
            _ => unreachable!(),
        }
    }
    cnt
}

#[aoc(day16, part1)]
fn part1(g: &Grid) -> usize {
    energized(g, 0, 0, Direction::Right)
}

#[aoc(day16, part2)]
fn part2(g: &Grid) -> usize {
    let (mr, mc) = g.dimensions();

    (0..mc)
        .map(|c| energized(g, 0, c, Direction::Down).max(energized(g, mr - 1, c, Direction::Up)))
        .max()
        .unwrap()
        .max(
            (0..mr)
                .map(|r| {
                    energized(g, r, 0, Direction::Right).max(energized(
                        g,
                        r,
                        mc - 1,
                        Direction::Left,
                    ))
                })
                .max()
                .unwrap(),
        )
}
