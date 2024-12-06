use crate::prelude::*;

fn parse(input: &str) -> (Grid<char>, Vec<(usize, usize, Direction)>, usize) {
    let g: Grid<char> = Grid::from_str(input).unwrap();

    let ((r, c), dir) = g.iter().find(|(_, el)| {
        Direction::from_str(&el.to_string()).is_ok()
    }).unwrap();
    let dir = Direction::from_str(&dir.to_string()).unwrap();

    let (r, c) = (r as isize, c as isize);
    let (path, cnt, _) = simulate(&g, r, c, dir);

    (g, path, cnt)
}

fn simulate(g: &Grid<char>, mut r: isize, mut c: isize, mut dir: Direction) -> (Vec<(usize, usize, Direction)>, usize, bool) {
    let mut path = Vec::new();
    let mut cnt = 0;
    let (rr, cc) = g.dimensions();
    let mut visited_with_dir = vec![vec![vec![false; 4]; cc]; rr];
    let mut visited = vec![vec![false; cc]; rr];
    visited[r as usize][c as usize] = true;
    visited_with_dir[r as usize][c as usize][dir.ind()] = true;
    let mut escaped = true;

    loop {
        path.push((r as usize, c as usize, dir));
        let (nr, nc) = dir.go(r, c);
        if nr < 0 || nc < 0 || nr >= rr as isize || nc >= cc as isize {
            break;
        } else if *g.get(nr as usize, nc as usize) == '#' {
            dir = dir.next();
        } else {
            (r, c) = (nr, nc);
            if !visited[r as usize][c as usize] {
                visited[r as usize][c as usize] = true;
                cnt += 1;
            }
        }

        if visited_with_dir[r as usize][c as usize][dir.ind()] {
            escaped = false;
            break;
        } else {
            visited_with_dir[r as usize][c as usize][dir.ind()] = true;
        }
    }

    (path, cnt, escaped)
}

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    parse(input).2
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    let (mut g, path, _) = parse(input);
    let (rr, cc) = g.dimensions();
    let (ir, ic, dir) = path[0];

    let candidates = path.iter().filter_map(|(r, c, dir)| {
        let (r, c) = (*r as isize, *c as isize);
        let (nr, nc) = dir.go(r, c);
        if nr >= 0 && nc >= 0 && nr < rr as isize && nc < cc as isize &&
            *g.get(nr as usize, nc as usize) == '.' {
                Some((nr as usize, nc as usize))
            } else {
                None
            }
    }).unique().collect::<Vec<_>>();

    candidates.into_iter().filter(|(r, c)| {
        if *g.get(*r, *c) == '.' {
            g.set(*r, *c, '#');
            let (_, _, escaped) = simulate(&g, ir as isize, ic as isize, dir);
            g.set(*r, *c, '.');
            !escaped
        } else {
            false
        }
    }).count()
}
