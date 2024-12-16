use crate::prelude::*;
use std::{cmp::Ordering, collections::BinaryHeap};

struct Inp {
    map: Grid<char>,
    sr: usize,
    sc: usize,
    er: usize,
    ec: usize
}
#[aoc_generator(day16)]
fn parse(inp: &str) -> Inp {
    let map: Grid<char> = Grid::from_str(inp).unwrap();
    let ((sr, sc), _) = map.iter().filter(|(_, ch)| **ch == 'S').next().unwrap();
    let ((er, ec), _) = map.iter().filter(|(_, ch)| **ch == 'E').next().unwrap();
    Inp { map, sr, sc, er, ec }
}

#[derive(Eq, PartialEq)]
struct Dijk {
    cost: u32,
    r: usize,
    c: usize,
    dir: Direction,
}
impl Ord for Dijk {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.r.cmp(&other.r))
            .then_with(|| self.c.cmp(&other.c))
            .then_with(|| self.dir.ind().cmp(&other.dir.ind()))
    }
}
impl PartialOrd for Dijk {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type FromType = HashMap<(usize, usize, usize), Vec<(usize, usize, usize)>>;
fn solve(inp: &Inp) -> (u32, Vec<Vec<Vec<u32>>>, FromType) {
    let (rr, cc) = inp.map.dimensions();
    let mut dist = vec![vec![vec![u32::MAX ; 4]; cc]; rr];
    let mut from: FromType = HashMap::new();
    let mut heap = BinaryHeap::new();

    fn push_heap(inp: &Inp, heap: &mut BinaryHeap<Dijk>, dist: &mut [Vec<Vec<u32>>], cost: u32, r: usize, c: usize, dir: Direction, from: &mut FromType, fr: usize, fc: usize, fd: Direction) {
        if *inp.map.get(r, c) != '#' && cost < dist[r][c][dir.ind()] {
            dist[r][c][dir.ind()] = cost;
            heap.push(Dijk {
                cost, r, c, dir
            });
            if cost > 0 {
                from.insert((r, c, dir.ind()), vec![(fr, fc, fd.ind())]);
            }
        } else if cost == dist[r][c][dir.ind()] {
            from.get_mut(&(r, c, dir.ind())).unwrap().push((fr, fc, fd.ind()));
        }
    }

    push_heap(inp, &mut heap, &mut dist, 0, inp.sr, inp.sc, Direction::Right, &mut from, 0, 0, Direction::Right);

    let mut min = u32::MAX;
    while let Some(Dijk {
        cost,
        r,
        c,
        dir,
    }) = heap.pop()
    {
        if r == inp.er && c == inp.ec {
            min = min.min(cost);
        }
        if cost > dist[r][c][dir.ind()] {
            continue;
        }
        if let Some((nr, nc)) = inp.map.coord_with_dir(r, c, dir) {
            push_heap(inp, &mut heap, &mut dist, cost + 1, nr, nc, dir, &mut from, r, c, dir);
        }
        push_heap(inp, &mut heap, &mut dist, cost + 1000, r, c, dir.next(), &mut from, r, c, dir);
        push_heap(inp, &mut heap, &mut dist, cost + 1000, r, c, dir.prev(), &mut from, r, c, dir);
    }
    (min, dist, from)
}

#[aoc(day16, part1)]
fn part1(inp: &Inp) -> u32 {
    solve(inp).0
}

#[aoc(day16, part2)]
fn part2(inp: &Inp) -> usize {
    let (min, dist, from) = solve(inp);
    let mut vis = HashSet::new();
    fn recur(vis: &mut HashSet<(usize, usize, usize)>, from: &FromType, r: usize, c: usize, d: usize) {
        if vis.contains(&(r, c, d)) {
            return;
        }
        vis.insert((r, c, d));
        if let Some(v) = from.get(&(r, c, d)) {
            v.iter().for_each(|(r, c, d)| recur(vis, from, *r, *c, *d));
        }
    }
    Direction::iter().for_each(|dir| {
        if dist[inp.er][inp.ec][dir.ind()] == min {
            recur(&mut vis, &from, inp.er, inp.ec, dir.ind());
        }
    });
    vis.into_iter().map(|(r, c, d)| (r, c)).unique().count()
}

