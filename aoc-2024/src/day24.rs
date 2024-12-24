use crate::prelude::*;

#[derive(Clone)]
struct Gate {
    vars: HashMap<String, bool>,
    cond: HashMap<String, (String, String, String)>
}

impl Gate {
    fn get_vars(&self) -> impl Iterator<Item=&String> {
        self.vars.keys().chain(self.cond.keys()).unique()
    }
    fn get(&mut self, var: &str) -> bool {
        if self.vars.contains_key(var) {
            self.vars[var]
        } else {
            let tr = self.compute(var);
            self.vars.insert(var.to_owned(), tr);
            tr
        }
    }

    fn compute(&mut self, var: &str) -> bool {
        let (a, op, b) = self.cond[var].clone();
        let a = self.get(&a);
        let b = self.get(&b);
        match op.as_str() {
            "XOR" => a ^ b,
            "OR" => a || b,
            "AND" => a && b,
            _ => unreachable!()
        }
    }
}
#[aoc_generator(day24)]
fn parse(inp: &str) -> Gate {
    let mut tok = inp.split("\n\n");
    let vars = tok.next().unwrap().lines().map(|l| {
        let mut tok = l.split(": ");
        (tok.next().unwrap().to_owned(), tok.next().unwrap().parse::<usize>().unwrap() != 0)
    }).collect();

    let cond = tok.next().unwrap().lines().map(|l| {
        let mut tok = l.split(" -> ");
        let mut ops = tok.next().unwrap().extract_tokens::<String>();
        let var = tok.next().unwrap().to_string();
        (var, (ops.next().unwrap(), ops.next().unwrap(), ops.next().unwrap()))
    }).collect();

    Gate {
        vars,
        cond
    }
}

#[aoc(day24, part1)]
fn part1(inp: &Gate) -> usize {
    let mut inp = inp.clone();
    let mut vars: Vec<String> = inp.get_vars().filter(|v| v.starts_with("z")).map(String::to_owned).collect();
    vars.sort_by(|a, b| b.cmp(a));
    vars.iter().for_each(|v| {
        println!("{} {}", v, inp.get(v));
    });
    vars.into_iter().fold(0, |acc, v| ((acc << 1) | inp.get(&v) as usize))
}

#[aoc(day24, part2)]
fn part2(inp: &Gate) -> usize {
    let mut inp = inp.clone();
    let mut vars: Vec<String> = inp.get_vars().filter(|v| v.starts_with("z")).map(String::to_owned).collect();
    vars.sort();
    //    vars[0][1..].parse::<usize>().unwrap()
    vars[vars.len() - 1][1..].parse::<usize>().unwrap()
}
