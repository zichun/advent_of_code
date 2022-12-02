#[derive(Clone, Copy, Eq, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Win,
    Draw,
    Lose,
}

impl From<char> for Choice {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Choice::Rock,
            'B' | 'Y' => Choice::Paper,
            'C' | 'Z' => Choice::Scissors,
            _ => unimplemented!()
        }
    }
}

impl From<usize> for Choice {
    fn from(c: usize) -> Self {
        match c {
            1 => Choice::Rock,
            2 => Choice::Paper,
            3 => Choice::Scissors,
            _ => unimplemented!(),
        }
    }
}

impl From<char> for Result {
    fn from(c: char) -> Self {
        match c {
            'X' => Result::Lose,
            'Y' => Result::Draw,
            'Z' => Result::Win,
            _ => unimplemented!()
        }
    }
}

impl Result {
    fn score(&self) -> u32 {
        match self {
            Result::Win => 6,
            Result::Draw => 3,
            Result::Lose => 0,
        }
    }
}

impl Choice {
    fn score(&self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn is_win(a: Choice, b: Choice) -> bool { // returns true if a wins b
        (b.score()) % 3 == a.score() - 1
    }

    fn resolve(&self, c: Choice) -> Result {
        if *self == c {
            Result::Draw
        } else if Choice::is_win(*self, c) {
            Result::Win
        } else {
            Result::Lose
        }
    }
}

fn resolve(game: &[(Choice, Choice)]) -> u32 {
    game.iter().map(|(a, b)| {
        b.resolve(*a).score() + b.score()
    }).sum()
}

pub fn part1(input: &str) -> u32 {
    let game = input.lines().map(|line| {
        (Choice::from(line.as_bytes()[0] as char), Choice::from(line.as_bytes()[2] as char))
    }).collect::<Vec<_>>();

    resolve(&game)
}

pub fn part2(input: &str) -> u32 {
    input.lines().map(|line| {
        let res = Result::from(line.as_bytes()[2] as char);
        let lchoice = Choice::from(line.as_bytes()[0] as char);

        let rchoice = match res {
            Result::Win => Choice::from((lchoice.score() % 3 + 1) as usize),
            Result::Draw => lchoice,
            Result::Lose => Choice::from(((lchoice.score() + 1) % 3 + 1) as usize),
        };

        res.score() + rchoice.score()
    }).sum()
}


#[test]
fn test() {
    let input = "A Y
B X
C Z";
    assert_eq!(part1(&input), 15);
    assert_eq!(part2(&input), 12);
}

#[test]
fn test_1() {
    assert_eq!(part1("A X"), 4);
    assert_eq!(part1("A Y"), 8);
    assert_eq!(part1("A Z"), 3);

    assert_eq!(part1("B X"), 1);
    assert_eq!(part1("B Y"), 5);
    assert_eq!(part1("B Z"), 9);

    assert_eq!(part1("C X"), 7);
    assert_eq!(part1("C Y"), 2);
    assert_eq!(part1("C Z"), 6);

    assert_eq!(part1("A X
A Y
A Z
B X
B Y
B Z
C X
C Y
C Z"), 45);
}
