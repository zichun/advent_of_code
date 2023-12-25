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
    fn pick_random(&self, n: usize) -> usize {
        // prefer nodes below n
        // this assumes that the min-cuts do not start / end at source/sink
        let under_n = self.edges.iter().enumerate().filter(|(ind, (l, r))| *l < n || *r < n).map(|(ind, _)| ind).collect::<Vec<_>>();
        if !under_n.is_empty() {
            let r = rand::thread_rng().gen_range(0..under_n.len());
            under_n[r]
        } else {
            rand::thread_rng().gen_range(0..self.edges.len())
        }
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

#[aoc(day25, part1)]
fn part1(graph: &Graph) -> usize {
    loop {
        let mut g = graph.clone();
        let n = g.vcnt;
        for _ in 0..g.vcnt - 2 {
            g.merge(g.pick_random(n));
        }
        if g.edges.len() == 3{
            let mut n = g.nodes.iter();
            return *n.next().unwrap().1 * *n.next().unwrap().1;
        }
    }
}
