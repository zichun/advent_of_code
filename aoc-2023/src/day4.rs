use aoc_runner_derive::{aoc_generator, aoc};
use std::collections::{VecDeque, HashSet};

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
    fn parse_card(s: &str) -> HashSet<u32> {
        s.split(" ").filter_map(|t| t.trim().parse::<u32>().ok()).collect()
    }
    input.split("\n")
        .map(|line| {
            let mut line = line.split(": ");
            let _ = line.next().unwrap();
            let mut line = line.next().unwrap().split(" | ");
            Card {
                win: parse_card(line.next().unwrap()),
                draw: parse_card(line.next().unwrap())
            }
        }).collect()
}

#[aoc(day4, part1)]
fn part1(input: &[Card]) -> u32 {
    input.iter().map(|c| {
        let win = c.winning();
        if win > 0 { 1 << (win - 1) }
        else { 0 }
    }).sum::<usize>() as u32
}

#[aoc(day4, part2)]
fn part2(input: &[Card]) -> u32 {
    let mut q = VecDeque::new();
    input.iter().map(|c| {
        let win = c.winning();
        let cards = if q.len() > 0 {
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
    }).sum::<usize>() as u32
}