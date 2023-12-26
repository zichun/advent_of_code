use crate::prelude::*;
use rand::Rng;

#[derive(Clone)]
struct Graph {
    vcnt: usize,
    nodes: HashMap<usize, usize>,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    fn merge(&mut self, edge: usize) {
        let node = self.vcnt;
        self.vcnt += 1;
        let (l, r) = self.edges[edge];
        self.edges.remove(edge);

        let lcnt = self.nodes.remove(&l).unwrap();
        let rcnt = self.nodes.remove(&r).unwrap();
        self.nodes.insert(node, lcnt + rcnt);

        let mut toadd = Vec::new();
        let edges = self.edges.iter().filter(|(ef, et)| {
            if *ef == l && *et == r {
                false
            } else if *ef == l || *ef == r || *et == l || *et == r {
                toadd.push((*ef, *et));
                false
            } else {
                true
            }
        }).copied().collect::<Vec<_>>();

        self.edges = edges;
        toadd.into_iter().for_each(|(nl, nr)| {
            if nl == l || nl == r {
                self.edges.push((node.min(nr), node.max(nr)));
            } else {
                self.edges.push((node.min(nl), node.max(nl)));
            }
        });
    }
    fn pick_random(&self) -> usize {
        rand::thread_rng().gen_range(0..self.edges.len())
    }
}

#[aoc_generator(day25)]
fn parse(inp: &str) -> Graph {
    let mut nodes: HashMap<String, usize> = HashMap::new();
    let mut vcnt = 0;
    let mut edges = Vec::new();

    inp.lines().for_each(|l| {
        let mut l = l.split(": ");
        let from = l.next().unwrap();
        l.next().unwrap().extract_tokens::<String>()
            .for_each(|to| {
                nodes.entry(from.to_owned()).or_insert_with(|| { vcnt += 1; vcnt - 1 });
                nodes.entry(to.clone()).or_insert_with(|| { vcnt += 1; vcnt - 1 });
                let (from, to) = (nodes[from], nodes[&to]);
                edges.push((from.min(to), to.max(from)));
            });
    });
    Graph {
        vcnt,
        nodes: (0..vcnt).map(|ind| (ind, 1)).collect(),
        edges,
    }
}

fn contract(graph: &mut Graph, t: usize)  {
    while graph.nodes.len() > t {
        graph.merge(graph.pick_random());
    }
}

fn mincut(graph: &mut Graph) -> Option<usize> {
    if graph.nodes.len() <= 6 {
        contract(graph, 2);
        if graph.edges.len() == 3 {
            let mut n = graph.nodes.iter();
            Some(*n.next().unwrap().1 * *n.next().unwrap().1)
        } else {
            None
        }
    } else {
        let t = 1 + (graph.nodes.len() as f64 / (2.0_f64)).floor() as usize;
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
