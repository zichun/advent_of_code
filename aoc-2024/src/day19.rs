use crate::prelude::*;

struct Inp {
    towels: Vec<String>,
    pats: Vec<String>
}
#[aoc_generator(day19)]
fn parse(inp: &str) -> Inp {
    let mut tok = inp.split("\n\n");
    let towels = tok.next().unwrap().split(",").map(|s| s.trim().to_owned()).collect();
    let pats = tok.next().unwrap().lines().map(|s| s.trim().to_owned()).collect();
    Inp { towels, pats }
}

fn possible(towels: &[String], pat: &str, memo: &mut HashMap<String, usize>) -> usize {
    if pat.is_empty() {
        1
    } else if memo.contains_key(pat) {
        memo[pat]
    } else {
        let tr = towels.iter().map(|t| {
            if t.len() <= pat.len() && pat.starts_with(t) {
                possible(towels, &pat[t.len()..pat.len()], memo)
            } else {
                0
            }
        }).sum();
        memo.insert(pat.to_owned(), tr);
        tr
    }
}

#[aoc(day19, part1)]
fn part1(inp: &Inp) -> usize {
    let mut memo = HashMap::new();
    inp.pats.iter().filter(|p| possible(&inp.towels, p, &mut memo) > 0).count()
}

#[aoc(day19, part2)]
fn part2(inp: &Inp) -> usize {
    let mut memo = HashMap::new();
    inp.pats.iter().map(|p| possible(&inp.towels, p, &mut memo)).sum()
}
