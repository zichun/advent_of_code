use aoc_runner_derive::{aoc_generator, aoc};

use itertools::Itertools;

fn score(press: u64, total: u64) -> u64 {
    (total - press) * press
}
#[aoc(day6, part1)]
fn part1(input: &str) -> u64 {
    let (time, dist) = input.lines()
        .map(|l|
             l.split(" ").skip(1)
             .filter_map(|t| t.parse::<u64>().ok()))
        .collect_tuple().unwrap();
    std::iter::zip(time, dist)
        .map(|(t, d)| {
            (1..t).filter(|x| score(*x, t) > d).count() as u64
        }).product()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> u64 {
    let (t, d) = input.lines()
        .map(|l| l.split(": ").nth(1).unwrap().replace(" ", "").parse::<u64>().unwrap())
        .collect_tuple().unwrap();

    // find peak
    let (mut left, mut right) = (0, t);
    while right > left {
        let left_split = left + (right - left) / 3;
        let right_split = right - (right - left) / 3;

        if score(left_split, t) < score(right_split, t) {
            left = left_split + 1;
        } else {
            right = right_split - 1;
        }
    }

    fn bsearch(t: u64, mut left: u64, mut right: u64, find: u64, desc: bool) -> u64 {
        while right > left {
            let mid = (left + right) / 2;
            let mid_score = score(mid, t);
            if mid_score < find {
                if !desc { left = mid + 1; }
                else { right = mid - 1; }
            } else {
                if !desc { right = mid - 1; }
                else { left = mid + 1; }
            }
        }
        left
    }
    let l = bsearch(t, 0, left, d, false);
    let r = bsearch(t, left, t, d, true);

    r + 1 - l
}
