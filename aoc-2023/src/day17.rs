use crate::prelude::*;
use std::collections::BinaryHeap;

type Grid = crate::prelude::Grid<u32>;

#[aoc_generator(day17)]
fn parse(inp: &str) -> Grid {
    Grid::from_str(inp).unwrap()
}

#[derive(Eq, PartialEq)]
struct Dijk {
    cost: u32,
    r: usize,
    c: usize,
    dir: Direction,
    times: usize,
}
impl Ord for Dijk {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.r.cmp(&other.r))
            .then_with(|| self.c.cmp(&other.c))
            .then_with(|| self.dir.ind().cmp(&other.dir.ind()))
            .then_with(|| self.times.cmp(&other.times))
    }
}
impl PartialOrd for Dijk {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn solve<const MINTURN: usize, const MAXTURN: usize>(g: &Grid) -> u32 {
    let (mr, mc) = g.dimensions();
    let mut dist = vec![vec![vec![vec![u32::MAX; MAXTURN + 1]; 4]; mc]; mr];
    let mut heap = BinaryHeap::new();

    dist[0][0][Direction::Right.ind()][1] = 0;
    heap.push(Dijk {
        cost: 0,
        r: 0,
        c: 0,
        dir: Direction::Right,
        times: 0,
    });
    heap.push(Dijk {
        cost: 0,
        r: 0,
        c: 0,
        dir: Direction::Down,
        times: 0,
    });

    let mut ans = u32::MAX;

    while let Some(Dijk {
        cost,
        r,
        c,
        dir,
        times,
    }) = heap.pop()
    {
        if r == mr - 1 && c == mc - 1 {
            if MINTURN == 0 || (MINTURN > 0 && times >= MINTURN) {
                ans = ans.min(cost);
            }
        }
        let mut new_dirs = Direction::iter()
            .filter(|d| *d != dir.opp() && *d != dir)
            .collect::<Vec<_>>();
        if times < MAXTURN {
            new_dirs.push(dir);
        }
        if times < MINTURN {
            new_dirs = vec![dir];
        }

        new_dirs.iter().for_each(|nd| {
            if let Some((nr, nc)) = g.coord_with_dir(r, c, *nd) {
                let new_cost = cost + g.get(nr, nc);
                let times = if *nd == dir { times + 1 } else { 1 };
                if new_cost < dist[nr][nc][nd.ind()][times] {
                    dist[nr][nc][nd.ind()][times] = new_cost;
                    heap.push(Dijk {
                        cost: new_cost,
                        r: nr,
                        c: nc,
                        dir: *nd,
                        times,
                    });
                }
            }
        });
    }
    ans
}

#[aoc(day17, part1)]
fn part1(g: &Grid) -> u32 {
    solve::<1, 3>(g)
}

#[aoc(day17, part2)]
fn part2(g: &Grid) -> u32 {
    solve::<4, 10>(g)
}
