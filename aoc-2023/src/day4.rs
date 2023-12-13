use crate::prelude::*;

struct Card {
    win: HashSet<u32>,
    draw: HashSet<u32>,
}

impl Card {
    fn winning(&self) -> usize {
        self.draw.iter().filter(|k| self.win.contains(k)).count()
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Card> {
    input
        .split('\n')
        .map(|line| {
            let mut line = line.split(": ");
            let _ = line.next().unwrap();
            let mut line = line.next().unwrap().split(" | ");
            Card {
                win: line.next().unwrap().extract_tokens::<u32>().collect(),
                draw: line.next().unwrap().extract_tokens::<u32>().collect(),
            }
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &[Card]) -> u32 {
    input
        .iter()
        .map(|c| {
            let win = c.winning();
            if win > 0 {
                1 << (win - 1)
            } else {
                0
            }
        })
        .sum::<usize>() as u32
}

#[aoc(day4, part2)]
fn part2(input: &[Card]) -> u32 {
    let mut q = VecDeque::new();
    input
        .iter()
        .map(|c| {
            let win = c.winning();
            let cards = if !q.is_empty() {
                q.pop_front().unwrap() + 1
            } else {
                1
            };
            for i in 0..win {
                if q.len() > i {
                    q[i] += cards;
                } else {
                    q.push_back(cards);
                }
            }
            cards
        })
        .sum::<usize>() as u32
}
