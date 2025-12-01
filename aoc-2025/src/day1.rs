use crate::prelude::*;

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<(bool, usize)> {
    input
        .lines()
        .map(|s| {
            let mut b = s.bytes();
            let is_left = b.next().unwrap() as char == 'L';
            let num = String::from_utf8(b.collect::<Vec<u8>>()).unwrap().parse::<usize>().unwrap();
            (is_left, num)
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(inp: &[(bool, usize)]) -> u32 {
    let mut start = 50;
    let mut zero_cnt = 0;
    inp.iter().for_each(|(is_left, num)| {
        if *is_left {
            start -= *num as i32;
        } else {
            start += *num as i32;
        }

        while start < 0 {
            start += 100;
        }
        while start > 99 {
            start -= 100;
        }

        if start == 0 {
            zero_cnt += 1;
        }
    });

    zero_cnt
}

#[aoc(day1, part2)]
fn part2(inp: &[(bool, usize)]) -> u32 {
    let mut start = 50;
    let mut zero_cnt = 0;
    inp.iter().for_each(|(is_left, num)| {
        if *is_left {
            start -= *num as i32;
        } else {
            start += *num as i32;
        }

        while start < 0 {
            start += 100;
            zero_cnt += 1;
        }
        while start > 99 {
            start -= 100;
            zero_cnt += 1;
        }
    });

    zero_cnt
}
