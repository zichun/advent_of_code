use crate::prelude::*;

#[derive(Clone)]
struct Gate {
    vars: HashMap<String, bool>,
    cond: HashMap<String, (String, String, String)>,
}

impl Gate {
    fn get_vars(&self) -> impl Iterator<Item=&String> {
        self.vars.keys().chain(self.cond.keys()).unique()
    }
    fn get(&mut self, var: &str) -> bool {
        let mut vis = HashSet::new();
        self.get_inner(var, &mut vis)
    }
    fn get_inner(&mut self, var: &str, vis: &mut HashSet<String>) -> bool {
        if vis.contains(var) {
            return false;
        }
        vis.insert(var.to_owned());
        if self.vars.contains_key(var) {
            self.vars[var]
        } else {
            let tr = self.compute(var, vis);
            self.vars.insert(var.to_owned(), tr);
            tr
        }
    }
    fn compute(&mut self, var: &str, vis: &mut HashSet<String>) -> bool {
        let (a, op, b) = self.cond[var].clone();
        let a = self.get_inner(&a, vis);
        let b = self.get_inner(&b, vis);
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
        let (a, op, b) = (ops.next().unwrap(), ops.next().unwrap(), ops.next().unwrap());
        (var, (a.clone().min(b.clone()).to_owned(), op, b.max(a).to_owned()))
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
    vars.into_iter().fold(0, |acc, v| ((acc << 1) | inp.get(&v) as usize))
}

type ForwardType = HashMap<String, Vec<usize>>;
fn compute(cond: &[(String, String, String, String)], forward: &ForwardType, rng: std::ops::Range<usize>) -> bool {
    let mut cache = HashMap::new();
    let mut q = VecDeque::new();

    for case in 0..3 {
        cache.clear();

        for ind in rng.clone() {
            let (x, y) = (format!("x{:0>2}", ind), format!("y{:0>2}", ind));
            if case == 0 {
                cache.insert(x.clone(), true);
                cache.insert(y, true);
            } else if case == 1 {
                cache.insert(x.clone(), false);
                cache.insert(y, false);
            } else {
                cache.insert(x.clone(), true);
                cache.insert(y, false);
            }
            q.push_back(x);
        }

        let mut vis = HashSet::new();
        let mut problem = false;
        while let Some(nxt) = q.pop_front() {
            if let Some(v) = forward.get(&nxt) {
                for cind in v {
                    let (a, op, b, to) = &cond[*cind];
                    let a = *cache.get(a).unwrap_or(&false);
                    let b = *cache.get(b).unwrap_or(&false);
                    let res = match op.as_str() {
                        "XOR" => a ^ b,
                        "OR" => a || b,
                        "AND" => a && b,
                        _ => unreachable!()
                    };
                    cache.insert(to.to_owned(), res);

                    if to.starts_with("z") {
                        let zind = get_ind(to);
                        if zind < rng.start {
                            problem = true;
                        } else {
                            if case == 0 && ((zind == rng.start && res) || (zind > rng.start && !res)) {
                                problem = true;
                            } else if case == 1 && res {
                                problem = true;
                            } else if case == 2 && ((zind == rng.end && res) || (zind < rng.end && !res)) {
                                problem = true;
                            }
                        }
                    }
                    if !vis.contains(&to) {
                        vis.insert(to);
                        q.push_back(to.to_owned());
                    }
                }
            }
        }
        if problem {
            return false;
        }
    }
    true
}

fn verify(cond: &[(String, String, String, String)], maxz: usize) -> bool {
    use rand::prelude::*;

    let mut q = VecDeque::new();
    let cond = cond.iter().map(|(a, op, b, to)| (to.to_owned(), (a.to_owned(), op.to_owned(), b.to_owned()))).collect();
    let mut g = Gate {
        vars: HashMap::new(),
        cond
    };

    let maxz = 45;
    for _tries in 0..10 {
        let mut rng = rand::thread_rng();
        let xr = rng.gen::<u64>() % (1 << maxz);
        let yr = rng.gen::<u64>() % (1 << maxz);
        let zr = xr + yr;
        let mut vars = HashMap::new();
        for ind in 0..maxz {
            let (x, y) = (format!("x{:0>2}", ind), format!("y{:0>2}", ind));
            vars.insert(x.clone(), xr & (1 << ind) > 0);
            vars.insert(y, yr & (1 << ind) > 0);
            q.push_back(x);
        }
        g.vars = vars;

        for zind in 0..maxz {
            let z = format!("z{:0>2}", zind);
            let res = g.get(&z);
            if res != (zr & (1 << zind) > 0) {
                return false
            }
        }
    }
    true
}

fn get_ind(s: &str) -> usize {
    s[1..].parse::<usize>().unwrap()
}

#[aoc(day24, part2)]
fn part2(inp: &Gate) -> String {
    let mut inp = inp.clone();

    let mut vars: Vec<String> = inp.get_vars().filter(|v| v.starts_with("z")).map(String::to_owned).collect();
    vars.sort();
    let maxz = get_ind(&vars[vars.len() - 1]);

    let cond = inp.cond.iter().map(|(k, (a, op, b))| {
        (a.clone(), op.clone(), b.clone(), k.clone())
    }).collect::<Vec<_>>();

    let mut bad_z = Vec::new();
    let mut bad_xor = Vec::new();
    let mut solns = Vec::new();
    cond.iter().enumerate().for_each(|(ind, (a, op, b, to))| {
        if op == "XOR" && !a.starts_with("x") && !to.starts_with("z") {
            solns.push(to.to_owned());
            bad_xor.push(ind);
        } else if to.starts_with("z") && op != "XOR" && to != "z45" {
            solns.push(to.to_owned());
            bad_z.push(ind);
        }
    });
    assert_eq!(bad_z.len(), bad_xor.len());

    let _ = bad_z.iter().permutations(bad_z.len()).find(|badz| {
        let mut cond = cond.clone();
        for i in 0..badz.len() {
            swap(&mut cond, *badz[i], bad_xor[i]);
        }

        for i in 0..cond.len() {
            if bad_z.contains(&i) || bad_xor.contains(&i) || cond[i].3.starts_with("z") {
                continue;
            }
            for j in i+1..cond.len() {
                if bad_z.contains(&j) || bad_xor.contains(&j) || cond[j].3.starts_with("z"){
                    continue;
                }
                swap(&mut cond, i, j);
                let forward = compute_forward(&cond);
                if verify(&cond, maxz) {
                    solns.push(cond[i].3.to_owned());
                    solns.push(cond[j].3.to_owned());
                    return true;
                }
                swap(&mut cond, i, j);
            }
        }
        false
    }).unwrap();

    fn swap(cond: &mut Vec<(String, String, String, String)>, i: usize, j: usize) {
        let tmp = cond[i].3.clone();
        cond[i].3 = cond[j].3.clone();
        cond[j].3 = tmp;
    }

    fn compute_forward(cond: &[(String, String, String, String)]) -> ForwardType {
        let mut forward: ForwardType = HashMap::new();
        for (cind, (a, _, b, _to)) in cond.iter().enumerate() {
            forward.entry(a.to_owned()).or_default().push(cind);
            forward.entry(b.to_owned()).or_default().push(cind);
        }
        forward
    }

    solns.sort();
    solns.into_iter().join(",")
}
