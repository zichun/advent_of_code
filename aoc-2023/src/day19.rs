use crate::prelude::*;

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}
impl Part {
    fn from(inp: &str) -> Self {
        let mut inp = inp
            .split(&['=', ',', '}'])
            .filter_map(|t| t.parse::<u32>().ok());
        Part {
            x: inp.next().unwrap(),
            m: inp.next().unwrap(),
            a: inp.next().unwrap(),
            s: inp.next().unwrap(),
        }
    }
}

struct Rule {
    el: char,
    ord: char,
    check: u32,
    to: String,
}
impl Rule {
    fn from(inp: &str) -> Self {
        let mut inp = inp.split(":");
        let mut c = inp.next().unwrap().chars();
        Rule {
            el: c.next().unwrap(),
            ord: c.next().unwrap(),
            check: c.collect::<String>().parse::<u32>().unwrap(),
            to: inp.next().unwrap().to_owned(),
        }
    }
    fn check(&self, p: &Part) -> bool {
        let pvalue = match self.el {
            'x' => p.x,
            'm' => p.m,
            'a' => p.a,
            's' => p.s,
            _ => unreachable!(),
        };
        match self.ord {
            '<' => pvalue < self.check,
            '>' => pvalue > self.check,
            _ => unreachable!(),
        }
    }
}
struct Rules {
    check: Vec<Rule>,
    last: String,
}
impl Rules {
    fn from(inp: &str) -> Self {
        let (check, last) = inp
            .split(",")
            .fold((Vec::new(), String::new()), |(mut acc, s), el| {
                if el.contains(":") {
                    acc.push(Rule::from(el));
                    (acc, s)
                } else {
                    (acc, el.to_owned())
                }
            });
        Self { check, last }
    }
    fn get_next(&self, part: &Part) -> String {
        if let Some(r) = self.check.iter().find(|rule| rule.check(part)) {
            r.to.clone()
        } else {
            self.last.clone()
        }
    }
}
struct Input {
    graph: HashMap<String, Rules>,
    parts: Vec<Part>,
}

#[aoc_generator(day19)]
fn parse(inp: &str) -> Input {
    let mut inp = inp.split("\n\n");
    let graph = inp
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut l = l.split("}").next().unwrap().split("{");
            let key = l.next().unwrap();
            (key.to_owned(), Rules::from(l.next().unwrap()))
        })
        .collect();
    let parts = inp.next().unwrap().lines().map(|l| Part::from(l)).collect();
    Input { graph, parts }
}

#[aoc(day19, part1)]
fn part1(inp: &Input) -> u32 {
    inp.parts
        .iter()
        .filter(|part| {
            let mut cur = "in".to_owned();
            while cur != "A" && cur != "R" {
                cur = inp.graph[&cur].get_next(part);
            }
            cur == "A"
        })
        .map(|p| p.x + p.m + p.a + p.s)
        .sum()
}

#[derive(Default, Clone, Debug)]
struct Constraint(Vec<(u32, u32)>);
impl Constraint {
    fn intersect(&mut self, rule: &Rule, opp: bool) {
        let (ord, check) = if opp {
            if rule.ord == '<' {
                ('>', rule.check - 1)
            } else {
                ('<', rule.check + 1)
            }
        } else {
            (rule.ord, rule.check)
        };
        self.0 = match ord {
            '<' => self
                .0
                .iter()
                .filter_map(|(l, r)| {
                    if check >= *r {
                        Some((*l, *r))
                    } else if check <= *l {
                        None
                    } else {
                        Some((*l, check - 1))
                    }
                })
                .collect(),
            '>' => self
                .0
                .iter()
                .filter_map(|(l, r)| {
                    if check <= *l {
                        Some((*l, *r))
                    } else if check >= *r {
                        None
                    } else {
                        Some((check + 1, *r))
                    }
                })
                .collect(),
            _ => unreachable!(),
        };
    }
    fn score(&self) -> u64 {
        self.0
            .iter()
            .map(|(l, r)| (r + 1 - l) as u64)
            .product::<u64>()
    }
}
#[derive(Default, Clone, Debug)]
struct PartConstraint {
    x: Constraint,
    m: Constraint,
    a: Constraint,
    s: Constraint,
}
impl PartConstraint {
    fn intersect(&self, r: &Rule, opp: bool) -> Self {
        let mut res = self.clone();
        match r.el {
            'x' => res.x.intersect(r, opp),
            'm' => res.m.intersect(r, opp),
            'a' => res.a.intersect(r, opp),
            's' => res.s.intersect(r, opp),
            _ => unreachable!(),
        }
        res
    }
    fn score(&self) -> u64 {
        self.x.score() * self.m.score() * self.a.score() * self.s.score()
    }
}

#[aoc(day19, part2)]
fn part2(inp: &Input) -> u64 {
    fn solve(inp: &Input, cur: String, mut constraint: PartConstraint, dep: usize) -> u64 {
        if cur == "R" {
            return 0;
        } else if cur == "A" {
            return constraint.score();
        }

        let mut tr = inp.graph[&cur].check.iter().fold(0, |tr, rule| {
            let solved = solve(
                inp,
                rule.to.clone(),
                constraint.intersect(rule, false),
                dep + 1,
            );
            constraint = constraint.intersect(rule, true);
            tr + solved
        });
        tr += solve(inp, inp.graph[&cur].last.clone(), constraint, dep + 1);
        tr
    }

    solve(
        inp,
        "in".to_owned(),
        PartConstraint {
            x: Constraint(vec![(1, 4000)]),
            m: Constraint(vec![(1, 4000)]),
            a: Constraint(vec![(1, 4000)]),
            s: Constraint(vec![(1, 4000)]),
        },
        0,
    )
}
