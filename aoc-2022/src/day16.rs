use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct Graph {
    map: HashMap<String, usize>,
    valve: Vec<u32>,
    adj: Vec<Vec<u32>>
}

fn ind(map: &mut HashMap<String, usize>, valve: &mut Vec<u32>, valve_name: &str) -> usize {
    if map.contains_key(valve_name) {
        map[valve_name]
    } else {
        let tr = valve.len();
        map.insert(valve_name.to_owned(), tr);
        valve.push(0);
        tr
    }
}

fn parse(input: &str) -> Graph {
    let mut map = HashMap::new();
    let mut valve = Vec::new();
    let mut adj = vec![vec![0; 64]; 64];
    input.lines().for_each(|l| {
        let re = Regex::new("Valve ([A-Z][A-Z]) has flow rate=([0-9]+); tunnels? leads? to valves? ([A-Z, ]+)").unwrap();
        if let Some(cap) =  re.captures(l)  {
            let dest = cap[3].to_owned();
            let from = ind(&mut map, &mut valve, &cap[1]);
            let flow = cap[2].parse::<u32>().unwrap();

            valve[from] = flow;
            dest.split(", ").map(|token| ind(&mut map, &mut valve, token)).for_each(|to| {
                adj[to][from] = 1;
                adj[from][to] = 1;
            });
        }
    });

    for k in 0..adj.len() {
        for i in 0..adj.len() {
            for j in 0..adj.len() {
                if adj[i][k] > 0 && adj[k][j] > 0 {
                    if adj[i][j] == 0 || adj[i][j] > adj[i][k] + adj[k][j] {
                        adj[i][j] = adj[i][k] + adj[k][j];
                    }
                }
            }
        }
    }

    Graph { map, valve, adj }
}

fn compressed(visited: &Vec<bool>) -> u64 {
    let mut tr = 0;
    for i in 0..visited.len() {
        if visited[i] {
            tr += 1;
        }
        tr *= 2;
    }
    tr
}

type Memo = HashMap<(u32, usize, u64), i64>;
fn f(time_left: u32, at: usize, visited: &mut Vec<bool>, g: &Graph, memo: &mut Memo) -> i64 {
    let mut tr = 0;

    if time_left == 0 {
        return 0;
    }

    let com = compressed(visited);
    {
        let entry = memo.entry((time_left, at, com));
        if let std::collections::hash_map::Entry::Occupied(o) = entry {
            return *o.get();
        }
    }

    let mut cand = 0;

    for out in 0..g.valve.len() {
        if !visited[out] && g.adj[at][out] + 1 < time_left {
            visited[out] = true;
            let pressure = g.valve[out] * (time_left - 1 - g.adj[at][out]);

            cand = cand.max(
                pressure as i64 + f(time_left - 1 - g.adj[at][out], out, visited, g, memo));
            visited[out] = false;
        }
    }
    tr += cand;

    memo.insert((time_left, at, com), tr);
    tr
}

pub fn part1(input: &str) -> i64 {
    let g = parse(input);
    let mut visited = vec![false; g.valve.len()];
    let aa = g.map["AA"];

    g.valve.iter().enumerate().filter(|(_, v)| **v == 0)
        .for_each(|(ind, _)| visited[ind] = true );

    let mut memo = HashMap::new();
    f(30, aa, &mut visited, &g, &mut memo)
}

type Memo2 = HashMap<(u32, usize, bool, u64), i64>;
fn f2(time_left: u32, at: usize, el_moved: bool, visited: &mut Vec<bool>, g: &Graph, memo: &mut Memo2) -> i64 {
    let mut tr = 0;

    if el_moved && time_left == 0 {
        return 0;
    }

    let com = compressed(visited);
    {
        let entry = memo.entry((time_left, at, el_moved, com));
        if let std::collections::hash_map::Entry::Occupied(o) = entry {
            return *o.get();
        }
    }

    let mut cand = if el_moved == false {
        f2(26, g.map["AA"], true, visited, g, memo)
    } else {
        0
    };

    for out in 0..g.valve.len() {
        if !visited[out] && g.adj[at][out] + 1 < time_left {
            visited[out] = true;
            let pressure = g.valve[out] * (time_left - 1 - g.adj[at][out]);

            cand = cand.max(
                pressure as i64 + f2(time_left - 1 - g.adj[at][out], out, el_moved, visited, g, memo));
            visited[out] = false;
        }
    }
    tr += cand;

    memo.insert((time_left, at, el_moved, com), tr);
    tr
}

pub fn part2(input: &str) -> i64 {
    let g = parse(input);
    let mut visited = vec![false; g.valve.len()];
    let aa = g.map["AA"];

    g.valve.iter().enumerate().filter(|(_, v)| **v == 0)
        .for_each(|(ind, _)| visited[ind] = true );

    let mut memo = HashMap::new();
    f2(26, aa, false, &mut visited, &g, &mut memo)
}

#[test]
fn test() {
    let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
    assert_eq!(part1(input), 1651);
    assert_eq!(part2(input), 1707);
}
