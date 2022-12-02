use std::collections::VecDeque;

enum Register {
    W, X, Y, Z, Int(i64)
}

impl Register {
    fn from(c: &str) -> Self {
        if c.bytes().next().unwrap().is_ascii_alphabetic() {
            match c.bytes().next().unwrap() as char {
                'w' => Register::W,
                'x' => Register::X,
                'y' => Register::Y,
                'z' => Register::Z,
                _ => panic!("Invalid input"),
            }
        } else {
            Register::Int(c.parse::<i64>().unwrap())
        }
    }
    fn set(&self, state: &mut [i64; 4], val: i64) {
        match self {
            Register::W => state[0] = val,
            Register::X => state[1] = val,
            Register::Y => state[2] = val,
            Register::Z => state[3] = val,
            Register::Int(_) => panic!("Invalid set"),
        }
    }
    fn get(&self, state: &[i64; 4]) -> i64 {
        match self {
            Register::W => state[0],
            Register::X => state[1],
            Register::Y => state[2],
            Register::Z => state[3],
            Register::Int(f) => *f
        }
    }
    fn exec_and_set(
        &self, r: &Register, state: &mut [i64; 4], exec: fn(i64, i64) -> i64,
        check: fn(i64, i64) -> bool
    ) -> bool {
        let v0 = self.get(state);
        let v1 = r.get(state);
        if !check(v0, v1)
        {
            false
        }
        else
        {
            self.set(state, exec(v0, v1));
            true
        }
    }
}
enum Instruction {
    Inp(Register),
    Add(Register, Register),
    Mul(Register, Register),
    Div(Register, Register),
    Mod(Register, Register),
    Eql(Register, Register),
}

impl Instruction {
    fn from(line: &str) -> Self {
        let mut iter = line.split(" ");
        let instr = iter.next().unwrap();
        match instr {
            "inp" => Instruction::Inp(Register::from(iter.next().unwrap())),
            "mul" => Instruction::Mul(Register::from(iter.next().unwrap()), Register::from(iter.next().unwrap())),
            "div" => Instruction::Div(Register::from(iter.next().unwrap()), Register::from(iter.next().unwrap())),
            "mod" => Instruction::Mod(Register::from(iter.next().unwrap()), Register::from(iter.next().unwrap())),
            "eql" => Instruction::Eql(Register::from(iter.next().unwrap()), Register::from(iter.next().unwrap())),
            "add" => Instruction::Add(Register::from(iter.next().unwrap()), Register::from(iter.next().unwrap())),
            _ => panic!("Invalid Instructions")
        }
    }
}

struct Machine {
    code: Vec<Instruction>
}

struct Runtime<'a> {
    machine: &'a Machine,
    loc: usize,
    state: [i64; 4],
    input: VecDeque<i64>
}

impl<'a> Iterator for Runtime<'a> {
    type Item = [i64; 4];

    fn next(&mut self) -> Option<Self::Item> {
        let mut processed_inp = false;
        while self.loc + 1 < self.machine.code.len() {
            let valid = match &self.machine.code[self.loc] {
                Instruction::Inp(r) => {
                    if processed_inp {
                        return Some(self.state.clone())
                    }
                    processed_inp = true;
                    if self.input.is_empty() {
                        false
                    } else {
                        r.set(&mut self.state, self.input.pop_front().unwrap());
                        true
                    }
                },
                Instruction::Mul(r0, r1) => {
                    r0.exec_and_set(r1, &mut self.state, |a, b| { a * b }, |_, _| { true } )
                },
                Instruction::Add(r0, r1) => {
                    r0.exec_and_set(r1, &mut self.state, |a, b| { a + b }, |_, _| { true } )
                },
                Instruction::Div(r0, r1) => {
                    r0.exec_and_set(r1, &mut self.state, |a, b| { a / b }, |_, b| { b != 0 })
                },
                Instruction::Mod(r0, r1) => {
                    r0.exec_and_set(r1, &mut self.state, |a, b| { a / b }, |a, b| { a >= 0 && b > 0 })
                },
                Instruction::Eql(r0, r1) => {
                    r0.exec_and_set(r1, &mut self.state, |a, b| { if a == b { 1 } else { 0 }}, |_, _| { true } )
                },
            };

            if !valid {
                return None;
            }

            self.loc += 1;
        }

        Some(self.state.clone())
    }
}

impl Machine {
    fn from(inp: &str) -> Self {
        let code = inp.lines().map(|l| {
            Instruction::from(l)
        }).collect::<Vec<_>>();

        Machine {
            code
        }
    }

    fn execute<'a>(&'a self, inp: &[i64]) -> Runtime<'a> {
        Runtime {
            machine: self,
            loc: 0,
            state: [0; 4],
            input: inp.iter().map(ToOwned::to_owned).collect::<VecDeque<_>>()
        }
    }
}

pub fn p1(inp: &str) -> String {
    let m = Machine::from(inp);
    for i in 1..=9 {
        for j in 1..=9 {
            for k in 1..=9 {
                let mut foo = m.execute(&vec![i, j, k]);
                foo.next().unwrap();
                foo.next().unwrap();
                println!("{} {} {}: {:?}", i, j, k, foo.next().unwrap());
            }
        }
    }

    "".to_owned()
}
