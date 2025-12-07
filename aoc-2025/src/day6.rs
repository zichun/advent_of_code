use crate::prelude::*;

#[aoc(day6, part1)]
fn part1(inp: &str) -> u64 {
    let mut nums: Vec<Vec<u64>> = Vec::new();
    let mut ops = Vec::new();

    inp.lines().for_each(|l| {
        let firstchar =  l.chars().next().unwrap();
        if firstchar == '*' || firstchar == '+' { // last line
            ops = l.parse_tokens::<char>().collect();
        } else {
            nums.push(l.parse_tokens::<u64>().collect());
        }

    });

    ops.iter().enumerate().map(|(i, &op)| {
        nums.iter().fold(if op == '+' { 0 } else { 1 }, |acc, el| {
            if op == '+' {
                acc + el[i]
            } else {
                acc * el[i]
            }
        })
    }).sum()
}

#[aoc(day6, part2)]
fn part2(inp: &str) -> u64 {
    let mut ops: Vec<(char, usize)> = Vec::new();

    let vertnums = inp.lines().fold(Vec::new(), |mut acc, line| {
        let firstchar =  line.chars().next().unwrap();
        if firstchar == '*' || firstchar == '+' { // last line
            ops = line.chars().enumerate().filter_map(|(ind, ch)| {
                if ch != ' ' {
                    Some((ch, ind))
                } else {
                    None
                }
            }).collect();
            return acc;
        }

        acc.resize(line.len(), 0);
        zip(
            acc.iter(),
            line.chars()
                .map(|c| if c == ' ' { 0 } else { c as u8 - b'0' })
        ).map(|(&ori, new)| {
            if new == 0 {
                ori
            } else {
                ori * 10 + (new as u64)
            }
        }).collect()
    });

    ops.iter().map(|&(op, ind)| {
        vertnums.iter()
            .skip(ind)
            .take_while(|&&num| num > 0)
            .fold(if op == '+' { 0 } else { 1 }, |acc, &el| {
                if op == '+' {
                    acc + el
                } else {
                    acc * el
                }
            })
    }).sum()
}
