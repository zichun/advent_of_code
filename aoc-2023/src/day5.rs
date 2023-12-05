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
    let mut seeds: Vec<Range> = Vec::new();
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
