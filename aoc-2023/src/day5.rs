use std::ops::Range;

use aoc_runner_derive::{aoc_generator, aoc};

#[derive(Debug)]
struct Mapping {
    source: u32,
    dest: u32,
    range: u32,
}
#[derive(Debug)]
struct Input {
    seeds: Vec<u32>,
    maps: Vec<Vec<Mapping>>,
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Input {
    let mut inputs = input.split("\n\n");
    let seeds = inputs.next().unwrap().split(": ")
        .skip(1).next().unwrap().split(" ").map(|s| s.parse::<u32>().unwrap()).collect();
    let maps = inputs.map(|map| {
        let mut map = map.split("\n").skip(1).map(|map_line| {
            let mut tokens = map_line.split(" ");
            Mapping {
                dest: tokens.next().unwrap().parse::<u32>().unwrap(),
                source: tokens.next().unwrap().parse::<u32>().unwrap(),
                range: tokens.next().unwrap().parse::<u32>().unwrap(),
            }
        }).collect::<Vec<_>>();
        map.sort_by(|a, b| a.source.cmp(&b.source));
        map
    }).collect();
    Input {
        seeds,
        maps,
    }
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> u32 {
    input.seeds.iter()
        .map(|seed| {
            let location = input.maps.iter().fold(
                *seed,
                |acc, el| {
                    match el.iter().find(|mapping| acc >= mapping.source && acc < mapping.source + mapping.range) {
                        Some(mapping) => {
                            mapping.dest + acc - mapping.source
                        }
                        None => acc
                    }
                });
            location
        })
        .min().unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> u32 {
    let mut seeds: Vec<Range<u32>> = Vec::new();

    let mut seeds_inp = input.seeds.iter();
    loop {
        match seeds_inp.next() {
            Some(start) => {
                let len = seeds_inp.next().unwrap();
                seeds.push(*start..(start + len));
            }
            None => break,
        }
    }
    seeds.sort_by(|a, b| a.start.cmp(&b.start));

    input.maps.iter().for_each(|map| {
        let mut new_seeds = Vec::new();
        let mut seeds_iter = seeds.iter();
        let mut seed = seeds_iter.next().cloned();

        'map_iter: for m in map.iter() {
            let m_range = m.source..(m.source + m.range);
            loop {
                match seed {
                    Some(ref s) => {
                        if s.end <= m_range.start {
                            new_seeds.push(s.clone());
                            seed = seeds_iter.next().cloned();
                        } else if s.start < m_range.start {
                            new_seeds.push(s.start..m_range.start);
                            seed = Some(m_range.start..s.end);
                        } else if s.start < m_range.end && s.end <= m_range.end {
                            let add = s.start - m_range.start;
                            let add_end = s.end - m_range.start;
                            new_seeds.push((m.dest + add)..(m.dest + add_end));
                            seed = seeds_iter.next().cloned();
                        } else if s.start < m_range.end {
                            let add = s.start - m_range.start;
                            let add_end = m_range.end - m_range.start;
                            new_seeds.push((m.dest + add)..(m.dest + add_end));
                            seed = Some(m_range.end..s.end);
                        } else {
                            continue 'map_iter;
                        }
                    },
                    None => break 'map_iter,
                }
            }
        }

        while let Some(s) = seed {
            new_seeds.push(s.clone());
            seed = seeds_iter.next().cloned()
        }

        // normalize new_seeds - sort and merge
        new_seeds.sort_by(|a, b| a.start.cmp(&b.start));
        seeds = new_seeds.iter().fold(Vec::new(), |mut acc, el| {
            let len = acc.len();
            if len == 0 {
                acc.push(el.clone());
            } else {
                let last = &mut acc[len - 1];
                if el.start <= last.end {
                    (*last).end = (*last).end.max(el.end);
                } else {
                    acc.push(el.clone());
                }
            }
            acc
        });
    });
    seeds[0].start
}
