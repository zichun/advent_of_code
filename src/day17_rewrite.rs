// Re-write of day17 based on implementations of others
use std::collections::HashSet;
use std::collections::HashMap;

//type Vector = (i32, i32, i32, i32);

#[derive(PartialEq, Eq, Hash, Debug)]
struct Vector(Vec<i32>);

struct ConwayCube {
    actives: HashSet<Vector>
}

fn cartesian(inp: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if inp.len() == 0 {
        vec![vec![]]
    } else {
        let mut iter = inp.into_iter();
        let first = iter.next().unwrap();
        let others = iter.map(|x| x.to_owned()).collect::<Vec<_>>();
        let others_cart = cartesian(&others);

        first.iter().flat_map(move |&el| {
            let mut to = others_cart.clone();
            for other in &mut to {
                other.push(el);
            }
            to
        })
            .collect::<Vec<_>>()
    }
}

impl ConwayCube {
    fn parse(input: &str, dimensions: usize) -> Self {
        assert!(dimensions >= 3);
        let actives = input.lines().
            enumerate().flat_map(|(y, row)| {
                row.chars().enumerate().filter_map(move |(x, c)| match c {
                    '.' => None,
                    '#' => Some(Vector(vec![x as i32, y as i32].into_iter()
                                       .chain(vec![0; dimensions - 2].into_iter())
                                       .collect())),
                    e => panic!("Unexpected {}", e)
                })
            })
            .collect::<HashSet<Vector>>();
        ConwayCube { actives }
    }
    fn next(&mut self) {
        let mut neighbors: HashMap<Vector, usize> = HashMap::new();

        let n = self.actives.iter().take(1).next().unwrap().0.len();
        let dim = vec![0, -1, 1];
        let cart = cartesian(&(0..n).into_iter()
                  .map(move |_| dim.clone())
                  .collect::<Vec<_>>())
            .into_iter()
            .filter(|modifier| modifier.iter().filter(|&x| *x != 0).count() > 0)
            .collect::<Vec<_>>();

        for vector in &self.actives {
            cart.iter().for_each(|modifier| {
                let mut x = vector.0.clone();
                for i in 0..x.len() {
                    x[i] += modifier[i];
                }
                
                let v = Vector(x);
                *neighbors.entry(v).or_insert(0) += 1;
            });
        }
        let new_actives = neighbors.into_iter()
            .filter(|(pos, count)| {
                match (self.actives.contains(pos), count) {
                    (true, 2) | (_, 3) => true,
                    _ => false
                }
            })
            .map(|(pos, _)| pos)
            .collect();
        self.actives = new_actives;
    }
    fn count(&self) -> usize {
        self.actives.len()
    }
}

pub fn day17_1(input: &str) -> usize {
    let mut cube = ConwayCube::parse(input, 3);
    for _ in 0..6 {
        cube.next();
    }
    cube.count()
}

pub fn day17_2(input: &str) -> usize {
    let mut cube = ConwayCube::parse(input, 4);
    for _ in 0..6 {
        cube.next();
    }
    cube.count()
}

#[test]
fn test_day17_1() {
    let inp = ".#.
..#
###";
    assert_eq!(day17_1(inp), 112);
}

#[test]
fn test_day17_2() {
    let inp = ".#.
..#
###";
    assert_eq!(day17_2(inp), 848);
}
