use std::collections::{HashMap, VecDeque};

static ROCKS: &'static str = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##";

type Rock = Vec<(i32, i32)>;
fn parse_rock(rocks: &str) -> Vec<Rock> {
    rocks.split("\n\n")
        .map(|r| {
            let mut tr = Vec::new();
            r.lines().rev().enumerate()
                .for_each(|(r, l)|
                          l.chars().enumerate().filter(|(_, c)| *c == '#').for_each(|(c, _)| {
                              tr.push((r as i32, c as i32));
                          }));
            tr
        })
        .collect()
}

type IsLeft = bool;
struct Tetris<'a> {
    map: VecDeque<[bool; 7]>,
    wind: &'a mut dyn Iterator<Item = (usize, &'a IsLeft)>,
    rock_ind: usize,
    rocks: &'a [Rock],
    wind_len: usize,
    last_wind_ind: usize,
    cache: HashMap<(usize, usize, u64), (usize, usize)>
}

fn absolute_rock(rock: &Rock, r: usize, c: usize) -> Rock {
    rock.iter().map(|(rr, cc)| (rr + r as i32, cc + c as i32)).collect()
}

impl<'a> Tetris<'a> {
    fn new(wind: &'a mut impl Iterator<Item = (usize, &'a IsLeft)>, rocks: &'a[Rock], wind_len: usize) -> Self {
        Tetris {
            map: VecDeque::new(),
            wind,
            rock_ind: 0,
            rocks,
            wind_len,
            last_wind_ind: 0,
            cache: HashMap::new()
        }
    }

    fn conflict(&self, rock: &Rock, r: usize, c: usize) -> bool {
        absolute_rock(rock, r, c).iter().filter(|(r, c)| {
            if *c < 0 || *c >= 7 {
                true
            } else if (*r as usize) < self.map.len() {
                self.map[*r as usize][*c as usize]
            } else {
                false
            }
        }).count() > 0
    }
    fn blow(&mut self, rock: &Rock, r: usize, c: i32) -> i32 {
        let (wind, is_left) = self.wind.next().unwrap();
        self.last_wind_ind = wind;
        match *is_left {
            true => if c <= 0 || self.conflict(rock, r, c as usize - 1) { c } else { c - 1 },
            false => if self.conflict(rock, r, c as usize + 1) { c } else { c + 1 }
        }
    }
    fn place_rock(&mut self, rock: &Rock, r: usize, c: usize) {
        absolute_rock(rock, r, c).iter()
            .for_each(|(r, c)| {
                let (r, c) = (*r as usize, *c as usize);
                while r >= self.map.len() {
                    self.map.push_back([false; 7]);
                }
                assert_eq!(self.map[r][c], false);
                self.map[r][c] = true;
            });
    }
    fn compress_top(&self, layers: usize) -> u64 {
        let h = self.map.len();
        assert!(layers <= h);
        let mut tr = 0;
        for l in 0..layers {
            let lc = self.map[h - 1 - l].iter()
                .fold(0u32, |acc, c|
                      (acc + if *c { 1 } else { 0 }) << 1) as u64;
            tr = (tr * 24804727 + lc) % 512069826173;
        }
        tr
    }
    fn has_repeat(&mut self) -> Option<(usize, usize)> {
        let wind = (self.last_wind_ind + 1) % self.wind_len;
        let rind = self.rock_ind;
        let top_7 = self.compress_top(5);
        self.cache.get(&(wind, rind, top_7)).copied()
    }
    fn drop_rock(&mut self, ind: usize) {
        let rock = &self.rocks[self.rock_ind];
        self.rock_ind = (self.rock_ind + 1) % self.rocks.len();

        let (mut r, mut c) = (self.map.len() + 3, 2);
        loop {
            c = self.blow(rock, r, c as i32) as usize;
            if r == 0 || self.conflict(&rock, r - 1, c) {
                self.place_rock(rock, r, c);
                break;
            } else {
                r -= 1;
            }
        }

        if self.map.len() > 10 {
            let rind = self.rock_ind;
            let top_7 = self.compress_top(5);
            self.cache.insert((self.last_wind_ind, rind, top_7), (self.map.len(), ind));
        }
    }
    fn debug(&self) {
        self.map.iter().enumerate().rev().for_each(|(ind, l)| {
            println!("{}: {}", ind, l.iter().map(|b| if *b { '#' } else { '.' }).collect::<String>());
        });
    }
}

fn parse(input: &str) -> Vec<IsLeft> {
    input.trim().chars().map(|c| c == '<').collect()
}

pub fn part1(input: &str) -> usize {
    let rocks = parse_rock(ROCKS);
    let wind_vec = parse(input);
    let mut wind = wind_vec.iter().enumerate().cycle();
    let mut tetris = Tetris::new(&mut wind, &rocks, wind_vec.len());

    (0..2022).for_each(|ind| {
        tetris.drop_rock(ind);
    });
    tetris.map.len()
}

pub fn part2(input: &str) -> u64 {
    let rocks = parse_rock(ROCKS);
    let wind_vec = parse(input);
    let mut wind = wind_vec.iter().enumerate().cycle();
    let mut tetris = Tetris::new(&mut wind, &rocks, wind_vec.len());

    const target: usize = 1000000000000;
    for ind in 0..target {
        tetris.drop_rock(ind);

        if ind > 1000 {
            if let Some((ph, piter)) = tetris.has_repeat() {
                todo!();
                break;
            }
        }
    };
    0
}

#[test]
fn test() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(part1(input), 3068);
    assert_eq!(part2(input), 1514285714288);
}
