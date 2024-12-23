use crate::prelude::*;


#[aoc(day23, part1)]
fn part1(inp: &str) -> usize {
    let g = inp.lines().fold(Graph::new(), |mut acc, el| {
        let mut tok = el.split('-');
        let (a, b) = (tok.next().unwrap(), tok.next().unwrap());
        acc.add_edge(a, b);
        acc.add_edge(b, a);
        acc
    });
    let r = g.adj_list.iter().flat_map(|(v, edges)| {
        edges.iter().flat_map(|v2| {
            g.vertices.iter().filter_map(|v3| {
                if v3 != v && v3 != v2 && g.is_connected(v, v3) && g.is_connected(v2, v3) &&
                    (v.starts_with('t') || v2.starts_with('t') || v3.starts_with('t')) {
                    Some((*v, *v2, *v3))
                } else {
                    None
                }
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    r.len() / 6
}

#[aoc(day23, part2)]
fn part2(inp: &str) -> String {
    let g = inp.lines().fold(Graph::new(), |mut acc, el| {
        let mut tok = el.split('-');
        let (a, b) = (tok.next().unwrap(), tok.next().unwrap());
        acc.add_edge(a, b);
        acc.add_edge(b, a);
        acc
    });
    let mut c = get_max_cliques(&g);
    c.sort_by_key(|b| std::cmp::Reverse(b.len()));
    let mut el = c[0].iter().collect::<Vec<_>>();
    el.sort();
    el.into_iter().join(",")
}
