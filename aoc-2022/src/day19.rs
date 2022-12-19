use std::collections::HashMap;

struct Blueprint {
    ore_bot: u8,
    clay_bot: u8,
    obsidian_bot: (u8, u8),
    geode_bot: (u8, u8),
}

fn parse(input: &str) -> Vec<Blueprint> {
    input.lines().map(|l| {
        let c = l.split(' ').map(|x| x.to_owned()).collect::<Vec<_>>();
        Blueprint {
            ore_bot: c[6].parse().unwrap(),
            clay_bot: c[12].parse().unwrap(),
            obsidian_bot: (c[18].parse().unwrap(), c[21].parse().unwrap()),
            geode_bot: (c[27].parse().unwrap(), c[30].parse().unwrap())
        }
    }).collect()
}

type State = [u8; 7];
type Memo = HashMap<State, u8>;

fn next(state: &State) -> State {
    let mut tr = state.clone();
    tr[0] -= 1;
    for i in 1..=3 {
        tr[i] += tr[i + 3];
    }
    tr
}
fn f(state: State, blueprint: &Blueprint, memo: &mut Memo) -> u8 {
    if state[0] <= 1 {
        return 0;
    } else if memo.contains_key(&state) {
        return memo[&state];
    } else {
        let st = next(&state);
        let mut tr = 0;

        if state[1] >= blueprint.geode_bot.0 &&
            state[3] >= blueprint.geode_bot.1
        {
            let mut st = next(&state);
            st[1] -= blueprint.geode_bot.0;
            st[3] -= blueprint.geode_bot.1;
            tr = tr.max(f(st, blueprint, memo) + st[0]);
        } else {
            tr = tr.max(f(st, blueprint, memo));
            // ore, clay, obs, geode
            if state[1] >= blueprint.ore_bot {
                let mut st = next(&state);
                st[1] -= blueprint.ore_bot;
                st[4] += 1;
                tr = tr.max(f(st, blueprint, memo));
            }
            if state[1] >= blueprint.clay_bot {
                let mut st = next(&state);
                st[1] -= blueprint.clay_bot;
                st[5] += 1;
                tr = tr.max(f(st, blueprint, memo));
            }
            if state[1] >= blueprint.obsidian_bot.0 &&
                state[2] >= blueprint.obsidian_bot.1
            {
                let mut st = next(&state);
                st[1] -= blueprint.obsidian_bot.0;
                st[2] -= blueprint.obsidian_bot.1;
                st[6] += 1;
                tr = tr.max(f(st, blueprint, memo));
            }
        }

        memo.insert(state, tr);
        tr
    }
}

fn p1(blueprint: &Blueprint) -> u64 {
    let mut state: State = [0; 7];
    let mut memo = HashMap::new();
    state[0] = 24;
    state[4] = 1;
    let ans = f(state, blueprint, &mut memo) as u64;
    println!("ans: {}", ans);
    ans
}

pub fn part1(input: &str) -> u64 {
    let bps = parse(input);
    bps.iter().enumerate().map(|(ind, bp)| {
        (ind + 1) as u64 * p1(bp)
    }).sum()
}

fn p2(blueprint: &Blueprint) -> u64 {
    let mut state: State = [0; 7];
    let mut memo = HashMap::new();
    state[0] = 32;
    state[4] = 1;
    let ans = f(state, blueprint, &mut memo) as u64;
    println!("ans: {}", ans);
    ans
}

pub fn part2(input: &str) -> u64 {
    let bps = parse(input);
    bps.iter().take(3).map(|bp| {
        p2(bp)
    }).product()
}

#[test]
fn test() {
    assert_eq!(p1(&Blueprint { ore_bot: 4, clay_bot: 2, obsidian_bot: (3, 14), geode_bot: (2, 7) }), 9);
    assert_eq!(p1(&Blueprint { ore_bot: 2, clay_bot: 3, obsidian_bot: (3, 8), geode_bot: (3, 12) }), 12);
    let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    assert_eq!(part1(input), 33);
    assert_eq!(part2(input), 62);
}
