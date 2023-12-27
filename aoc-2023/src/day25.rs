use crate::prelude::*;
use rand::Rng;

#[derive(Clone)]
struct Graph {
    nodes: Vec<usize>,
    adj: Vec<Vec<usize>>,
    vertex_cnt: usize,
    edge_cnt: usize,
//    edges: Vec<(usize, usize)>,
}

impl Graph {
    fn merge_edge(&mut self) {
        let pick = rand::thread_rng().gen_range(0..self.edge_cnt);

        let mut sofar = 0;
        for i in 0..self.adj.len() {
            if sofar + self.adj[i].len() > pick {
                let rm = self.adj[i][pick - sofar];
                self.nodes[rm] += self.nodes[i];
                self.nodes[i] = 0;

                for j in 0..self.adj[i].len() {
                    let n = self.adj[i][j];
                    if n != rm {
                        self.adj[n].iter_mut().for_each(|nn| {
                            if *nn == i {
                                *nn = rm;
                            }
                        });
                        self.adj[rm].push(n);
                    }
                }
                self.adj[i].clear();

                let br = self.adj[rm].len();
                self.adj[rm].retain(|&x| x != i);
                self.edge_cnt -= (br - self.adj[rm].len()) * 2;
                self.vertex_cnt -= 1;

                break;
            } else if sofar > pick {
                break;
            }
            sofar += self.adj[i].len();
        }
    }
}

#[aoc_generator(day25)]
fn parse(inp: &str) -> Graph {
    let mut nodes: HashMap<String, usize> = HashMap::new();
    let mut vertex_cnt = 0;
    let mut edge_cnt = 0;
    let mut adj = Vec::new();

    inp.lines().for_each(|l| {
        let mut l = l.split(": ");
        let from = l.next().unwrap();
        l.next().unwrap().extract_tokens::<String>()
            .for_each(|to| {
                nodes.entry(from.to_owned()).or_insert_with(|| { vertex_cnt += 1; vertex_cnt - 1 });
                nodes.entry(to.clone()).or_insert_with(|| { vertex_cnt += 1; vertex_cnt - 1 });

                let (from, to) = (nodes[from], nodes[&to]);
                let (from, to) = (from.min(to), from.max(to));

                while to >= adj.len() {
                    adj.push(Vec::new());
                }
                adj[from].push(to);
                adj[to].push(from);
                edge_cnt += 2
            });
    });
    Graph {
        vertex_cnt,
        nodes: vec![1; vertex_cnt],
        adj,
        edge_cnt,
    }
}

fn contract(graph: &mut Graph, t: usize)  {
    while graph.vertex_cnt > t {
        graph.merge_edge();
    }
}

fn mincut(graph: &mut Graph) -> Option<usize> {
    if graph.vertex_cnt <= 6 {
        contract(graph, 2);
        if graph.edge_cnt == 6 {
            let mut n = graph.nodes.iter().filter(|x| **x != 0);
            Some(*n.next().unwrap() * *n.next().unwrap())
        } else {
            None
        }
    } else {
        let t = 1 + (graph.vertex_cnt as f64 / (2.0_f64)).floor() as usize;
        let mut g2 = graph.clone();
        contract(graph, t);
        mincut(graph)
            .or_else(|| {
                contract(&mut g2, t);
                mincut(&mut g2)
            })
    }
}

#[aoc(day25, part1)]
fn part1(graph: &Graph) -> usize {
    loop {
        let mut g = graph.clone();
        if let Some(ans) = mincut(&mut g) {
            println!("{}", ans);
            return ans;
        }
    }
}
