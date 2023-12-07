use crate::prelude::*;

#[derive(Clone, Eq, PartialEq, Ord)]
struct Card {
    // hand is representated by a tuple:
    //   - sorted cardinalities of values, e.g 2 pairs would be [2, 2, 1] and full house would be [3, 2]
    //   - list of original value, e.g T55J5 would be [10, 5, 5, 11, 5]
    hand: (Vec<usize>, Vec<u32>),
    bid: u32,
}
impl Card {
    fn process_joker(&mut self) {
        let mut jokers = 0;
        self.hand.1.iter_mut().for_each(|x| {
            if *x == 11 {
                *x = 1;
                jokers += 1;
            }
        });
        let mut cardinalities = self
            .hand
            .0
            .iter()
            .fold((false, Vec::new()), |(found, mut v), el| {
                if found || !found && *el != jokers {
                    v.push(*el);
                    (found, v)
                } else {
                    (true, v)
                }
            })
            .1;
        if cardinalities.len() > 0 {
            cardinalities[0] += jokers;
        } else {
            cardinalities.push(jokers);
        }
        self.hand.0 = cardinalities;
    }
}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        for (asize, bsize) in std::iter::zip(&self.hand.0, &other.hand.0) {
            if asize != bsize {
                return Some(asize.cmp(&bsize));
            }
        }
        for (acard, bcard) in std::iter::zip(&self.hand.1, &other.hand.1) {
            if acard != bcard {
                return Some(acard.cmp(&bcard));
            }
        }
        unreachable!()
    }
}

#[aoc_generator(day7)]
fn parse(inp: &str) -> Vec<Card> {
    fn parse_hand(cards: &str) -> (Vec<usize>, Vec<u32>) {
        let cards: Vec<u32> = cards
            .chars()
            .map(|c| match c {
                '2'..='9' => (c as u32) - '0' as u32,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!(),
            })
            .collect();
        let mut cnt: Vec<usize> = cards
            .iter()
            .counts()
            .into_iter()
            .map(|(_, cnt)| cnt)
            .collect();
        cnt.sort_by(|a, b| b.cmp(a));
        (cnt, cards)
    }
    inp.lines()
        .map(|l| {
            let mut l = l.split(" ");
            Card {
                hand: parse_hand(&l.next_token::<String>()),
                bid: l.next_token(),
            }
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &[Card]) -> u32 {
    let mut cards = input.to_vec();
    cards.sort();
    cards
        .iter()
        .enumerate()
        .map(|(ind, c)| (ind as u32 + 1) * c.bid)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[Card]) -> u32 {
    let mut cards = input.to_vec();
    cards.iter_mut().for_each(|c| c.process_joker());
    cards.sort();
    cards
        .iter()
        .enumerate()
        .map(|(ind, c)| (ind as u32 + 1) * c.bid)
        .sum()
}
