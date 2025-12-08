use crate::prelude::*;

#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<(u64, u64, u64)> {
    input.lines().map(|l| {
        let mut tok = l.split(",");
        (tok.next_token(), tok.next_token(), tok.next_token())
    }).collect()
}

fn dist(pt0: &(u64, u64, u64), pt1: &(u64, u64, u64)) -> u64 {
    (pt0.0 - pt1.0) * (pt0.0 - pt1.0) +
    (pt0.1 - pt1.1) * (pt0.1 - pt1.1) +
    (pt0.2 - pt1.2) * (pt0.2 - pt1.2)
}

struct DisjointSet {
    set: Vec<usize>,
    set_cnt: usize
}
impl DisjointSet {
    fn new(size: usize) -> DisjointSet {
        DisjointSet {
            set: (0..size).map(|i| i).collect::<Vec<_>>(),
            set_cnt: size
        }
    }
    fn get_set(&mut self, a: usize) -> usize {
        if self.set[a] == a {
            a
        } else {
            self.set[a] = self.get_set(self.set[a]);
            self.set[a]
        }
    }
    fn union(&mut self, a: usize, b: usize) {
        let a_parent = self.get_set(a);
        let b_parent = self.get_set(b);
        self.set[a_parent] = b_parent;

        if a_parent != b_parent {
            self.set_cnt -= 1;
        }
    }
    fn set_cnt(&self) {
        self.set_cnt;
    }
}

#[aoc(day8, part1)]
fn part1(pts: &[(u64, u64, u64)]) -> u64 {
    let mut dists = Vec::new();
    for i in 0..pts.len() {
        for j in (i+1)..pts.len() {
            dists.push((dist(&pts[i], &pts[j]), i, j));
        }
    }

    let mut set = DisjointSet::new(pts.len());

    dists.sort();
    for i in 0..1000 {
        set.union(dists[i].1, dists[i].2);
    }

    let mut setcnter = HashMap::new();
    for i in 0..pts.len() {
        let parent = set.get_set(i);
        *setcnter.entry(parent).or_insert(0) += 1;
    }

    let mut circuits_size = setcnter.iter().map(|(_, b)| *b).collect::<Vec<_>>();
    circuits_size.sort();
    circuits_size.iter().rev().take(3).product()
}

#[aoc(day8, part2)]
fn part2(pts: &[(u64, u64, u64)]) -> u64 {
    let mut dists = Vec::new();
    for i in 0..pts.len() {
        for j in (i+1)..pts.len() {
            dists.push((dist(&pts[i], &pts[j]), i, j));
        }
    }

    let mut set = DisjointSet::new(pts.len());

    dists.sort();
    for i in 0..dists.len() {
        set.union(dists[i].1, dists[i].2);
        if set.set_cnt == 1 {
            return pts[dists[i].1].0 * pts[dists[i].2].0;
        }
    }

    unreachable!()
}
