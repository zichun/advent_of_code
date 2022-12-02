
struct Bracket {
    open: char,
    close: char,
    score: u64,
    score2: u64,
}

struct Brackets {
    brackets: Vec<Bracket>
}

impl Brackets {
    fn new() -> Self {
        let mut brackets = Vec::new();
        brackets.push(
            Bracket {
                open: '(',
                close: ')',
                score: 3,
                score2: 1
            }
        );
        brackets.push(
            Bracket {
                open: '[',
                close: ']',
                score: 57,
                score2: 2
            }
        );
        brackets.push(
            Bracket {
                open: '{',
                close: '}',
                score: 1197,
                score2: 3
            }
        );
        brackets.push(
            Bracket {
                open: '<',
                close: '>',
                score: 25137,
                score2: 4
            }
        );
        Brackets { brackets }
    }

    fn get_score(&self, inp: &str) -> (u64, u64) {
        let mut stack = Vec::new();
        for c in inp.chars() {
            if let Some(b) = self.brackets.iter().filter(|b| b.open == c).next() {
                stack.push(b);
            } else {
                if stack.len() == 0 || stack.last().unwrap().close != c {
                    return (self.brackets.iter().filter(|b| b.close == c).next().unwrap().score, 0)
                } else {
                    let _ = stack.pop();
                }
            }
        }
        let mut incomplete_score = 0;
        while let Some(b) = stack.pop() {
            incomplete_score = incomplete_score * 5 + b.score2;
        }
        (0, incomplete_score)
    }
}

pub fn p1(input: &str) -> u64 {
    let brackets = Brackets::new();
    input.lines().map(|line| brackets.get_score(line).0).sum()
}
pub fn p2(input: &str) -> u64 {
    let brackets = Brackets::new();
    let mut collect = input.lines().filter_map(|line|
                                               match brackets.get_score(line) {
                                                   (0, s) => Some(s),
                                                   _ => None
                                               }).collect::<Vec<_>>();
    collect.sort();
    collect[collect.len() / 2]
}

#[test]
fn test() {
    let input = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

    assert_eq!(p1(input), 26397);
    assert_eq!(p2(input), 288957);
}
