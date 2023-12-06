use crate::prelude::*;

fn to_digits(line: &str) -> (u32, u32) {
    let mut min = None;
    let mut max = None;

    fn find(min: &mut Option<(usize, u32)>,
            max: &mut Option<(usize, u32)>,
            line: &str, st: &str, dig: u32) {
        if let Some(ind) = line.find(st) {
            *min = Some(min.map_or((ind, dig), |prev| {
                prev.min((ind, dig))
            }));
        }
        if let Some(ind) = line.rfind(st) {
            *max = Some(max.map_or((ind, dig), |prev| {
                prev.max((ind, dig))
            }));
        }
    }

    vec![("one", "1"), ("two", "2"), ("three", "3"),
         ("four", "4"), ("five", "5"), ("six", "6"),
         ("seven", "7"), ("eight", "8"), ("nine", "9")]
        .iter().for_each(|(st, dig)| {
            let d = dig.parse::<u32>().unwrap();
            find(&mut min, &mut max, line, st, d);
            find(&mut min, &mut max, line, dig, d);
        });
    (min.unwrap().1, max.unwrap().1)
}

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<String> {
    input.split("\n")
        .map(|chunk| chunk.to_owned())
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[String]) -> u32 {
    input.iter().map(|l| {
        let x = l.chars().filter_map(|x| x.to_string().parse::<u32>().ok()).collect::<Vec<_>>();
        x[0] * 10 + x[x.len() - 1]
    }).sum()
}


#[aoc(day1, part2)]
fn part2(input: &[String]) -> u32 {
    input.iter().map(|l| {
        let x = to_digits(l);
        x.0 * 10 + x.1
    }).sum()
}
