struct Test {
    modulo: u64,
    if_true: usize,
    if_false: usize,
}

impl Test {
    fn parse(mut iter: std::str::Lines) -> Self {
        let modulo = iter.next().unwrap().split(" ").last().unwrap().parse().unwrap();
        let if_true = iter.next().unwrap().split(" ").last().unwrap().parse().unwrap();
        let if_false = iter.next().unwrap().split(" ").last().unwrap().parse().unwrap();
        Test {
            modulo, if_true, if_false
        }
    }
    fn invoke(&self, new: u64) -> usize {
        if new % self.modulo == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: Test,
    inspected: usize,
}

enum Op {
    Old,
    Num(u64),
}
impl Op {
    fn from(input: &str) -> Self {
        match input {
            "old" => Op::Old,
            num => Op::Num(num.parse::<u64>().unwrap()),
        }
    }
    fn ev(&self, old: u64) -> u64 {
        if let Op::Num(num) = self { *num } else { old }
    }
}

fn eval(old: u64, operand: &str, left: &Op, right: &Op) -> u64 {
    let left = left.ev(old);
    let right = right.ev(old);
    match operand {
        "+" => left + right,
        "*" => left * right,
        _ => unimplemented!(),
    }
}

impl Monkey {
    fn parse(input: &str) -> Monkey {
        let mut iter = input.lines();
        iter.next();
        let items = iter.next().unwrap().split(": ")
            .last().unwrap().split(", ")
            .map(|num| num.parse::<u64>().unwrap()).collect::<Vec<_>>();
        let operation = {
            let mut op_iter = iter.next().unwrap().split("new = ").last().unwrap().split(" ");
            let left = Op::from(op_iter.next().unwrap());
            let operand = op_iter.next().unwrap().to_owned();
            let right = Op::from(op_iter.next().unwrap());
            Box::new(move |old| { eval(old, &operand, &left, &right) })
        };
        let test = Test::parse(iter);

        Monkey {
            items,
            operation,
            test,
            inspected: 0
        }
    }
}

fn parse(input: &str) -> Vec<Monkey> {
    input.split("\n\n")
        .map(|block| {
            Monkey::parse(block)
        })
        .collect()
}

fn process(monkeys: &mut [Monkey], iter_cnt: usize, post_op: Box<dyn Fn(u64) -> u64>) -> u64 {
    let modulo_product = monkeys.iter().map(|m| m.test.modulo).product::<u64>();
    let items = monkeys.iter().enumerate().flat_map(|(ind, m)|
                                                    m.items.iter().map(move |w| (ind, *w))).collect::<Vec<_>>();
    items.iter().for_each(|(ind, wor)| {
        let mut wor = *wor;
        let mut ind = *ind;
        let mut rnd = 0;
        while rnd < iter_cnt {
            monkeys[ind].inspected += 1;
            wor = (monkeys[ind].operation)(wor) % modulo_product;
            wor = post_op(wor);
            let new_ind = monkeys[ind].test.invoke(wor);
            if new_ind < ind {
                rnd += 1;
            }
            ind = new_ind;
        }
    });

    let mut inspect_cnt = monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();
    inspect_cnt.sort_by(|a, b| b.partial_cmp(a).unwrap());
    (inspect_cnt[0] * inspect_cnt[1]) as u64
}

pub fn part1(input: &str) -> u64 {
    let mut monkeys = parse(input);
    process(&mut monkeys, 20, Box::new(|worry| worry / 3))
}

pub fn part2(input: &str) -> u64 {
    let mut monkeys = parse(input);
    process(&mut monkeys, 10000, Box::new(|worry| worry))
}

#[test]
fn test() {
    let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    assert_eq!(part1(input), 10605);
    assert_eq!(part2(input), 2713310158);
}
