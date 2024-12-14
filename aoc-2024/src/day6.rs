use crate::prelude::*;

#[derive(Clone)]
struct Blocks {
    rows: Vec<Vec<usize>>,
    cols: Vec<Vec<usize>>,
}

// pre-compute indices of blocks so that we can fast-forward and find the next block in a given dir
fn compute_blocks(g: &Grid<char>) -> Blocks {
    let rows = g
        .rows()
        .map(|r| {
            r.enumerate()
                .filter_map(|(ind, ch)| if *ch == '#' { Some(ind) } else { None })
                .collect::<Vec<_>>()
        })
        .collect();

    let cols = g
        .cols()
        .map(|c| {
            c.enumerate()
                .filter_map(|(ind, ch)| if *ch == '#' { Some(ind) } else { None })
                .collect::<Vec<_>>()
        })
        .collect();

    Blocks { rows, cols }
}

#[aoc_generator(day6)]
fn parse(input: &str) -> (Grid<char>, Blocks, (usize, usize, Direction)) {
    let g: Grid<char> = Grid::from_str(input).unwrap();

    let ((r, c), dir) = g
        .iter()
        .find(|(_, el)| Direction::from_str(&el.to_string()).is_ok())
        .unwrap();
    let dir = Direction::from_str(&dir.to_string()).unwrap();
    let blocks = compute_blocks(&g);

    (g, blocks, (r, c, dir))
}

fn simulate(
    g: &Grid<char>,
    b: &Blocks,
    mut r: isize,
    mut c: isize,
    mut dir: Direction,
    zoom: bool,
) -> (Vec<(usize, usize, Direction)>, bool) {
    let mut path = Vec::new();
    let (rr, cc) = g.dimensions();
    let mut visited = HashSet::new();
    visited.insert((r, c, dir));
    let mut escaped = true;

    loop {
        path.push((r as usize, c as usize, dir));
        let (nr, nc) = dir.go(r, c);
        if nr < 0 || nc < 0 || nr >= rr as isize || nc >= cc as isize {
            break;
        } else if *g.get(nr as usize, nc as usize) == '#' {
            dir = dir.next();
        } else if zoom {
            let next_block = match dir {
                Direction::Up => b.cols[c as usize].iter().rev().find(|bl| **bl < r as usize),
                Direction::Right => b.rows[r as usize].iter().find(|bl| **bl > c as usize),
                Direction::Down => b.cols[c as usize].iter().find(|bl| **bl > r as usize),
                Direction::Left => b.rows[r as usize].iter().rev().find(|bl| **bl < c as usize),
            };
            match next_block {
                Some(ind) => {
                    (r, c) = match dir {
                        Direction::Up => (*ind as isize + 1, c),
                        Direction::Down => (*ind as isize - 1, c),
                        Direction::Right => (r, *ind as isize - 1),
                        Direction::Left => (r, *ind as isize + 1),
                    };
                }
                None => break,
            }
        } else {
            (r, c) = (nr, nc);
        }

        if visited.contains(&(r, c, dir)) {
            escaped = false;
            break;
        } else {
            visited.insert((r, c, dir));
        }
    }

    (path, escaped)
}

#[aoc(day6, part1)]
fn part1((g, b, (r, c, dir)): &(Grid<char>, Blocks, (usize, usize, Direction))) -> usize {
    let path = simulate(g, b, *r as isize, *c as isize, *dir, false).0;
    path.into_iter().map(|(r, c, _)| (r, c)).unique().count()
}

#[aoc(day6, part2)]
fn part2((g, b, (r, c, dir)): &(Grid<char>, Blocks, (usize, usize, Direction))) -> usize {
    let mut g = g.clone();
    let mut b = b.clone();
    let path = simulate(&g, &b, *r as isize, *c as isize, *dir, false).0;

    let (rr, cc) = g.dimensions();
    let (ir, ic, dir) = path[0];

    let candidates = path
        .iter()
        .filter_map(|(r, c, dir)| {
            let (r, c) = (*r as isize, *c as isize);
            let (nr, nc) = dir.go(r, c);
            if nr >= 0
                && nc >= 0
                && nr < rr as isize
                && nc < cc as isize
                && *g.get(nr as usize, nc as usize) == '.'
            {
                Some((nr as usize, nc as usize))
            } else {
                None
            }
        })
        .unique()
        .collect::<Vec<_>>();

    candidates
        .into_iter()
        .filter(|(r, c)| {
            if *g.get(*r, *c) == '.' {
                g.set(*r, *c, '#');
                b.rows[*r].push(*c);
                b.cols[*c].push(*r);
                b.rows[*r].sort();
                b.cols[*c].sort();

                let (_, escaped) = simulate(&g, &b, ir as isize, ic as isize, dir, true);

                g.set(*r, *c, '.');
                let ind = b.rows[*r].binary_search(c).unwrap();
                b.rows[*r].remove(ind);
                let ind = b.cols[*c].binary_search(r).unwrap();
                b.cols[*c].remove(ind);

                !escaped
            } else {
                false
            }
        })
        .count()
}
