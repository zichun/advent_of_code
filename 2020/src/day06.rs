use std::collections::HashSet;
use std::collections::HashMap;

fn part1(block: &str) -> u32
{
    block.chars().filter(|x| x.is_alphabetic()).collect::<HashSet<_>>().len() as u32
}

pub fn day06_1(input: &str) -> u32
{
    input
        .split("\n\n")
        .map(|x| {
            x.split("\r\n\r\n").map(|y| part1(y)).sum::<u32>()
        })
        .sum()

}

pub fn part2(block: &str) -> u32
{
    let mut map = HashMap::new();
    block.chars().filter(|x| x.is_alphabetic()).for_each(|x| {
        if map.contains_key(&x)
        {
            *map.get_mut(&x).unwrap() += 1;
        }
        else
        {
            map.insert(x, 1);
        }
    });
    let ans = block.lines().count();
    map.iter().filter(|x| *x.1 == ans).count() as u32

}

pub fn day06_2(input: &str) -> u32
{
    input
        .split("\n\n")
        .map(|x| {
            x.split("\r\n\r\n").map(|y| part2(y)).sum::<u32>()
        })
        .sum()
}

#[test]
fn test_day06_1()
{
    let inp = "abc

a
b
c

ab
ac

a
a
a
a

b";
    assert_eq!(day06_1(&inp), 11);
}

#[test]
fn test_day06_2()
{
    let inp = "abc

a
b
c

ab
ac

a
a
a
a

b";
    assert_eq!(day06_2(&inp), 6);
}
