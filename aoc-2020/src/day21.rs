use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Default)]
struct Day21 {
    allegens: HashMap<String, HashSet<String>>,
    input: Vec<HashSet<String>>
}

impl Day21 {
    fn add_rule(&mut self, line: &str) {
        let mut line = line.split(" (contains");
        let ingr = line.next().unwrap().split(" ").map(|x| x.trim().to_owned()).collect::<HashSet<String>>();
        line.next().unwrap().split(")").next().unwrap().split(",")
            .for_each(|all| {
                let all = all.trim();
                if !self.allegens.contains_key(all) {
                    self.allegens.insert(all.into(), ingr.clone());
                } else {
                    let int = self.allegens[all].intersection(&ingr).map(|x| x.to_owned()).collect::<HashSet<String>>();
                    self.allegens.insert(all.into(), int);
                }
            });
        self.input.push(ingr);
    }
    fn resolve(&mut self) {
        let mut modified = true;
        while modified {
            modified = false;
            let isolated = self.allegens.iter().filter(|(_, b)| b.len() == 1)
                .map(|(_, b)| b.iter().next().unwrap().to_owned())
                .collect::<Vec<_>>();
            isolated.iter().for_each(|iso| {
                self.allegens.iter_mut().for_each(|(_, b)| {
                    if b.len() > 1 && b.contains(iso) {
                        modified = true;
                        b.remove(iso);
                    }
                });
            });
        }
    }
    fn part1(&self) -> u32 {
        self.input.iter().map(|ingr| {
            ingr.iter().filter(|i| {
                self.allegens.iter().filter(|(_, ing)| {
                    ing.contains(*i)
                }).count() == 0
            }).count() as u32
        }).sum()
    }
    fn part2(&self) -> String {
        let mut gg = self.allegens
            .iter()
            .collect::<Vec<_>>();
        gg.sort_by(|a, b| {
            a.0.cmp(b.0)
        });
        gg.iter().map(|(_, all)| {
            all.iter().next().unwrap().to_owned()
        }).collect::<Vec<_>>().join(",")
    }
}

pub fn day21_1(input: &str) -> u32 {
    let mut day21 = Day21::default();
    input.lines().for_each(|line| day21.add_rule(line));
    day21.part1()
}

pub fn day21_2(input: &str) -> String {
    let mut day21 = Day21::default();
    input.lines().for_each(|line| day21.add_rule(line));
    day21.resolve();
    day21.part2()
}

#[test]
fn test_day21() {
    let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
    assert_eq!(day21_1(input), 5);
    assert_eq!(day21_2(input), "mxmxvkd,sqjhc,fvjkl");
}
