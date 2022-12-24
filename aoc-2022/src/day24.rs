use std::collections::{HashSet, HashMap, BinaryHeap};

const DT: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
#[derive(Copy, Clone)]
enum Dir {
    Right = 0, Down, Left, Up
}
struct Map {
    r: isize,
    c: isize,
    blizzards: Vec<(isize, isize, Dir)>,
    cache_blizzards: HashMap<isize, HashSet<(isize, isize)>>,
}
impl Map {
    fn get_blizzard_at_tick(&mut self, tick: isize) -> &HashSet<(isize, isize)> {
        let (rsize, csize) = (self.r - 2, self.c - 2);
        self.cache_blizzards.entry(tick).or_insert_with(|| {
            self.blizzards.iter().map(|(r, c, dir)| {
                let (rr, cc) = DT[*dir as usize];
                let nr = (*r - 1 + (tick * rr)).rem_euclid(rsize) + 1;
                let nc = (*c - 1 + (tick * cc)).rem_euclid(csize) + 1;
                (nr, nc)
            }).collect::<HashSet<_>>()
        })
    }
    fn can_go(&mut self, r: isize, c: isize, tick: isize) -> bool {
        if r == 0 && c == 1 {
            return true;
        }
        if r <= 0 || c <= 0 || r >= self.r - 1 || c >= self.c - 1 {
            return false;
        }
        !self.get_blizzard_at_tick(tick).contains(&(r, c))
    }
    fn print_at_tick(&mut self, tick: isize) {
        let (max_r, max_c) = (self.r, self.c);
        let nb = self.get_blizzard_at_tick(tick);
        for r in 0..max_r {
            for c in 0..max_c {
                if r == 0 || c == 0 || r == max_r - 1 || c == max_c - 1 {
                    print!("#");
                } else if nb.contains(&(r, c)) {
                    print!("*");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

fn parse(input: &str) -> Map {
    let (mut rmax, mut cmax) = (0, 0);
    let blizzards = input.lines().enumerate().flat_map(|(r, line)| {
        if line.trim() == "" {
            return vec![];
        }
        rmax = rmax.max(r as isize);
        line.chars().enumerate().filter_map(|(c, ch)| {
            cmax = cmax.max(c as isize);
            let (r, c) = (r as isize, c as isize);
            match ch {
                '>' => Some((r, c, Dir::Right)),
                'v' => Some((r, c, Dir::Down)),
                '<' => Some((r, c, Dir::Left)),
                '^' => Some((r, c, Dir::Up)),
                _ => None
            }
        }).collect::<Vec<_>>()
    }).collect();
    Map {
        r: rmax + 1, c: cmax + 1,
        blizzards,
        cache_blizzards: HashMap::new(),
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

pub fn part1(input: &str) -> isize {
    let mut map = parse(input);
    let mut heap = BinaryHeap::new();
    let (er, ec) = (map.r - 1, map.c - 2);
    let mut visited = HashSet::new();

    heap.push(State {
        r: 0, c: 1, dist: 0, dist_to_end: man_dist(er, ec, 0, 1)
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
                map.print_at_tick(state.dist);
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
    panic!("cant find a path");
}

pub fn part2(input: &str) -> isize {
    0
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
}
