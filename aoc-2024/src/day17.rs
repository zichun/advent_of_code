use std::marker::PhantomData;
use enum_iterator::Sequence;

use crate::prelude::*;

trait Inst {
    fn from_op(op: u8) -> Self;
    fn execute<O: Ops>(&self, op: u8, o: &mut O) -> ComputerOutput;
}
trait Ops {
    fn combo(&self, op: u8) -> i64;
    fn write_register(&mut self, reg: usize, val: i64);
    fn get_register(&self, reg: usize) -> i64;
}

struct Inp {
    registers: Vec<i64>,
    program: Vec<u8>
}

#[aoc_generator(day17)]
fn parse(inp: &str) -> Inp {
    let mut toks = inp.split("\n\n");
    let registers = toks.next().unwrap().lines().map(|l| {
        l.split(": ").nth(1).unwrap().parse::<i64>().unwrap()
    }).collect();
    let program = toks.next().unwrap().split(": ").nth(1).unwrap().
        split(",").map(|p| p.parse::<u8>().unwrap()).collect();

    Inp { registers, program }
}

struct Operands(i64, i64, i64);

impl Ops for Operands {
    fn combo(&self, op: u8) -> i64 {
        match op {
            0..=3 => op as i64,
            4..=6 => self.get_register(op as usize - 4),
            _ => unreachable!()
        }
    }
    fn write_register(&mut self, reg: usize, val: i64) {
        let r = match reg {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => unreachable!()
        };
        *r = val;
    }
    fn get_register(&self, reg: usize) -> i64 {
        match reg {
            0 => self.0,
            1 => self.1,
            2 => self.2,
            _ => unreachable!()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Sequence, Debug, Hash)]
enum Instructions {
    Adv = 0,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}
impl Inst for Instructions {
    fn from_op(op: u8) -> Self {
        enum_iterator::all::<Instructions>().enumerate().find(|(ind, _)| *ind == op as usize).unwrap().1
    }
    fn execute<O: Ops>(&self, operand: u8, o: &mut O) -> ComputerOutput {
        match self {
            Instructions::Adv => { // 0: A = A / 2^combo(op)
                o.write_register(0, o.get_register(0) / i64::pow(2, o.combo(operand).try_into().unwrap()));
                ComputerOutput::Null
            },
            Instructions::Bxl => { // 1: B = B ^ op
                o.write_register(1, o.get_register(1) ^ operand as i64);
                ComputerOutput::Null
            },
            Instructions::Bst => { // 2: B = combo(op) % 8
                o.write_register(1, o.combo(operand) & 7);
                ComputerOutput::Null
            },
            Instructions::Jnz => { // 3: goto op if A != 0
                if o.get_register(0) == 0 {
                    ComputerOutput::Null
                } else {
                    ComputerOutput::Jmp(operand)
                }
            },
            Instructions::Bxc => { // 4: B = B ^ C
                o.write_register(1, o.get_register(1) ^ o.get_register(2));
                ComputerOutput::Null
            },
            Instructions::Out => { // 5: print combo(op) % 8
                ComputerOutput::Out((o.combo(operand) & 7) as u8)
            },
            Instructions::Bdv => { // 6: B = A / 2^combo(op)
                o.write_register(1, o.get_register(0) / i64::pow(2, o.combo(operand).try_into().unwrap()));
                ComputerOutput::Null
            },
            Instructions::Cdv => { // 7: C = A / 2^combo(op)
                o.write_register(2, o.get_register(0) / i64::pow(2, o.combo(operand).try_into().unwrap()));
                ComputerOutput::Null
            },
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum ComputerOutput {
    Null,
    Jmp(u8),
    Out(u8),
}
struct Computer<I: Inst, O: Ops> {
    op: O,
    program: Vec<u8>,
    inst_ptr: usize,
    output: Vec<u8>,
    halted: bool,
    _inst: PhantomData<I>
}
impl<I: Inst, O: Ops> Computer<I, O> {
    fn new(op: O, program: Vec<u8>) -> Self {
        Computer {
            op,
            program,
            inst_ptr: 0,
            output: Vec::new(),
            halted: false,
            _inst: PhantomData
        }
    }
    fn is_halted(&self) -> bool {
        self.halted
    }
    fn tick(&mut self) {
        let opcode = self.program[self.inst_ptr];
        let operand = self.program[self.inst_ptr + 1];

        let instruction = I::from_op(opcode);
        let out = instruction.execute(operand, &mut self.op);
        self.process(out);
    }
    fn process(&mut self, out: ComputerOutput) {
        match out {
            ComputerOutput::Jmp(ind) => self.inst_ptr = ind as usize,
            ComputerOutput::Null => self.inst_ptr += 2,
            ComputerOutput::Out(o) => {
                self.output.push(o);
                self.inst_ptr += 2;
            },
        }

        if self.inst_ptr >= self.program.len() {
            self.halted = true;
        }
    }
    fn debug(&self) {
        println!("Registers: {} {} {}", self.op.get_register(0), self.op.get_register(1), self.op.get_register(2));
        println!("Instr: {}", self.inst_ptr);
    }
}

#[aoc(day17, part1)]
fn part1(inp: &Inp) -> String {
    let op = Operands(inp.registers[0], inp.registers[1], inp.registers[2]);
    let mut c: Computer<Instructions, _> = Computer::new(op, inp.program.clone());

    while !c.is_halted() {
        c.tick();
    }
    c.output.into_iter().join(",").to_string()
}

#[aoc(day17, part2)]
fn part2(inp: &Inp) -> usize {
/*
while A > 0:
2 4: B = A % 8
1 1: B = B ^ b0001
7 5: C = A / 2^B
1 4: B = B ^ b0100
0 3: A = A / 8
4 5: B = B ^ C
5 5: PRINT B
3 0
*/
    let mut target = inp.program.clone();
    target.reverse();
    fn recur(pos: usize, target: &[u8], mut a: usize) -> Option<usize>{
        assert!(pos < target.len());

        let mut b = a & 7;
        b ^= 1;
        let c = a / usize::pow(2, b as u32);
        b ^= 4;
        b ^= c;
        b &= 7;
        if b == target[pos] as usize {
            if pos == target.len() - 1 {
                return Some(a);
            }
            for add in 0..8 {
                let na = a * 8 + add;
                if na > 0 {
                    if let Some(s) = recur(pos + 1, target, na) {
                        return Some(s);
                    }
                }
            }
        }
        None
    }

    for a in 1..8 {
        if let Some(s) = recur(0, &target, a) {
            return s;
        }
    }
    0
}
