use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

type Cubes = Vec<Vec<i32>>;

fn parse(input: &str) -> Cubes {
    input.lines().map(|l| {
        l.split(",").map(|n| n.parse().unwrap()).collect()
    }).collect()
}
pub fn part1(input: &str) -> usize {
    let cubes = parse(input);
    let set: HashSet<Vec<i32>> = cubes.iter().cloned().collect();

    cubes.iter().map(|c| {
        let mut sides = 6;
        for del in vec![-1, 1] {
            for i in 0..3 {
                let mut adj_cube = c.clone();
                adj_cube[i] += del;
                if set.contains(&adj_cube) {
                    sides -= 1;
                }
            }
        }
        sides
    }).sum()
}

fn inside(cube: &[i32], min: &[i32], max: &[i32]) -> bool {
    for i in 0..3 {
        if cube[i] < min[i] - 1 || cube[i] > max[i] + 1 {
            return false;
        }
    }
    true
}

pub fn part2(input: &str) -> usize {
    let cubes = parse(input);
    let set: HashSet<Vec<i32>> = cubes.iter().cloned().collect();

    let mut max_ext = [i32::min_value(); 3];
    let mut min_ext = [i32::max_value(); 3];
    for i in 0..3 {
        cubes.iter().for_each(|c| {
            max_ext[i] = max_ext[i].max(c[i]);
            min_ext[i] = min_ext[i].min(c[i]);
        })
    }

    let mut visited: HashSet<Vec<i32>> = HashSet::new();
    let mut q: VecDeque<Vec<i32>> = VecDeque::new();
    let start = vec![min_ext[0] - 1, min_ext[1] - 1, min_ext[2] - 1];
    visited.insert(start.clone());
    q.push_back(start);

    while !q.is_empty() {
        let c = q.pop_front().unwrap();
        for del in vec![-1, 1] {
            for i in 0..3 {
                let mut adj_cube = c.clone();
                adj_cube[i] += del;
                if inside(&adj_cube, &min_ext, &max_ext) && !visited.contains(&adj_cube)
                    && !set.contains(&adj_cube) {
                        visited.insert(adj_cube.clone());
                        q.push_back(adj_cube);
                    }
            }
        }
    }

    cubes.iter().map(|c| {
        let mut ext_sides = 0;
        for del in vec![-1, 1] {
            for i in 0..3 {
                let mut adj_cube = c.clone();
                adj_cube[i] += del;

                if visited.contains(&adj_cube) {
                    ext_sides += 1;
                }
            }
        }
        ext_sides
    }).sum()
}

#[test]
fn test() {
    let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
    assert_eq!(part1(input), 64);
    assert_eq!(part2(input), 58);
}
