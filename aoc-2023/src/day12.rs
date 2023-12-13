use crate::prelude::*;

#[derive(Clone)]
struct Input {
    pattern: Vec<char>,
    damages: Vec<u64>,
}

#[aoc_generator(day12)]
fn parse(inp: &str) -> Vec<Input> {
    inp.lines()
        .map(|l| {
            let mut l = l.split(" ");
            Input {
                pattern: l.next().unwrap().to_owned().chars().collect(),
                damages: l
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect(),
            }
        })
        .collect()
}

fn solve(inp: &Input) -> u64 {
    type MemoType = Vec<Vec<Option<u64>>>;

    fn f(inp: &Input, i: usize, j: usize, memo: &mut MemoType) -> u64 {
        if i >= inp.pattern.len() && j >= inp.damages.len() {
            return 1;
        } else if i >= inp.pattern.len() {
            return 0;
        } else if let Some(memoed) = memo[i][j] {
            return memoed;
        }

        fn eat(inp: &Input, mut i: usize, j: usize, memo: &mut MemoType) -> u64 {
            if j >= inp.damages.len() {
                return 0;
            }
            let mut to_eat = inp.damages[j];
            while to_eat > 0 && i < inp.pattern.len() {
                if inp.pattern[i] == '.' {
                    return 0;
                }
                i += 1;
                to_eat -= 1;
            }
            if to_eat > 0 || (i < inp.pattern.len() && inp.pattern[i] == '#') {
                return 0;
            }
            f(inp, i + 1, j + 1, memo)
        }
        let tr = match inp.pattern[i] {
            '.' => f(inp, i + 1, j, memo),
            '#' => eat(inp, i, j, memo),
            '?' => eat(inp, i, j, memo) + f(inp, i + 1, j, memo),
            _ => unreachable!(),
        };
        memo[i][j] = Some(tr);
        tr
    }

    let mut memo: MemoType = vec![vec![None; inp.damages.len() + 1]; inp.pattern.len() + 1];
    f(inp, 0, 0, &mut memo)
}

#[aoc(day12, part1)]
fn part1(inp: &[Input]) -> u64 {
    inp.iter().map(|inp| solve(&inp)).sum()
}

#[aoc(day12, part2)]
fn part2(inp: &[Input]) -> u64 {
    inp.iter()
        .map(|inp| {
            let mut pat = inp.pattern.clone();
            pat.push('?');
            let inp = Input {
                pattern: pat
                    .iter()
                    .cycle()
                    .take(pat.len() * 5 - 1)
                    .map(|l| *l)
                    .collect(),
                damages: inp
                    .damages
                    .iter()
                    .cycle()
                    .take(inp.damages.len() * 5)
                    .map(|l| *l)
                    .collect(),
            };
            solve(&inp)
        })
        .sum()
}
