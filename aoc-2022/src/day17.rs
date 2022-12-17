use std::collections::VecDeque;

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
    rocks: &'a [Rock]
}

fn absolute_rock(rock: &Rock, r: usize, c: usize) -> Rock {
    rock.iter().map(|(rr, cc)| (rr + r as i32, cc + c as i32)).collect()
}

impl<'a> Tetris<'a> {
    fn new(wind: &'a mut impl Iterator<Item = (usize, &'a IsLeft)>, rocks: &'a[Rock]) -> Self {
        Tetris {
            map: VecDeque::new(),
            wind,
            rock_ind: 0,
            rocks
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
        match *(self.wind.next().unwrap().1) {
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
    fn drop_rock(&mut self) {
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
    let mut tetris = Tetris::new(&mut wind, &rocks);

    (0..2022).for_each(|_| {
        tetris.drop_rock();
    });
    tetris.map.len()
}

pub fn part2(input: &str) -> u32 {
    0
}

#[test]
fn test() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(part1(input), 3068);
}
