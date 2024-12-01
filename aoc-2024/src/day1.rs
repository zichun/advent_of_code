use crate::prelude::*;

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input.lines()
        .fold((Vec::new(), Vec::new()), |(mut v0, mut v1), el| {
            let mut tok = el.split_ascii_whitespace();
            v0.push(tok.next().unwrap().parse::<u32>().unwrap());
            v1.push(tok.next().unwrap().parse::<u32>().unwrap());
            (v0, v1)
        })
}

#[aoc(day1, part1)]
fn part1(input: &str) -> u32 {
    let (mut l0, mut l1) = parse(input);
    l0.sort();
    l1.sort();
    zip(l0, l1).map(|(a, b)| a.abs_diff(b)).sum()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
    let (l0, l1) = parse(input);
    let cnt_map: HashMap<u32, u32> = l1.iter()
        .fold(HashMap::new(), |mut acc, el| {
            *acc.entry(*el).or_insert(0) += 1;
            acc
        });

    l0.iter().map(|el| *el * cnt_map.get(el).unwrap_or(&0)).sum()
}
