use std::{any::Any, sync::Mutex};

use aoc_helper::prelude::num::integer::lcm;

use crate::prelude::*;

struct Module {
    module_type: char,
    module: Mutex<Box<dyn ModuleTrait>>,
    outputs: Vec<String>,
}

struct Machine {
    modules: HashMap<String, Module>,
}

struct MachineRunner<'a> {
    machine: &'a Machine,
    low: usize,
    high: usize,
    pulses: VecDeque<(bool, &'a str, &'a str)>,
}
impl<'a> MachineRunner<'a> {
    fn with_machine(machine: &'a Machine) -> Self {
        Self {
            machine,
            low: 0,
            high: 0,
            pulses: VecDeque::new(),
        }
    }
    fn new_pulse(&mut self, module_name: &'a str, pulse: bool, from: &'a str) {
        self.pulses.push_back((pulse, module_name, from));
        if pulse {
            self.high += 1;
        } else {
            self.low += 1;
        }
    }
    fn press_button(&mut self, lookfor: &str) -> Option<String> {
        self.new_pulse("broadcaster", false, "");
        let mut found = None;
        while let Some((pulse, module_name, from)) = self.pulses.pop_front() {
            if module_name == lookfor && pulse {
                found = Some(from.to_owned());
            }
            if let Some(module) = self.machine.modules.get(module_name) {
                let mut module_write = module.module.lock().unwrap();
                if let Some(output_pulse) = module_write.pulse(pulse, from) {
                    module
                        .outputs
                        .iter()
                        .for_each(|dest| self.new_pulse(dest, output_pulse, module_name));
                }
            }
        }
        found
    }
}

trait ModuleTrait: std::fmt::Debug {
    fn pulse(&mut self, pulse: bool, from: &str) -> Option<bool>;
    fn new_input(&mut self, _from: &str) {}
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
struct Broadcaster {}
impl ModuleTrait for Broadcaster {
    fn pulse(&mut self, pulse: bool, _from: &str) -> Option<bool> {
        Some(pulse)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Default, Debug)]
struct FlipFlop {
    state: bool,
}
impl ModuleTrait for FlipFlop {
    fn pulse(&mut self, pulse: bool, _from: &str) -> Option<bool> {
        if pulse {
            None
        } else {
            self.state = !self.state;
            Some(self.state)
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Default, Debug)]
struct Conjunction {
    input_cache: HashMap<String, bool>,
}
impl ModuleTrait for Conjunction {
    fn pulse(&mut self, pulse: bool, from: &str) -> Option<bool> {
        self.input_cache.insert(from.to_owned(), pulse);
        if self.input_cache.iter().all(|(_k, v)| *v == true) {
            Some(false)
        } else {
            Some(true)
        }
    }
    fn new_input(&mut self, from: &str) {
        self.input_cache.insert(from.to_owned(), false);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[aoc_generator(day20)]
fn parse(inp: &str) -> Machine {
    let mut modules = HashMap::new();
    let mut from = HashMap::new();
    inp.lines().for_each(|l| {
        let mut l = l.split(" -> ");
        let mut module_name = l.next().unwrap().to_owned();
        let mut module_type = 'b';
        let outputs = l
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();

        if module_name != "broadcaster" {
            module_type = module_name.chars().next().unwrap();
            module_name = module_name.chars().skip(1).collect::<String>();
        }

        outputs.iter().for_each(|to| {
            from.entry(to.to_owned())
                .or_insert(Vec::new())
                .push(module_name.clone());
        });

        let module: Box<dyn ModuleTrait> = match module_type {
            'b' => Box::new(Broadcaster {}),
            '%' => Box::new(FlipFlop::default()),
            '&' => Box::new(Conjunction::default()),
            _ => unreachable!(),
        };

        modules.insert(
            module_name.clone(),
            Module {
                module_type,
                module: Mutex::new(module),
                outputs,
            },
        );
    });

    from.iter().for_each(|(to, inputs)| {
        if let Some(module) = modules.get_mut(to) {
            if module.module_type == '&' {
                let mut module_write = module.module.lock().unwrap();
                inputs.iter().for_each(|inp| module_write.new_input(inp));
            }
        }
    });

    Machine { modules }
}

#[aoc(day20, part1)]
fn part1(machine: &Machine) -> usize {
    let mut runner = MachineRunner::with_machine(machine);
    (0..1000).for_each(|_| {
        runner.press_button("");
    });
    runner.low * runner.high
}

#[aoc(day20, part2)]
fn part2(machine: &Machine) -> usize {
    let mut runner = MachineRunner::with_machine(machine);
    let mut periods = HashMap::new();
    for t in 1.. {
        if let Some(from) = runner.press_button("gh") {
            periods.insert(from, t);
            if periods.len() == 4 {
                break;
            }
        }
    }
    periods.iter().fold(1, |acc, (_, p)| lcm(acc, *p))
}
