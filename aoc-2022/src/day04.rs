struct Interval(usize, usize);

impl Interval {
    fn contains(&self, other: &Interval) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }
    fn overlap(&self, other: &Interval) -> bool {
        (self.0 >= other.0 && self.0 <= other.1) ||
            (other.0 >= self.0 && other.0 <= self.1)
    }
}

impl From<&str> for Interval {
    fn from(s: &str) -> Self {
        let mut s = s.split("-");
        Interval(
            s.next().unwrap().parse::<usize>().unwrap(),
            s.next().unwrap().parse::<usize>().unwrap()
        )
    }
}

fn parse(input: &str) -> Vec<(Interval, Interval)> {
    input.lines()
        .map(|l| {
            let mut l = l.split(",");
            (l.next().unwrap().into(),
             l.next().unwrap().into())
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|(one, two)|
                one.contains(two) || two.contains(one))
        .count()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|(one, two)|
                one.overlap(two))
        .count()
}

#[test]
fn test() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    assert_eq!(part1(input), 2);
    assert_eq!(part2(input), 4);
}
