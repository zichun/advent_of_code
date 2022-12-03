use itertools::Itertools;
use std::collections::HashSet;

fn score(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        (c as u32) - ('a' as u32) + 1
    } else {
        26 + (c as u32) - ('A' as u32) + 1
    }
}

pub fn part1(input: &str) -> u32 {
    input.lines()
        .map(|line| {
            let len = line.len();
            (line.chars().take(len / 2).collect::<HashSet<_>>(),
             line.chars().skip(len / 2).collect::<HashSet<_>>())
        })
        .map(|(left, right)| {
            let dup = left.iter()
                .filter(|&x| right.contains(x))
                .next()
                .unwrap();
            score(*dup)
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    fn to_hash(opt_str: Option<&str>) -> HashSet<char> {
        opt_str.unwrap().chars().collect::<HashSet<_>>()
    }

    input.lines()
        .batching(|it| {
            let (a, b, c) = (it.next(), it.next(), it.next());
            if a.is_none() {
                None
            } else {
                Some((to_hash(a),
                      to_hash(b),
                      to_hash(c)))
            }
        })
        .map(|(h0, h1, h2)| {
            let dup = h0.iter()
                .filter(|&c| h1.contains(c))
                .filter(|&c| h2.contains(c))
                .next().unwrap();
            score(*dup)
        }).sum()
}
#[test]
fn test() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    assert_eq!(part1(input), 157);
    assert_eq!(part2(input), 70);
}
