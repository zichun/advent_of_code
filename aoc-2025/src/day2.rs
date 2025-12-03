use crate::prelude::*;

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<(usize, usize)> {
    input.split(",")
        .map(|ran| {
            let mut tok = ran.split("-");
            (tok.next_token(), tok.next_token())
        })
        .collect()
}

fn invalid_id(a: usize) -> bool {
    let a = a.to_string();
    if a.len() % 2 == 1 {
        false
    } else {
        a.bytes().take(a.len() / 2).eq(a.bytes().skip(a.len() / 2))
    }
}

#[aoc(day2, part1)]
fn part1(inp: &[(usize, usize)]) -> usize {
    inp.iter().map(|(a, b)| {
        (*a..=*b).filter(|&n| invalid_id(n)).sum::<usize>()
    }).sum()
}

fn invalid_id_part2(a: usize) -> bool {
    fn repeated(a: &str, times: usize) -> bool {
        if !a.len().is_multiple_of(times) {
            false
        } else {
            let len = a.len() / times;
            for i in (len..a.len()).step_by(len) {
                let first = a.bytes().take(len);
                if !first.eq(a.bytes().skip(i).take(len)) {
                    return false;
                }
            }
            true
        }
    }

    let a = a.to_string();
    for i in 2..=a.len() {
        if repeated(&a, i) {
            return true;
        }
    }
    false
}

#[aoc(day2, part2)]
fn part2(inp: &[(usize, usize)]) -> usize {
    inp.iter().map(|(a, b)| {
        (*a..=*b).filter(|&n| invalid_id_part2(n)).sum::<usize>()
    }).sum()
}

