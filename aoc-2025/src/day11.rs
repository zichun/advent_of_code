use crate::prelude::*;

#[aoc_generator(day11)]
fn parse_input(inp: &str) -> Graph<String> {
    let mut g = Graph::new();
    inp.lines().for_each(|l| {
        let mut tok = l.split(": ");
        let from: String = tok.next_token();
        let to = tok.next_token::<String>();
        (&to as &str).parse_tokens::<String>().for_each(|to| {
            g.add_edge(from.clone(), to);
        });
    });
    g
}

#[aoc(day11, part1)]
fn part1(g: &Graph<String>) -> usize {
    fn vis(g: &Graph<String>, node: String, memo: &mut HashMap<String, usize>) -> usize {
        if node == "out" {
            return 1;
        } else if memo.contains_key(&node) {
            return memo[&node];
        }

        let mut tr = 0;
        if g.adj_list.contains_key(&node) {
            for v in &g.adj_list[&node] {
                tr += vis(g, v.to_owned(), memo);
            }
        }
        memo.insert(node, tr);
        tr
    }

    let mut memo = HashMap::new();
    vis(g, "you".to_owned(), &mut memo)
}

#[aoc(day11, part2)]
fn part2(g: &Graph<String>) -> usize {
    fn vis(g: &Graph<String>, dacfft: u8, node: String, memo: &mut HashMap<(String, u8), usize>) -> usize {
        if node == "out" {
            if dacfft == 3 {
                return 1;
            } else {
                return 0;
            }
        } else if memo.contains_key(&(node.clone(), dacfft)) {
            return memo[&(node, dacfft)];
        }

        let mut tr = 0;
        if g.adj_list.contains_key(&node) {
            for v in &g.adj_list[&node] {
                let ndacfft = if v == "dac" {
                    dacfft | 1
                } else if v == "fft" {
                    dacfft | 2
                } else {
                    dacfft
                };
                tr += vis(g, ndacfft, v.to_owned(), memo);
            }
        }
        memo.insert((node, dacfft), tr);
        tr
    }

    let mut memo = HashMap::new();
    vis(g, 0, "svr".to_owned(), &mut memo)
}
