#![allow(dead_code)]

use std::{fmt::{Display, Formatter}, ops::*};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub fn eval<T>(&self, l: T, r: T) -> T
    where T: Add<Output=T> + Mul<Output=T> + Sub<Output=T> + Div<Output=T> {
        match self {
            Op::Add => l.add(r),
            Op::Sub => l.sub(r),
            Op::Mul => l.mul(r),
            Op::Div => l.div(r),
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mul => write!(f, "*"),
            Op::Div => write!(f, "/"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Symbol<T> {
    Operator(Op),
    Number(T),
    LeftParen,
    RightParen,
}

pub struct Infix<T> {
    expr: Vec<Symbol<T>>
}

impl<T> Infix<T>
where T: Copy + Add<Output=T> + Mul<Output=T> + Sub<Output=T> + Div<Output=T> + std::cmp::PartialEq + std::fmt::Display + std::fmt::Debug
{
    pub fn eval(&self) -> Result<T, ()> {
        let rpn: Rpn<T> = self.into();
        println!("{:?}", self.expr);
        rpn.eval()
    }
}

impl<T> std::str::FromStr for Infix<T>
where T: From<u16> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut expr: Vec<Symbol<T>> = Vec::new();
        if let Some(acc) = s.chars().fold(None, |acc, el| {
            if el.is_ascii_digit() {
                let digit = el.to_digit(10).unwrap();
                match acc {
                    None => Some(digit),
                    Some(acc) => Some(acc * 10 + digit)
                }
            } else {
                if let Some(acc) = acc {
                    expr.push(Symbol::Number(T::from(acc.try_into().unwrap())));
                }
                if el == ' ' {
                    None
                } else {
                    let symb = match el {
                        '(' => Symbol::<T>::LeftParen,
                        ')' => Symbol::RightParen,
                        '+' => Symbol::Operator(Op::Add),
                        '-' => Symbol::Operator(Op::Sub),
                        '*' => Symbol::Operator(Op::Mul),
                        '/' => Symbol::Operator(Op::Div),
                        _ => Symbol::Operator(Op::Add),
                    };
                    expr.push(symb);
                    None
                }
            }
        }) {
            expr.push(Symbol::Number(T::from(acc.try_into().unwrap())));
        }
        Ok(Self {
            expr
        })
    }
}

impl<T> From<&Infix<T>> for Rpn<T>
where T: std::cmp::PartialEq + Copy
{
    fn from(infx: &Infix<T>) -> Self {
        let mut rpn = Vec::new();
        let mut ops = Vec::new();

        for symb in &infx.expr {
            match symb {
                Symbol::Operator(_) => {
                    while !ops.is_empty() {
                        let op: &&Symbol<T> = ops.last().unwrap();
                        if **op != Symbol::LeftParen {
                            rpn.push(*ops.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                    ops.push(symb);
                },
                Symbol::LeftParen => ops.push(symb),
                Symbol::RightParen => {
                    let mut found = false;
                    while let Some(op) = ops.pop() {
                        if *op == Symbol::LeftParen {
                            found = true;
                            break;
                        }
                        rpn.push(*op);
                    }
                    if !found {
                        panic!("Invalid infix notation");
                    }
                },
                Symbol::Number(_) => rpn.push(*symb),
            }
        }
        while let Some(op) = ops.pop() {
            if let Symbol::Operator(_) = op {
                rpn.push(*op);
            } else {
                panic!("Invalid infix notation");
            }
        }
        Rpn {
            stack: rpn
        }
    }
}

pub struct Rpn<T> {
    stack: Vec<Symbol<T>>
}

impl<T> Display for Rpn<T>
where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buf_stack: Vec<String> = Vec::new();
        for symb in &self.stack {
            match symb {
                Symbol::Operator(op) => {
                    let (r, l) = (buf_stack.pop().ok_or(std::fmt::Error)?, buf_stack.pop().ok_or(std::fmt::Error)?);
                    buf_stack.push(format!("({} {} {})", l, op, r).to_owned());
                },
                Symbol::Number(num) => buf_stack.push(num.to_string()),
                _ => unreachable!()
            }
        }
        let str = buf_stack.pop().ok_or(std::fmt::Error)?;
        write!(f, "{}", str)
    }
}

impl<T> Rpn<T>
where T: Copy + Add<Output=T> + Mul<Output=T> + Sub<Output=T> + Div<Output=T> + std::fmt::Display + std::fmt::Debug
{
    pub fn push(&mut self, symb: Symbol<T>) {
        self.stack.push(symb);
    }
    pub fn push_op(&mut self, op: Op) {
        self.stack.push(Symbol::Operator(op));
    }
    pub fn push_num(&mut self, t: T) {
        self.stack.push(Symbol::Number(t));
    }
    pub fn print(&self) {
        println!("{}", self);
    }
    pub fn eval(&self) -> Result<T, ()> {
        self.print();
        println!("{:?}", self.stack);
        let mut buf_stack: Vec<T> = Vec::new();
        for symb in &self.stack {
            match symb {
                Symbol::Operator(op) => {
                    let (r, l) = (buf_stack.pop().ok_or(())?, buf_stack.pop().ok_or(())?);
                    buf_stack.push(op.eval(l, r));
                },
                Symbol::Number(num) => buf_stack.push(*num),
                _ => unreachable!()
            }
        }
        if buf_stack.len() != 1 {
            Err(())
        } else {
            Ok(buf_stack.pop().unwrap())
        }
    }
}

#[test]
fn test() {
    use std::str::FromStr;

    let infx: Infix<u32> = Infix::from_str("1 + 2").unwrap();
    assert_eq!(infx.eval().unwrap(), 3);

    let infx: Infix<u32> = Infix::from_str("(3 + 4) * (1 + 2) - 4").unwrap();
    assert_eq!(infx.eval().unwrap(), 17);

    let infx: Infix<i32> = Infix::from_str("(13 - 15) * (123 + 456)").unwrap();
    assert_eq!(infx.eval().unwrap(), -1158);
}
