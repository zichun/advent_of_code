use crate::prelude::*;

fn recur(num: u64, times: usize, memo: &mut HashMap<(u64, usize), usize>) -> usize {
    let num_len = num.to_string().len();
    if times == 0 {
        1
    } else if let Some(m) = memo.get(&(num, times)) {
        *m
    } else {
        let tr = if num == 0 {
            recur(1, times - 1, memo)
        } else if num_len % 2 == 1 {
            recur(num * 2024, times - 1, memo)
        } else {
            let div = u64::pow(10, num_len as u32 / 2);
            recur(num / div, times - 1, memo)
                + recur(num % div, times - 1, memo)
        };
        memo.insert((num, times), tr);
        tr
    }
}

#[aoc(day11, part1)]
fn part1(inp: &str) -> usize {
    let mut memo = HashMap::new();
    inp.extract_tokens::<u64>().map(|num| recur(num, 25, &mut memo)).sum()
}

#[aoc(day11, part2)]
fn part2(inp: &str) -> usize {
    let mut memo = HashMap::new();
    inp.extract_tokens::<u64>().map(|num| recur(num, 75, &mut memo)).sum()
}
