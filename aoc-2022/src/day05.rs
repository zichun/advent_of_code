use regex::Regex;
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug)]
struct Hanoi(Vec<VecDeque<char>>);

impl Hanoi {
    fn mve(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.move_cnt {
            let c = self.0[instruction.from - 1].pop_back().unwrap();
            self.0[instruction.to - 1].push_back(c);
        }
    }
    fn mve_part2(&mut self, instruction: &Instruction) {
        let mut de = VecDeque::new();
        for _ in 0..instruction.move_cnt {
            let c = self.0[instruction.from - 1].pop_back().unwrap();
            de.push_back(c);
        }
        de.iter().rev().for_each(|c| {
            self.0[instruction.to - 1].push_back(*c);
        });
    }
    fn top(&self) -> String {
        self.0.iter()
            .map(|stack| stack.back().unwrap() )
            .collect::<String>()
    }
}

struct Instruction {
    move_cnt: usize,
    from: usize,
    to: usize,
}

fn parse(input: &str) -> (Hanoi, Vec<Instruction>) {
    let mut iter = input.split("\r\n\r\n");

    let hanoi = input
        .lines()
        .take_while(|l| {
            l.trim().len() > 0
        }).filter_map(|l| {
            let mut row = Vec::new();
            let mut i = 1;
            let len = l.len();
            while i < len {
                row.push(match l.as_bytes()[i] as char {
                    ' ' => None,
                    c => Some(c),
                });
                i += 4;
            }
            if row.len() > 0 {
                Some(row)
            } else {
                None
            }
        }).fold(Vec::new(), |mut acc, el| {
            el.iter().enumerate().for_each(|(ind, c)| {
                if ind >= acc.len() {
                    acc.push(VecDeque::new());
                }
                if let Some(c) = c {
                    acc[ind].push_front(*c);
                }
            });
            acc
        });

    let instructions: Vec<_> = input
        .lines()
        .filter_map(|l| {
            let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
            match re.captures(l) {
                None => None,
                Some(cap) =>
                    Some(Instruction {
                        move_cnt: cap[1].parse().unwrap(),
                        from: cap[2].parse().unwrap(),
                        to: cap[3].parse().unwrap(),
                    })
            }
        }).collect();

    (Hanoi(hanoi), instructions)
}

pub fn part1(input: &str) -> String {
    let (mut hanoi, instructions) = parse(input);
    instructions
        .iter()
        .for_each(|int| hanoi.mve(int));

    hanoi.top()
}

pub fn part2(input: &str) -> String {
    let (mut hanoi, instructions) = parse(input);
    instructions
        .iter()
        .for_each(|int| hanoi.mve_part2(int));

    hanoi.top()
}

#[test]
fn test() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    assert_eq!(part1(input), "CMZ".to_owned());
    assert_eq!(part2(input), "MCD".to_owned());
}
