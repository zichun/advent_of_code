use crate::prelude::*;

struct Seq {
    generator: Vec<(i64, i64)>,
}

#[aoc_generator(day9)]
fn parse(inp: &str) -> Vec<Seq> {
    fn process(inp: &[i64], collector: &mut Vec<(i64, i64)>) {
        if inp.iter().all_equal() {
            collector.push((inp[0], inp[0]));
        } else {
            collector.push((inp[0], inp[inp.len() - 1]));
            let diffs = inp
                .iter()
                .tuple_windows()
                .map(|(a, b)| (*b - *a) as i64)
                .collect::<Vec<_>>();
            process(&diffs, collector);
        }
    }
    inp.lines()
        .map(|l| {
            let input = l.parse_tokens::<i64>().collect::<Vec<_>>();
            let mut generator = Vec::new();
            process(&input, &mut generator);
            Seq { generator }
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[Seq]) -> i64 {
    input
        .iter()
        .map(|i| i.generator.iter().map(|(_first, last)| last).sum::<i64>())
        .sum()
}

#[aoc(day9, part2)]
fn part2(input: &[Seq]) -> i64 {
    input
        .iter()
        .map(|i| {
            i.generator
                .iter()
                .map(|(first, _last)| first)
                .rev()
                .fold(0, |acc, el| *el - acc)
        })
        .sum()
}
