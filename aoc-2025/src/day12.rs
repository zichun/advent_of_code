use crate::prelude::*;

#[aoc(day12, part1)]
fn part1(input: &str) -> usize {
    let mut tok = input.split("\n\n").peekable();

    let mut block_cnt = Vec::new();
    loop {
        let next_tok = tok.next().unwrap();
        if tok.peek().is_some() {
            block_cnt.push(next_tok.chars().filter(|&c| c == '#').count());
        } else {
            return next_tok.lines().filter(|line| {
                let mut tok = line.split(": ");

                let dimensions = tok.next_token::<String>();
                let mut dtok = dimensions.split("x");
                let area = dtok.next_token::<usize>() * dtok.next_token::<usize>();

                let cnts = (&tok.next_token::<String>() as &str).extract_tokens::<usize>().collect::<Vec<_>>();
                let required: usize = zip(cnts, &block_cnt).map(|(a, &b)| a * b).sum();
                required <= area
            }).count();
        }
    }

}
