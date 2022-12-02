use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Expr {
    Unit(i64),
    Op(Operator)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operator {
    Add, Mul, OpenBrace
}

struct PostfixEvaluator {
    stack: Vec<Expr>
}

impl PostfixEvaluator {
    fn parse(expr: &str, precedence: &HashMap<Operator, i8>) -> Self {
        let mut iter = expr.chars().filter(|x| !x.is_whitespace());
        let mut stack = Vec::new();
        let mut op_stack = Vec::new();

        fn get_precedence(op: Operator, precedence: &HashMap<Operator, i8>) -> i8 {
            if op == Operator::OpenBrace {
                std::i8::MIN
            } else if precedence.contains_key(&op) {
                precedence[&op]
            } else {
                0
            }
        }
        loop {
            match iter.next() {
                None => {
                    break;
                },
                Some(c) => {
                    if c.is_numeric() {
                        stack.push(Expr::Unit((c as u8 - b'0') as i64));
                    } else if c == ')' {
                        while *op_stack.last().unwrap() != Operator::OpenBrace {
                            stack.push(Expr::Op(op_stack.pop().unwrap()));
                        }
                        op_stack.pop().unwrap();
                    } else if c == '(' {
                        op_stack.push(Operator::OpenBrace);
                    } else {
                        let o = match c {
                            '+' => Operator::Add,
                            '*' => Operator::Mul,
                            _ => panic!("Invalid char")
                        };
                        let incoming_pre = get_precedence(o, precedence);
                        if op_stack.len() > 0 && incoming_pre <= get_precedence(*op_stack.last().unwrap(), precedence) {
                            stack.push(Expr::Op(op_stack.pop().unwrap()));
                        }
                        op_stack.push(o);
                    }
                }
            }
        }
        while op_stack.len() > 0 {
            stack.push(Expr::Op(op_stack.pop().unwrap()));
        }
        PostfixEvaluator { stack }
    }

    fn evaluate(&self) -> i64 {
        let stack = Vec::new();
        let mut stack = self.stack.iter().fold(stack, |mut stack, el| {
            match el {
                Expr::Unit(val) => {
                    stack.push(*val);
                },
                Expr::Op(Operator::Add) => {
                    let x = stack.pop().unwrap() + stack.pop().unwrap();
                    stack.push(x);
                }
                Expr::Op(Operator::Mul) => {
                    let x = stack.pop().unwrap() * stack.pop().unwrap();
                    stack.push(x);
                }
                _ => panic!("Unexpected operation")
            };
            stack
        });
        stack.pop().unwrap()
    }
}

fn evaluate(expr: &str, precedence: &HashMap<Operator, i8>) -> i64 {
    let eval = PostfixEvaluator::parse(expr, precedence);
    eval.evaluate()
}

pub fn day18_1(input: &str) -> i64 {
    let h = HashMap::new();
    input.lines().map(|x| evaluate(x, &h)).sum()
}
pub fn day18_2(input: &str) -> i64 {
    let mut h = HashMap::new();
    h.insert(Operator::Add, 2);
    h.insert(Operator::Mul, 1);
    input.lines().map(|x| evaluate(x, &h)).sum()
}

#[test]
fn test_evaluate() {
    let mut h = HashMap::new();
    assert_eq!(evaluate("2 * 3 + (4 * 5)", &h), 26);
    assert_eq!(evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)", &h), 437);
    assert_eq!(evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", &h), 12240);
    assert_eq!(evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", &h), 13632);

    h.insert(Operator::Add, 2);
    h.insert(Operator::Mul, 1);
    assert_eq!(evaluate("1 + (2 * 3) + (4 * (5 + 6))", &h), 51);
    assert_eq!(evaluate("2 * 3 + (4 * 5)", &h), 46);
}

