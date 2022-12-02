use std::collections::HashMap;

enum Rule {
    Unit(char),
    Pattern(Vec<Vec<usize>>)
}

#[derive(Default)]
struct Grammar {
    rules: HashMap<usize, Rule>
}

impl Grammar {
    fn add_rule(&mut self, rule_index: usize, rule: &str) {
        if rule.starts_with("\"") {
            let c = rule.chars().skip(1).next().unwrap();
            self.rules.insert(rule_index, Rule::Unit(c));
        } else {
            let pattern = rule.split("|")
                .map(|pattern| {
                    pattern
                        .split(" ")
                        .filter(|x| !x.is_empty())
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                }).collect::<Vec<_>>();
            self.rules.insert(rule_index, Rule::Pattern(pattern));
        }
    }

    fn match_rule(&self, rule_index: usize, string: &str) -> bool {

        fn test(rules: &HashMap<usize, Rule>, rule_index: usize, input: &[char], next: &[usize]) -> bool {
            if input.len() == 0 {
                return false;
            }
            match &rules[&rule_index] {
                Rule::Unit(c) => {
                    if *c == input[0] {
                        if next.is_empty() && input.len() == 1 {
                            true
                        } else if (next.is_empty() && input.len() > 1) ||
                            (input.len() == 1 && !next.is_empty())
                        {
                            false
                        } else {
                            test(rules, next[0], &input[1..input.len()], &next[1..next.len()])
                        }
                    } else {
                        false
                    }
                },
                Rule::Pattern(patterns) => {
                    for pattern in patterns {
                        let mut inner_next = pattern[1..pattern.len()].iter().map(|&x| x).collect::<Vec<_>>();
                        inner_next.extend(next);

                        if test(rules, pattern[0], input, &inner_next) {
                            return true;
                        }
                    }
                    false
                }
            }
        }

        let chars = string.chars().collect::<Vec<char>>();
        test(&self.rules, rule_index, &chars, &vec![])
    }
}



pub fn day19_1(input: &str) -> usize {
    let mut g = Grammar::default();
    input.lines()
        .take_while(|&line| !line.trim().is_empty())
        .for_each(|line| {
            let mut liter = line.split(": ");
            let rule_index = liter.next().unwrap().parse::<usize>().unwrap();
            let rules = liter.next().unwrap();
            g.add_rule(rule_index, rules);
        });

    input.lines().skip(g.rules.len() + 1)
        .filter(|&line| g.match_rule(0, line))
        .count()
}

pub fn day19_2(input: &str) -> usize {
    let mut g = Grammar::default();
    input.lines()
        .take_while(|&line| !line.trim().is_empty())
        .for_each(|line| {
            let mut liter = line.split(": ");
            let rule_index = liter.next().unwrap().parse::<usize>().unwrap();
            let rules = liter.next().unwrap();
            match rule_index {
                8 => g.add_rule(rule_index, "42 | 42 8"),
                11 => g.add_rule(rule_index, "42 31 | 42 11 31"),
                _  => g.add_rule(rule_index, rules)
            };
        });

    input.lines().skip(g.rules.len() + 1)
        .filter(|&line| g.match_rule(0, line))
        .count()
}

#[test]
fn test_day19() {
    let inp = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

bababa
ababbb
abbbab
aaabbb
aaaabbb";
    assert_eq!(day19_1(inp), 2);

    let inp = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

    assert_eq!(day19_2(inp), 12);
}
