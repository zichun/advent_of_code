use crate::prelude::*;

struct Input {
    ranges: Vec<(usize, usize)>,
    ids: Vec<usize>,
}

#[aoc_generator(day5)]
fn parse(inp: &str) -> Input {
    let mut tok = inp.split("\n\n");
    let mut ranges: Vec<(usize, usize)> = tok.next().unwrap().lines().map(|l| {
        let mut tok = l.split("-");
        (tok.next_token(), tok.next_token())
    }).collect();
    let ids = tok.next().unwrap().lines().map(|ids| ids.parse::<usize>().unwrap()).collect();

    ranges.sort_by(|&a, &b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });

    Input {
        ranges,
        ids
    }
}

#[aoc(day5, part1)]
fn part1(inp: &Input) -> usize {
    let mut ids = inp.ids.clone();
    ids.sort();

    let mut i = 0;
    ids.iter().filter(|&&id| {
        while id > inp.ranges[i].1 && i + 1 < inp.ranges.len() {
            i += 1;
        }

        id >= inp.ranges[i].0 && id <= inp.ranges[i].1
    }).count()
}

#[aoc(day5, part2)]
fn part2(inp: &Input) -> usize {
    inp.ranges.iter().fold((0, 0), |(cnt, most_right), &(mut left, right)| {
        left = left.max(most_right + 1);
        let new_cnt = if right + 1 >= left {
            cnt + right + 1 - left
        } else {
            cnt
        };
        (new_cnt, most_right.max(right))
    }).0
}
