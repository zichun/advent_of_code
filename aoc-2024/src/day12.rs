use crate::prelude::*;

fn floodfill(
    r: usize,
    c: usize,
    g: &Grid<char>,
    vis: &mut HashMap<(usize, usize), usize>,
    tag: usize,
) -> (usize, usize) {
    if vis.contains_key(&(r, c)) {
        return (0, 0);
    }
    vis.insert((r, c), tag);

    let cur = g.get(r, c);
    let mut tr = (1, 0);
    for dir in Direction::iter() {
        let (a, p) = match g.coord_with_dir(r, c, dir) {
            Some((r, c)) => {
                if g.get(r, c) != cur {
                    (0, 1)
                } else {
                    let (a, p) = floodfill(r, c, g, vis, tag);
                    (a, p)
                }
            }
            None => (0, 1),
        };
        tr.0 += a;
        tr.1 += p;
    }
    tr
}

#[aoc(day12, part1)]
fn part1(inp: &str) -> usize {
    let g: Grid<char> = Grid::from_str(inp).unwrap();
    let (rr, cc) = g.dimensions();
    let mut visited = HashMap::new();
    let mut tr = 0;
    for r in 0..rr {
        for c in 0..cc {
            let (area, param) = floodfill(r, c, &g, &mut visited, 0);
            tr += area * param;
        }
    }
    tr
}

#[aoc(day12, part2)]
fn part2(inp: &str) -> usize {
    let g: Grid<char> = Grid::from_str(inp).unwrap();
    let (rr, cc) = g.dimensions();
    let mut visited = HashMap::new();

    let mut tag = 0;
    let mut areas: HashMap<usize, usize> = HashMap::new();
    for r in 0..rr {
        for c in 0..cc {
            let (area, _param) = floodfill(r, c, &g, &mut visited, tag);
            if area > 0 {
                *areas.entry(tag).or_default() += area;
                tag += 1;
            }
        }
    }

    let mut sides: HashMap<usize, usize> = HashMap::new();
    for rot in 0..4 {
        for r in 0..rr {
            let mut prev = usize::MAX;
            for c in 0..cc {
                let (r, c) = g.coord_with_rot(r, c, rot);
                let (nr, nc) = Direction::from_ind(rot).go(r as isize, c as isize);
                let is_fence = if g.contains(nr, nc) {
                    g.get(nr as usize, nc as usize) != g.get(r, c)
                } else {
                    true
                };

                let cur_tag = *visited.get(&(r, c)).unwrap();
                if !is_fence || cur_tag != prev {
                    if is_fence {
                        *sides.entry(cur_tag).or_default() += 1;
                        prev = cur_tag;
                    } else {
                        prev = usize::MAX;
                    }
                }
            }
        }
    }

    areas
        .iter()
        .map(|(tag, area)| area * sides.get(tag).unwrap())
        .sum()
}
