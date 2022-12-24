use std::collections::{HashSet, HashMap, BinaryHeap};

const DT: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
#[derive(Copy, Clone)]
enum Dir {
    Right = 0, Down, Left, Up
}
struct Map {
    r: isize,
    c: isize,
    blizzards_dir: [HashSet<(isize, isize)>; 4]
}
impl Map {
    fn can_go(&mut self, r: isize, c: isize, tick: isize) -> bool {
        let (er, ec) = (self.r - 1, self.c - 2);
        if (r == 0 && c == 1) || (r == er && c == ec) {
            return true;
        }
        if r <= 0 || c <= 0 || r >= self.r - 1 || c >= self.c - 1 {
            return false;
        }
        let (rsize, csize) = (self.r - 2, self.c - 2);
        for dir in 0..4 {
            let (rr, cc) = DT[dir as usize];
            let nr = (r - 1 + (tick * rr)).rem_euclid(rsize) + 1;
            let nc = (c - 1 + (tick * cc)).rem_euclid(csize) + 1;
            if self.blizzards_dir[dir].contains(&(nr, nc)) {
                return false;
            }
        }
        true
    }
}

fn parse(input: &str) -> Map {
    let (mut rmax, mut cmax) = (0, 0);
    let mut blizzards_dir = [HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new()];
    input.lines().enumerate().for_each(|(r, line)| {
        if line.trim() == "" {
            return;
        }
        rmax = rmax.max(r as isize);
        line.chars().enumerate().for_each(|(c, ch)| {
            cmax = cmax.max(c as isize);
            let (r, c) = (r as isize, c as isize);
            match ch {
                '>' => blizzards_dir[2].insert((r, c)),
                'v' => blizzards_dir[3].insert((r, c)),
                '<' => blizzards_dir[0].insert((r, c)),
                '^' => blizzards_dir[1].insert((r, c)),
                _ => false,
            };
        });
    });
    Map {
        r: rmax + 1, c: cmax + 1,
        blizzards_dir,
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    r: isize,
    c: isize,
    dist: isize,
    dist_to_end: isize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_cost = self.dist + self.dist_to_end;
        let other_cost = other.dist + other.dist_to_end;
        other_cost.cmp(&self_cost)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn man_dist(r0: isize, c0: isize, r1: isize, c1: isize) -> isize {
    (r0 - r1).abs() + (c0 - c1).abs()
}

fn astar(map: &mut Map, sr: isize, sc: isize, er: isize, ec: isize, start_dist: isize) -> isize {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    heap.push(State {
        r: sr, c: sc, dist: start_dist, dist_to_end: man_dist(er, ec, sr, sc)
    });
    fn push(heap: &mut BinaryHeap<State>, visited: &mut HashSet<(isize, isize, isize)>, r: isize, c: isize, dist: isize, er: isize, ec: isize) {
        if !visited.contains(&(r, c, dist)) {
            visited.insert((r, c, dist));
            heap.push(State {
                r, c, dist,
                dist_to_end: man_dist(er, ec, r, c)
            });
        }
    }

    while !heap.is_empty() {
        let state = heap.pop().unwrap();
        for (rr, cc) in DT.iter() {
            let (r, c) = (state.r + *rr, state.c + *cc);
            if r == er && c == ec {
                return state.dist + 1;
            }
            if map.can_go(r, c, state.dist + 1) {
                push(&mut heap, &mut visited, r, c, state.dist + 1, er, ec);
            }
        };
        // wait
        if map.can_go(state.r, state.c, state.dist + 1) {
            push(&mut heap, &mut visited, state.r, state.c, state.dist + 1, er, ec);
        }
    }
    unreachable!()
}

pub fn part1(input: &str) -> isize {
    let mut map = parse(input);
    let (er, ec) = (map.r - 1, map.c - 2);
    astar(&mut map, 0, 1, er, ec, 0)
}

pub fn part2(input: &str) -> isize {
    let mut map = parse(input);
    let (er, ec) = (map.r - 1, map.c - 2);
    let one = astar(&mut map, 0, 1, er, ec, 0);
    let two = astar(&mut map, er, ec, 0, 1, one);
    astar(&mut map, 0, 1, er, ec, two)
}

#[test]
fn test() {
    let input = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
    assert_eq!(part1(input), 18);
    assert_eq!(part2(input), 54);
}
