use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone, Default)]
pub struct Graph<T> {
    pub vertices: HashSet<T>,
    pub in_degree: HashMap<T, usize>,
    pub out_degree: HashMap<T, usize>,
    pub adj_list: HashMap<T, HashSet<T>>,
}

impl<T> Graph<T>
where T: std::hash::Hash + Default + Eq + Copy
{
    pub fn new() -> Self {
        Graph::default()
    }

    pub fn add_edge(&mut self, a: T, b: T) -> bool {
        self.vertices.insert(a);
        self.vertices.insert(b);

        self.adj_list.entry(b).or_default();
        let out = self.adj_list.entry(a).or_default();
        if out.contains(&b) {
            return false;
        }
        out.insert(b);
        *self.in_degree.entry(b).or_default() += 1;
        *self.out_degree.entry(a).or_default() += 1;
        self.in_degree.entry(a).or_default();
        self.out_degree.entry(b).or_default();
        true
    }

    pub fn vertices_cnt(&self) -> usize {
        self.vertices.len()
    }
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
