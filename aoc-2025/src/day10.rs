use crate::prelude::*;

#[derive(Clone)]
struct Button(Vec<usize>);

impl FromStr for Button {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.chars().skip(1).take_while(|&c| c == ',' || c.is_ascii_digit()).collect::<String>();
        Ok(Button(s.split(",").map(|n| n.parse::<usize>().unwrap()).collect()))
    }
}

struct Input {
    switch: Vec<bool>,
    buttons: Vec<Button>,
    joltage: Button
}

impl Input {
    fn min_but(&self) -> usize {
        if self.switch.iter().filter(|&&c| c).count() == 0 {
            return 0;
        }

        for i in 1..self.buttons.len() {
            if self.buttons.iter().combinations(i).any(|buts| {
                let mut switch = vec![false; self.switch.len()];
                buts.iter().for_each(|but| {
                    but.0.iter().for_each(|&ind| switch[ind] = !switch[ind]);
                });
                switch == self.switch
            }) {
                return i;
            }
        }
        unreachable!()
    }

    fn min_joltage_dfs(&self) -> usize {
        let mut jolt_map = self.joltage.0.iter().enumerate().map(|(ind, &jolt)| {
            let button_contains = self.buttons.iter().filter(|b| b.0.contains(&ind)).count();
            (self.buttons.len() - button_contains, jolt, ind)
        }).collect::<Vec<_>>();

        jolt_map.sort();
        jolt_map.reverse();

        let buttons_map = jolt_map.iter().map(|(_, _, ind)| {
            let mut buttons_ind = self.buttons.iter().enumerate().filter(|(_, b)| b.0.contains(ind)).map(|(ind, _)| ind).collect::<Vec<_>>();
            buttons_ind.sort_by(|&ind0, &ind1| self.buttons[ind0].0.len().cmp(&self.buttons[ind1].0.len()));
            buttons_ind
        }).collect::<Vec<_>>();

        fn dfs(jolt_map: &[(usize, usize, usize)], buttons_map: &Vec<Vec<usize>>, joltage: &mut [usize], ind: usize, inp: &Input, memo: &mut HashMap<Vec<usize>, usize>) -> usize {
            if ind >= jolt_map.len() {
                return 0;
            }

            let jolt_map_ind = jolt_map[ind].2;

            if joltage[jolt_map_ind] == 0 {
                return dfs(jolt_map, buttons_map, joltage, ind + 1, inp, memo);
            } else if memo.contains_key(joltage) {
                return memo[joltage];
            }

            let mut tr = usize::MAX;

            let jolt = joltage[jolt_map_ind];
            let buttons_cnt = buttons_map[ind].len();

            let mut bins = vec![0; buttons_cnt];
            bins[buttons_cnt - 1] = jolt;

            loop {
                let mut presses = vec![0; joltage.len()];
                for i in 0..bins.len() {
                    // we press button_ind, bins[i] times
                    if bins[i] == 0 { continue; }
                    let button_ind = buttons_map[ind][i];
                    inp.buttons[button_ind].0.iter().for_each(|&sind| presses[sind] += bins[i]);
                }
                let overflow = presses.iter().enumerate().any(|(sind, &press_cnt)| press_cnt > joltage[sind]);

                if !overflow {

                    presses.iter().enumerate().for_each(|(sind, &press_cnt)|  joltage[sind] -= press_cnt);
                    let res = dfs(jolt_map, buttons_map, joltage, ind + 1, inp, memo);
                    presses.iter().enumerate().for_each(|(sind, &press_cnt)|  joltage[sind] += press_cnt);
                    if res < usize::MAX {
                        tr = tr.min(res + jolt);
                    }
                }

                let riter = bins.iter().rposition(|v| *v != 0).unwrap();
                if riter == 0 {
                    break;
                }
                let temp = bins[riter];
                bins[riter - 1] += 1;
                bins[riter] = 0;
                bins[buttons_cnt - 1] = temp - 1;
            }

            memo.insert(joltage.to_vec(), tr);
            tr
        }
        let mut joltage = self.joltage.0.clone();
        let mut memo = HashMap::new();
        let tr = dfs(&jolt_map, &buttons_map, &mut joltage, 0, self, &mut memo);
        println!("{}", tr);
        tr
    }

    fn min_joltage_astar(&self) -> usize {
        fn dist(cur: &[usize], goal: &[usize]) -> Option<usize> {
            let mut tr = 0;
            for i in 0..cur.len() {
                if goal[i] < cur[i] {
                    return None;
                }
                tr += goal[i] - cur[i];
            }
            Some(tr)
        }
        let mut astar = AStar::new();
        let start = vec![0; self.joltage.0.len()];
        let goal = self.joltage.0.clone();

        astar.visit(start.clone(), 0, dist(&start, &goal).unwrap());
        while let Some((current_node, current_cost)) = astar.pop() {
            if current_node == goal {
                println!("{}", current_cost);
                return current_cost;
            }
            for but in self.buttons.iter() {
                let mut new_node = current_node.clone();
                but.0.iter().for_each(|&ind| new_node[ind] += 1);
                if let Some(new_dist) = dist(&new_node, &goal) {
                    astar.visit(new_node, current_cost + 1, new_dist);
                }
            }
        }
        unreachable!()
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<Input> {
    fn parse_switch(switch: &str) -> Vec<bool> {
        let n = switch.len() - 2;
        switch.chars().skip(1).take(n).map(|c| c == '#').collect()
    }

    input.lines().map(|l| {
        let mut tok = l.split_whitespace().peekable();
        let switch = parse_switch(tok.next().unwrap());
        let mut buttons = Vec::new();
        let joltage;

        loop {
            let but = tok.next_token::<Button>();
            if tok.peek().is_none() {
                joltage = but;
                break;
            } else {
                buttons.push(but);
            }
        }
        Input {
            switch,
            buttons,
            joltage
        }
    }).collect()
}

#[aoc(day10, part1)]
fn part1(inps: &[Input]) -> usize {
    inps.iter().map(|inp| inp.min_but()).sum()
}

#[aoc(day10, part2)]
fn part2(inps: &[Input]) -> usize {
    inps.iter().map(|inp| inp.min_joltage_dfs()).sum()
}
