use std::collections::{HashSet, HashMap};

struct Simulation {
    elves: HashSet<(isize, isize)>,
    dir_offset: usize
}

/*
If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step.
*/
const DT: [[(isize, isize); 3]; 4] = [[(-1, -1), (-1, 0), (-1, 1)],
                                      [(1, -1), (1, 0), (1, 1)],
                                      [(-1, -1), (0, -1), (1, -1)],
                                      [(-1, 1), (0, 1), (1, 1)]];
impl Simulation {
    fn tick(&mut self) -> bool {
        let mut map = HashMap::<(isize, isize), Vec<(isize, isize)>>::new();
        let mut mv_cnt = 0;
        self.elves.iter().for_each(|(r, c)| {
            let mut has_neighbor = false;
            'outer: for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    if self.elves.contains(&(*r + dr, *c + dc)) {
                        has_neighbor = true;
                        break 'outer;
                    }
                }
            }
            if has_neighbor {
                for dir in 0..4 {
                    let dt = &DT[(dir + self.dir_offset) % 4];
                    let mut has = false;
                    for i in 0..3 {
                        let (rr, cc) = (*r + dt[i].0, *c + dt[i].1);
                        if self.elves.contains(&(rr, cc)) {
                            has = true;
                            break;
                        }
                    }
                    if !has {
                        mv_cnt += 1;
                        map.entry((*r + dt[1].0, *c + dt[1].1)).or_default()
                            .push((*r, *c));
                        return;
                    }
                }
            }
            map.entry((*r, *c)).or_default()
                .push((*r, *c));
        });
        self.elves = map.iter().flat_map(|(to, from)| {
            if from.len() == 1 {
                vec![*to]
            } else {
                mv_cnt -= 1;
                from.to_owned()
            }
        }).collect::<HashSet<_>>();
        self.dir_offset = (self.dir_offset + 1) % 4;
        mv_cnt > 0
    }
    fn print_elves(&self) {
        let ((r_min, r_max), (c_min, c_max)) = self.get_bounds();
        for r in r_min..=r_max {
            for c in c_min..=c_max {
                if self.elves.contains(&(r, c)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("");
    }
    fn get_bounds(&self) -> ((isize, isize), (isize, isize)) {
        let (r_min, r_max) = (
            self.elves.iter().map(|(r, _)| *r).min().unwrap(),
            self.elves.iter().map(|(r, _)| *r).max().unwrap());
        let (c_min, c_max) = (
            self.elves.iter().map(|(_, c)| *c).min().unwrap(),
            self.elves.iter().map(|(_, c)| *c).max().unwrap());
        ((r_min, r_max), (c_min, c_max))
    }
}

fn parse(input: &str) -> Simulation {
    let elves = input.lines().enumerate().flat_map(|(r, l)| {
        l.chars().enumerate().filter_map(|(c, ch)| {
            if ch == '#' {
                Some((r as isize, c as isize))
            } else {
                None
            }
        }).collect::<Vec<_>>()
    }).collect();

    Simulation {
        elves,
        dir_offset: 0
    }
}


pub fn part1(input: &str) -> usize {
    let mut sim = parse(input);
    for _ in 0..10 {
        sim.tick();
    }
    let ((r_min, r_max), (c_min, c_max)) = sim.get_bounds();
    ((r_max + 1 - r_min).abs() * (c_max + 1 - c_min).abs()) as usize -
        sim.elves.len()
}
pub fn part2(input: &str) -> usize {
    let mut sim = parse(input);
    for rnd in 0.. {
        if !sim.tick() {
            return rnd + 1;
        }
    }
    0
}

#[test]
fn test() {
    let input = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
    assert_eq!(part1(input), 110);
    assert_eq!(part2(input), 20);
}
