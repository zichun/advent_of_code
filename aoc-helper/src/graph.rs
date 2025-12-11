use std::collections::{HashSet, HashMap};
use rand::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Graph<T> {
    pub vertices: HashSet<T>,
    pub in_degree: HashMap<T, usize>,
    pub out_degree: HashMap<T, usize>,
    pub adj_list: HashMap<T, HashSet<T>>,
    pub adj_matrix: HashSet<(T, T)>,
}

impl<T> Graph<T>
where T: std::hash::Hash + Default + Eq + Clone
{
    pub fn new() -> Self {
        Graph::default()
    }

    pub fn add_edge(&mut self, a: T, b: T) -> bool {
        self.vertices.insert(a.clone());
        self.vertices.insert(b.clone());

        self.adj_list.entry(b.clone()).or_default();
        let out = self.adj_list.entry(a.clone()).or_default();
        if out.contains(&b) {
            return false;
        }
        out.insert(b.clone());
        *self.in_degree.entry(b.clone()).or_default() += 1;
        *self.out_degree.entry(a.clone()).or_default() += 1;
        self.in_degree.entry(a).or_default();
        self.out_degree.entry(b).or_default();
        true
    }

    pub fn is_connected(&self, a: T, b: T) -> bool {
        self.adj_list[&a].contains(&b)
    }

    pub fn vertices_cnt(&self) -> usize {
        self.vertices.len()
    }
}
pub fn get_max_cliques<T>(g: &Graph<T>) -> Vec<HashSet<T>>
where T: std::hash::Hash + Eq + Copy + Default {
    fn bron_kerbosh<T>(g: &Graph<T>, r: &mut HashSet<T>, mut p: HashSet<T>, mut x: HashSet<T>, cliques: &mut Vec<HashSet<T>>)
    where T: std::hash::Hash + Eq + Copy {
        if p.is_empty() && x.is_empty() {
            cliques.push(r.clone());
            return;
        }
        let mut rng = rand::thread_rng();
        let rind = f64::floor(rng.gen::<f64>() * (p.len() + x.len()) as f64) as usize;
        let u = if rind < p.len() {
            p.iter().nth(rind).unwrap()
        } else {
            x.iter().nth(rind - p.len()).unwrap()
        };

        let pivoted = p.difference(&g.adj_list[u]).copied().collect::<Vec<_>>();
        for v in pivoted {
            r.insert(v);
            let np = p.intersection(&g.adj_list[&v]).copied().collect::<HashSet<_>>();
            let nx = x.intersection(&g.adj_list[&v]).copied().collect::<HashSet<_>>();
            bron_kerbosh(g, r, np, nx, cliques);
            r.remove(&v);
            p.remove(&v);
            x.insert(v);
        }
    }
    let mut r = HashSet::new();
    let mut cliques = Vec::new();
    bron_kerbosh(g, &mut r, g.vertices.clone(), HashSet::new(), &mut cliques);
    cliques
}

#[derive(Default)]
struct Tarj<T> {
    ind: usize,
    sccs: Vec<HashSet<T>>,
    low: HashMap<T, usize>,
    indices: HashMap<T, usize>,
    stack: Vec<T>,
    on_stack: HashSet<T>
}

pub fn get_scc<T>(g: &Graph<T>) -> Vec<HashSet<T>>
where T: std::hash::Hash + Eq + Copy + Default {
    let mut tarj = Tarj::<T>::default();

    fn recur<T>(g: &Graph<T>, v: T, tarj: &mut Tarj<T>)
    where T: std::hash::Hash + Eq + Copy {
        tarj.indices.insert(v, tarj.ind);
        tarj.low.insert(v, tarj.ind);
        tarj.ind += 1;
        tarj.stack.push(v);
        tarj.on_stack.insert(v);

        for w in g.adj_list[&v].iter() {
            if !tarj.indices.contains_key(w) {
                recur(g, *w, tarj);
                let (low_v, low_w) = (tarj.low[&v], tarj.low[w]);
                tarj.low.insert(v, low_v.min(low_w));
            } else if tarj.on_stack.contains(w) {
                let (low_v, ind_w) = (tarj.low[&v], tarj.indices[w]);
                tarj.low.insert(v, low_v.min(ind_w));
            }
        }

        if tarj.low[&v] == tarj.indices[&v] {
            let mut scc = HashSet::new();
            loop {
                let w = tarj.stack.pop().unwrap();
                tarj.on_stack.remove(&w);
                scc.insert(w);
                if w == v {
                    break;
                }
            }
            tarj.sccs.push(scc);
        }
    }

    for v in &g.vertices {
        if !tarj.indices.contains_key(v) {
            recur(g, *v, &mut tarj);
        }
    }
    tarj.sccs.clone()
}

pub fn toposort<T>(g: &Graph<T>) -> Option<Vec<T>>
where T: std::hash::Hash + Eq + Copy + Default + std::fmt::Display
{
    let mut visited = HashMap::new();
    let mut indices = Vec::new();
    fn dfs<T>(node: &T, g: &Graph<T>, indices: &mut Vec<T>, visited: &mut HashMap<T, u8>) -> bool
    where T: std::hash::Hash + Eq + Copy + std::fmt::Display
    {
        if let Some(v) = visited.get(node) {
            if *v == 1 { // cycle found
                return false;
            }
            return true;
        }
        visited.insert(*node, 1);
        if !g.adj_list[node].iter().all(|v| {
            dfs(v, g, indices, visited)
        }) {
            false
        } else {
            indices.push(*node);
            visited.insert(*node, 2);
            true
        }
    }

    if g.vertices.iter().all(|v| {
        if *g.in_degree.get(v).unwrap() == 0 {
            dfs(v, g, &mut indices, &mut visited)
        } else {
            true
        }
    }) && indices.len() == g.vertices_cnt() {
        indices.reverse();
        Some(indices)
    } else {
        None
    }
}
