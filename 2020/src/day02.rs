use regex::Regex;

struct Day02Input {
    small: usize,
    large: usize,
    c: char,
    password: String
}

impl Day02Input {
    fn parse(line: &str) -> Self {
        let re = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): ([a-z]*)").unwrap();
        let cap = re.captures(line).ok_or("no match").unwrap();

        Day02Input {
            small: cap[1].parse().unwrap(),
            large: cap[2].parse().unwrap(),
            c: cap[3].parse().unwrap(),
            password: cap[4].to_string()
        }
    }
}

fn is_valid_1(small: usize, large: usize, c: char, password: &str) -> bool
{
    let occur = password.chars().filter(|x| *x == c).count();
    occur >= small && occur <= large
}

fn fulfilled_1(cond: &str) -> bool
{
    let inp = Day02Input::parse(cond);
    is_valid_1(inp.small, inp.large, inp.c, &inp.password)
}

pub fn day02_1(input: &str) -> usize
{
    input.lines().filter(|x| fulfilled_1(x)).count()
}

fn is_valid_2(small: usize, large: usize, c: char, password: &str) -> bool
{
    (password.chars().nth(small - 1).unwrap() == c) ^ (password.chars().nth(large - 1).unwrap() == c)
}

fn fulfilled_2(cond: &str) -> bool
{
    let inp = Day02Input::parse(cond);
    is_valid_2(inp.small, inp.large, inp.c, &inp.password)
}

pub fn day02_2(input: &str) -> usize
{
    input.lines().filter(|x| fulfilled_2(x)).count()
}

#[test]
fn test_fulfilled_1()
{
    assert_eq!(fulfilled_1(&"1-3 a: abcde"), true);
    assert_eq!(fulfilled_1(&"2-9 c: ccccccccc"), true);
    assert_eq!(fulfilled_1(&"1-3 b: cdefg"), false);

    assert_eq!(day02_1(&"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"), 2);
}

#[test]
fn test_2()
{
    assert_eq!(fulfilled_2(&"1-3 a: abcde"), true);
    assert_eq!(fulfilled_2(&"2-9 c: ccccccccc"), false);
    assert_eq!(fulfilled_2(&"1-3 b: cdefg"), false);

    assert_eq!(day02_2(&"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"), 1);
}
