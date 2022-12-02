use std::collections::HashSet;

#[derive(Clone)]
enum Instruction
{
    Nop(i32),
    Acc(i32),
    Jmp(i32)
}

impl Instruction
{
    fn parse(line: &str) -> Self
    {
        let mut iter = line.split(|x: char| x.is_whitespace());
        let instruction = iter.next().unwrap();
        let mut offset_str = iter.next().unwrap().chars();
        let mul = if offset_str.next().unwrap() == '-' { -1 } else { 1 };
        let offset = offset_str.collect::<String>().parse::<i32>().unwrap();
        match instruction {
            "nop" => Instruction::Nop(offset * mul),
            "acc" => Instruction::Acc(offset * mul),
            "jmp" => Instruction::Jmp(offset * mul),
            _ => panic!("Invalid instruction")
        }
    }
}

struct Machine
{
    instructions: Vec<Instruction>,
    accumulator: i32,
    address: u32
}

impl Machine
{
    fn load(input: &str) -> Self
    {
        Machine {
            instructions: input.lines().map(|x| Instruction::parse(x)).collect(),
            accumulator: 0,
            address: 0
        }
    }
    fn run_till_loop(&mut self) -> (i32, bool) {
        let mut set = HashSet::new();
        self.address = 0;
        self.accumulator = 0;
        loop
        {
            if self.address == self.instructions.len() as u32
            {
                return (self.accumulator, false);
            }
            else if self.address > self.instructions.len() as u32
            {
                return (self.accumulator, true);
            }

            if set.contains(&self.address)
            {
                return (self.accumulator, true);
            }
            set.insert(self.address);

            match self.instructions[self.address as usize] {
                Instruction::Nop(_) => {
                    self.address += 1;
                },
                Instruction::Acc(offset) => {
                    self.accumulator += offset;
                    self.address += 1;
                },
                Instruction::Jmp(offset) => {
                    if offset < 0
                    {
                        if (offset * -1) as u32 > self.address
                        {
                            return (self.accumulator, true);
                        }
                        self.address -= (offset * -1) as u32;
                    }
                    else
                    {
                        self.address += offset as u32;
                    }
                }
            };
        }
    }
    fn fix_machine(&mut self) -> i32
    {
        let original_instructions = self.instructions.clone();
        for i in 0..original_instructions.len()
        {
            if let Instruction::Acc(_) = original_instructions[i]
            {
                continue;
            }

            self.instructions = original_instructions.clone();

            if let Instruction::Nop(offset) = original_instructions[i]
            {
                self.instructions[i] = Instruction::Jmp(offset);
            }
            else if let Instruction::Jmp(offset) = original_instructions[i]
            {
                self.instructions[i] = Instruction::Nop(offset);
            }
            if let (acc, false) = self.run_till_loop()
            {
                return acc;
            }
        }
        panic!("Cannot find a solution");
    }
}

pub fn day08_2(input: &str) -> i32
{
    let mut machine = Machine::load(input);
    machine.fix_machine()
}

pub fn day08_1(input: &str) -> i32
{
    let mut machine = Machine::load(input);
    machine.run_till_loop().0
}

#[test]
fn test_day08_1()
{
    let inp = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    assert_eq!(day08_1(&inp), 5);
}

#[test]
fn test_day08_2()
{
    let inp = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    assert_eq!(day08_2(&inp), 8);
}
