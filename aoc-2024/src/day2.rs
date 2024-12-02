use crate::prelude::*;

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    input.lines()
        .filter_map(|l| {
            l.extract_tokens::<i32>().tuple_windows()
                .fold((true, 0), |(correct, diff), (a, b)| {
                    if !correct || !(1..=3).contains(&a.abs_diff(b)) || (b - a) * diff < 0 {
                        (false, diff)
                    } else {
                        (true, b - a)
                    }
                }).0.then_some(0)
        })
        .count()
}

fn is_safe_part2(inp: Vec<i32>) -> bool {
    let mut memo = [[[true, true], [true, true]],
        [[true, true], [false, false]],
        [[false, false], [false, false]]];

    let n = inp.len();
    for i in 1..n {
        if i > 1 {
            (0..2).for_each(|j| (0..2).for_each(|k| memo[i % 3][j][k] = false));

            if (1..=3).contains(&inp[i].abs_diff(inp[i - 2])) {
                if inp[i] > inp[i - 2] {
                    memo[i % 3][0][0] |= memo[(i - 2) % 3][1][0];
                } else {
                    memo[i % 3][0][1] |= memo[(i - 2) % 3][1][1];
                }
            }
        }
        if (1..=3).contains(&inp[i].abs_diff(inp[i - 1])) {
            if inp[i] > inp[i - 1] {
                memo[i % 3][0][0] |= memo[(i - 1) % 3][0][0];
                memo[i % 3][1][0] |= memo[(i - 1) % 3][1][0];
            } else {
                memo[i % 3][0][1] |= memo[(i - 1) % 3][0][1];
                memo[i % 3][1][1] |= memo[(i - 1) % 3][1][1];
            }
        }
    }
    let mut tr = (0..2).cartesian_product(0..2)
        .fold(false, |acc, (i, j)| acc || memo[(n - 1) % 3][i][j]);
    if n > 1 {
        tr |= memo[(n - 2) % 3][1][0];
        tr |= memo[(n - 2) % 3][1][1];
    }
    tr
}

#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
    input.lines()
        .filter_map(|l| {
            is_safe_part2(l.extract_tokens().collect::<Vec<i32>>()).then_some(0)
        })
        .count()
}

#[test]
fn test() {
    assert!(is_safe_part2(vec![7, 6, 4, 2, 1]));
    assert!(!is_safe_part2(vec![1, 2, 7, 8, 9]));
    assert!(!is_safe_part2(vec![9, 7, 6, 2, 1]));
    assert!(is_safe_part2(vec![1, 3, 2, 4, 5]));
    assert!(is_safe_part2(vec![8, 6, 4, 4, 1]));
    assert!(is_safe_part2(vec![1, 3, 6, 7, 9]));
}
