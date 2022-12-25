use itertools::Itertools;

struct Snafu(Vec<i8>);
const DIGITS: &'static str = "=-012";

impl Snafu {
    fn from(input: &str) -> Self {
        Snafu(input.chars()
              .rev()
              .map(|c|
                   DIGITS.chars().position(|c2| c2 == c).unwrap() as i8 - 2)
              .collect())
    }
    fn to_string(&self) -> String {
        self.0.iter().rev().map(|ind| DIGITS.as_bytes()[(*ind + 2) as usize] as char).collect()
    }
}

impl std::ops::Add for Snafu {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let vec = self.0.clone();
        let (mut acc, carry) = rhs.0.iter().enumerate().fold((vec, 0), |(mut acc, carry), (ind, el)| {
            if ind >= acc.len() {
                acc.push(0);
            }
            acc[ind] += carry + *el;
            if acc[ind] < -2 {
                acc[ind] += 5 as i8;
                (acc, -1)
            } else if acc[ind] >= 3 as i8 {
                acc[ind] -= 5 as i8;
                (acc, 1)
            } else {
                (acc, 0)
            }
        });
        if carry != 0 {
            acc.push(carry);
        }
        Snafu(acc)
    }
}

impl std::iter::Sum for Snafu {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        match iter.next() {
            Some(s) => s + iter.sum::<Snafu>(),
            None => Snafu(Vec::new())
        }
    }
}

pub fn part1(input: &str) -> String {
    input.lines().map(|l| Snafu::from(l)).sum::<Snafu>().to_string()
}

#[test]
fn test() {
    let input = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
    assert_eq!(part1(input), "2=-1=0");
}
