use std::collections::HashSet;

struct BagRule
{
    outer: String,
    qty: u32,
    inner: String
}

struct BagRules
{
    rules: Vec<BagRule>
}

impl BagRules {
    fn get_bag<'a, I>(iter: &mut I) -> String
        where I: Iterator<Item = &'a str>
    {
        iter.next().unwrap().to_string() + "-" + &iter.next().unwrap().to_string()
    }

    fn add_rule(&mut self, outer: String, qty: u32, inner: String)
    {
        self.rules.push( BagRule { outer, qty, inner } );
    }

    fn add_str_rule(&mut self, rule: &str) -> ()
    {
        let mut iter = rule.split(|x: char| x.is_whitespace());
        let outer = BagRules::get_bag(&mut iter);
        iter.next();
        iter.next();

        while let Some(qty) = iter.next()
        {
            if qty == "no" { break; }
            let qty = qty.parse::<u32>().ok().unwrap();
            let inner = BagRules::get_bag(&mut iter);
            self.add_rule(outer.to_string(), qty, inner.to_string());
            iter.next();
        }
    }

    fn can_contain(&self, bag: &str) -> u32
    {
        fn recur(rules: &BagRules, bag: &str, visited: &mut HashSet<String>) {
            if visited.contains(bag) {
                return;
            }
            visited.insert(bag.into());

            for rule in rules.rules.iter() {
                if rule.inner == bag {
                    recur(rules, &rule.outer, visited);
                }
            }
        }

        let mut visited: HashSet<String> = HashSet::new();
        recur(&self, bag, &mut visited);
        (visited.len() - 1) as u32
    }

    fn bag_count(&self, bag: &str) -> u32
    {
        let mut ans = 1;
        for rule in self.rules.iter() {
            if rule.outer == bag {
                ans += rule.qty * self.bag_count(&rule.inner);
            }
        }
        ans
    }

    fn new() -> Self
    {
        BagRules { rules: vec!() }
    }
}

pub fn day07_1(inp: &str) -> u32
{
    let mut rules = BagRules::new();
    inp.lines().for_each(|x| rules.add_str_rule(x));
    rules.can_contain(&"shiny-gold")
}

pub fn day07_2(inp: &str) -> u32
{
    let mut rules = BagRules::new();
    inp.lines().for_each(|x| rules.add_str_rule(x));
    rules.bag_count(&"shiny-gold") - 1
}

#[test]
fn test_day07_part1()
{
    let inp = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    assert_eq!(day07_1(&inp), 4);
}

#[test]
fn test_day07_part2()
{
    let inp = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    assert_eq!(day07_2(&inp), 32);

    let inp = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
    assert_eq!(day07_2(&inp), 126);
}
