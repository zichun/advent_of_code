use itertools::Itertools;
use std::iter::Sum;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input.lines()
        .batching(|it| {
            let cal = it.take_while(|e|
                          !e.trim().is_empty())
                .map(|e| e.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            if cal.is_empty() {
                None
            } else {
                Some(cal)
            }
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: &str) -> u32 {
    let cals = parse(input);

    let max = cals.iter()
        .map(|v|
             v.iter().sum::<u32>())
        .max();
    max.unwrap()
}

pub fn part2(input: &str) -> u32 {
    let cals = parse(input);
    let mut cals = cals.iter()
        .map(|v|
             v.iter().sum::<u32>())
        .collect::<Vec<_>>();

    cals.sort_by(|a, b| b.partial_cmp(a).unwrap());
    cals.iter().take(3).sum()
}


#[test]
fn test() {
    let inp = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    assert_eq!(part1(inp), 24000);
    assert_eq!(part2(inp), 45000);
}
