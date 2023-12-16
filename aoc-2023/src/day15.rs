use crate::prelude::*;

#[aoc_generator(day15)]
fn parse(inp: &str) -> Vec<String> {
    inp.split(",").map(|x| x.to_owned()).collect()
}

fn hash(inp: &str) -> u32 {
    inp.chars()
        .fold(0, |acc, el| (17 * (acc + (el as u32))) % 256)
}

#[aoc(day15, part1)]
fn part1(inp: &[String]) -> u32 {
    inp.iter().map(|s| hash(s)).sum()
}

enum Label {
    Eq(String, u32),
    Rm(String),
}

#[aoc(day15, part2)]
fn part2(inp: &[String]) -> u32 {
    let mut boxes = vec![Vec::<(String, u32)>::new(); 256];
    inp.iter()
        .map(|s| {
            if s.contains('=') {
                let mut s = s.split('=');
                Label::Eq(s.next_token(), s.next_token())
            } else {
                let mut s = s.split('-');
                Label::Rm(s.next_token())
            }
        })
        .for_each(|l| match l {
            Label::Eq(s, foc) => {
                let h = hash(&s);
                if let Some(len) = boxes[h as usize].iter_mut().find(|(len_s, _)| len_s == &s) {
                    len.1 = foc;
                } else {
                    boxes[h as usize].push((s, foc))
                }
            }
            Label::Rm(s) => {
                let h = hash(&s);
                boxes[h as usize] = boxes[h as usize]
                    .iter()
                    .filter(|(len_s, _)| len_s != &s)
                    .map(|t| t.to_owned())
                    .collect();
            }
        });
    boxes
        .iter()
        .enumerate()
        .map(|(box_num, b)| {
            b.iter()
                .enumerate()
                .map(|(len_num, (_, foc))| (box_num as u32 + 1) * (len_num as u32 + 1) * *foc)
                .sum::<u32>()
        })
        .sum::<u32>()
}
