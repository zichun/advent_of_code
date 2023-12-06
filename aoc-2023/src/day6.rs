use crate::prelude::*;

fn score(press: u64, total: u64) -> u64 {
    (total - press) * press
}
#[aoc(day6, part1)]
fn part1(input: &str) -> u64 {
    let (time, dist) = input.lines()
        .map(|l|
             l.extract_tokens::<u64>())
        .collect_tuple().unwrap();
    std::iter::zip(time, dist)
        .map(|(t, d)| {
            (1..t).filter(|x| score(*x, t) > d).count() as u64
        }).product()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> u64 {
    let (t, d) = input.lines()
        .map(|l| l
             .split(": ").skip(1).next().unwrap()
             .extract_tokens::<String>().join("").parse::<u64>().unwrap())
        .collect_tuple().unwrap();

    let peak = tsearch(0, t, |l, r| score(l, t) < score(r, t));
    let l = bsearch(0, peak, |x| score(x, t) < d);
    let r = bsearch(peak, t, |x| score(x, t) >= d);

    r + 1 - l
}
