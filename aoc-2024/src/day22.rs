use crate::prelude::*;

fn next(secret: isize) -> isize {
    let secret = ((secret * 64) ^ secret) % 16777216;
    let secret = ((secret / 32) ^ secret) % 16777216;
    ((secret * 2048) ^ secret) % 16777216
}

#[aoc(day22, part1)]
fn part1(inp: &str) -> isize {
    inp.extract_tokens::<isize>().map(|s| {
        (0..2000).fold(s, |from, _| next(from))
    }).sum()
}

#[aoc(day22, part2)]
fn part2(inp: &str) -> isize {
    let mut map = HashMap::new();
    inp.extract_tokens::<isize>().for_each(|mut s| {
        let or = s;
        let mut hash = 0;
        let mut seen = HashSet::new();
        for i in 0..2000 {
            let nxt = next(s);
            let del = (nxt % 10) - (s % 10);
            hash = hash * 20 + (del + 10);

            if i >= 3 {
                hash %= 160000;
                if !seen.contains(&hash) {
                    *map.entry(hash).or_default() += nxt % 10;
                    seen.insert(hash);
                }
            }

            s = nxt;
        }
    });

    *map.values().max().unwrap()
}
