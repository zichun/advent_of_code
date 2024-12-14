use std::cmp::Ordering;

use crate::prelude::*;

#[aoc_generator(day5)]
fn parse(input: &str) -> (Graph<u32>, Vec<Vec<u32>>) {
    let mut input = input.split("\n\n");
    let rules = input.next().unwrap();
    let updates = input.next().unwrap();
    let mut g = Graph::new();

    rules.lines().for_each(|el| {
        let mut toks = el.split('|');
        let (from, to) = (
            toks.next().unwrap().parse::<u32>().unwrap(),
            toks.next().unwrap().parse::<u32>().unwrap(),
        );
        g.add_edge(from, to);
    });

    let updates = updates
        .lines()
        .map(|l| {
            l.split(',')
                .map(|tok| tok.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (g, updates)
}

#[aoc(day5, part1)]
fn part1((g, updates): &(Graph<u32>, Vec<Vec<u32>>)) -> u32 {
    updates
        .iter()
        .map(|update| {
            if update.iter().combinations(2).all(|num| {
                if let Some(v) = g.adj_list.get(num[1]) {
                    if v.contains(num[0]) {
                        return false;
                    }
                }
                true
            }) {
                update[update.len() / 2]
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day5, part2)]
fn part2((g, updates): &(Graph<u32>, Vec<Vec<u32>>)) -> u32 {
    updates
        .iter()
        .map(|update| {
            let mut fixed = update.clone();
            fixed.sort_by(|a, b| {
                if g.adj_list[a].contains(b) {
                    Ordering::Less
                } else if g.adj_list[b].contains(a) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            if fixed != *update {
                fixed[fixed.len() / 2]
            } else {
                0
            }
        })
        .sum()
}
