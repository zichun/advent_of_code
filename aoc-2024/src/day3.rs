use crate::prelude::*;

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\(([1-9][0-9]{0,2}),([1-9][0-9]{0,2})\)").unwrap();
    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap())
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let re = Regex::new(r"(mul\([1-9][0-9]{0,2},[1-9][0-9]{0,2}\))|(do\(\))|(don't\(\))").unwrap();
    re.find_iter(input)
        .fold((true, 0), |(enabled, sum), c| {
            let c = c.as_str();
            if c == "do()" {
                (true, sum)
            } else if c == "don't()" {
                (false, sum)
            } else if enabled {
                let mul_re = Regex::new(r"\(([1-9][0-9]{0,2}),([1-9][0-9]{0,2})\)").unwrap();
                let [a, b] = mul_re
                    .captures_iter(c)
                    .map(|c| c.extract())
                    .next()
                    .unwrap()
                    .1;
                (
                    true,
                    sum + a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap(),
                )
            } else {
                (enabled, sum)
            }
        })
        .1
}
