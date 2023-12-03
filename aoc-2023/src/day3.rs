use aoc_runner_derive::{aoc_generator, aoc};
use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Type {
    Number(u32, usize),
    Pattern(char)
}
#[derive(Debug, Clone, Copy)]
struct Object {
    object: Type,
    row: usize,
    col: usize,
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Object> {
    let mut objects = Vec::new();
    input.split("\n")
        .enumerate()
        .for_each(|(row, line)| {
            let re = Regex::new(r"[^\.0-9]|[0-9]*").unwrap();
            re.find_iter(line).for_each(|m| {
                let s = m.as_str().trim();
                if s.len() > 0 {
                    let object = if s.len() == 1 && !s.chars().next().unwrap().is_numeric() {
                        Type::Pattern(s.chars().next().unwrap())
                    } else {
                        Type::Number(s.parse::<u32>().unwrap(), s.len())
                    };
                    objects.push(Object {
                        object,
                        row,
                        col: m.range().start
                    });
                }
            });
        });
    objects
}

fn is_adj(obj0: &Object, obj1: &Object, len: usize) -> bool {
    if obj0.row.abs_diff(obj1.row) > 1 {
        false
    } else {
        if obj0.col + 1 < obj1.col || obj1.col + len < obj0.col { false }
        else { true }
    }
}

#[aoc(day3, part1)]
fn part1(input: &[Object]) -> u32 {
    let shapes = input.iter().filter(|o| match o.object {
        Type::Number(_, _) => false,
        Type::Pattern(_) => true,
    }).collect::<Vec<_>>();

     input.iter().filter_map(|o| {
        match o.object {
            Type::Number(s, len) => {
                if shapes.iter().find(|s| is_adj(s, o, len)).is_some() {
                    Some(s)
                } else {
                    None
                }
            }
            Type::Pattern(_) => None,
        }
    }).sum()
}

#[aoc(day3, part2)]
fn part2(input: &[Object]) -> u32 {
    let numbers = input.iter().filter(|o| match o.object {
        Type::Number(_, _) => true,
        Type::Pattern(_) => false,
    }).collect::<Vec<_>>();

    input.iter().filter_map(|o| match o.object {
        Type::Pattern('*') => {
            let adj_numbers = numbers.iter().filter_map(|n| {
                if let Type::Number(s, l) = n.object {
                    if is_adj(o, n, l) { Some(s) }
                    else { None }
                } else {
                    None
                }
            }).collect::<Vec<_>>();

            if adj_numbers.len() == 2 {
                Some(adj_numbers.iter().product::<u32>())
            } else {
                None
            }
        },
        _ => None,
    }).sum()
}
