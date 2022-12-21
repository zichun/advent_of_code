use std::collections::HashMap;

struct Monkey {
    value: Option<i64>,
    operation: Option<(String, String, char)>
}

struct Monkeys {
    monkeys: HashMap<String, Monkey>
}

impl Monkeys {
    fn get_value(&self, monkey_name: &str) -> Option<i64> {
        let monkey = self.monkeys.get(monkey_name).unwrap();
        match monkey.value {
            Some(val) => Some(val),
            None => {
                let (left, right, op) = monkey.operation.as_ref()?;
                let (left, right) = (self.get_value(left)?, self.get_value(right)?);
                Some(match op {
                    '+' => left + right,
                    '-' => left - right,
                    '*' => left * right,
                    '/' => left / right,
                    _ => unimplemented!()
                })
            }
        }
    }
    fn get_value_p2(&self, monkey_name: &str, ans: i64) -> i64 {
        let monkey = self.monkeys.get(monkey_name).unwrap();
        match monkey.value {
            Some(val) => val,
            None => {
                if monkey.operation.is_none() {
                    assert_eq!(monkey_name, "humn");
                    ans
                } else {
                    let (left, right, op) = monkey.operation.as_ref().unwrap();
                    let (left_val, right_val) = (self.get_value(left), self.get_value(right));
                    match op {
                        '+' => if left_val.is_some() {
                            self.get_value_p2(right, ans - left_val.unwrap())
                        } else {
                            self.get_value_p2(left, ans - right_val.unwrap())
                        },
                        '*' => if left_val.is_some() {
                            self.get_value_p2(right, ans / left_val.unwrap())
                        } else {
                            self.get_value_p2(left, ans / right_val.unwrap())
                        },
                        '-' => if left_val.is_some() {
                            self.get_value_p2(right, left_val.unwrap() - ans)
                        } else {
                            self.get_value_p2(left, ans + right_val.unwrap())
                        },
                        '/' => if left_val.is_some() {
                            self.get_value_p2(right, left_val.unwrap() / ans)
                        } else {
                            self.get_value_p2(left, ans * right_val.unwrap())
                        },
                        _ => unimplemented!()
                    }
                }
            }
        }
    }
}

fn parse(input: &str, has_human: bool) -> Monkeys {
    let monkeys = input.lines().map(|l| {
        let mut iter = l.split(": ");
        let name = iter.next().unwrap().to_owned();
        let right = iter.next().unwrap();

        if let Ok(val) = right.parse::<i64>() {
            if has_human && name == "humn" {
                (name, Monkey { value: None, operation: None })
            } else {
                (name, Monkey { value: Some(val), operation: None })
            }
        } else {
            let mut iter = right.split(" ");
            let (op_a, op, op_b) = (iter.next().unwrap().to_owned(),
                                    iter.next().unwrap().to_owned(),
                                    iter.next().unwrap().to_owned());
            (name, Monkey { value: None, operation: Some((op_a, op_b, op.chars().next().unwrap())) })
        }
    }).collect();

    Monkeys { monkeys }
}

pub fn part1(input: &str) -> i64 {
    let monkeys = parse(input, false);
    monkeys.get_value("root").unwrap()
}

pub fn part2(input: &str) -> i64 {
    let monkeys = parse(input, true);
    let (op_a, op_b, _) = monkeys.monkeys.get("root").unwrap().operation.as_ref().unwrap();
    let (a_val, b_val) = (monkeys.get_value(op_a), monkeys.get_value(op_b));
    if a_val.is_none() {
        monkeys.get_value_p2(op_a, b_val.unwrap())
    } else {
        monkeys.get_value_p2(op_b, a_val.unwrap())
    }
}

#[test]
fn test() {
    let input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
    assert_eq!(part1(input), 152);
    assert_eq!(part2(input), 301);
}
