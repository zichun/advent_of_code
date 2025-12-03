use crate::prelude::*;

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Vec<usize>> {
    input.lines()
        .map(|l| l.bytes().map(|b| (b - b'0') as usize)
            .collect()
        ).collect()
}

fn top_k(bank: &[usize], k: usize) -> usize {
    let mut st = Vec::new();
    let n = bank.len();

    bank.iter().enumerate().for_each(|(i, &bi)| {
        while !st.is_empty() && bi > st[st.len() - 1] && n - i + st.len() > k {
            st.pop();
        }
        if st.len() < k {
            st.push(bi);
        }
    });

    st.iter().fold(0, |acc, el| acc * 10 + *el)
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<usize>]) -> usize {
    input.iter().map(|bank| {
        top_k(bank, 2)
    }).sum()
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<usize>]) -> usize {
    input.iter().map(|bank| {
        top_k(bank, 12)
    }).sum()
}
