use crate::prelude::*;

type Grid = crate::prelude::Grid<char>;

#[aoc_generator(day21)]
fn parse(inp: &str) -> Grid {
    Grid::from_str(inp).unwrap()
}

#[aoc(day21, part1)]
fn part1(grid: &Grid) -> usize {
    solve(grid, 64).0
}

#[aoc(day21, part2)]
fn part2(grid: &Grid) -> usize {
    let (_, visited) = solve(grid, 65 + 8 * 131);
    for i in (0..9).step_by(2) {
        let s = 65 + i * 131;
        let soln = visited.iter().filter(|(_, viscnt)| {
            **viscnt <= s && **viscnt % 2 == s % 2
        }).count();
        println!("{} {}", s, soln);
    }
    0
}

#[derive(Debug)]
struct Jmp {
    dir: Direction,
    r: isize,
    c: isize,
    skips: usize,
}

fn solve(grid: &Grid, steps: usize) -> (usize, HashMap<(isize, isize), usize>) {
    let (dimr, dimc) = grid.dimensions();
    let (sr, sc) = grid.iter().find(|(_, ch)| **ch == 'S').unwrap().0;
    let (sr, sc) = (sr as isize, sc as isize);

    let mut q = VecDeque::new();
    let mut visited = HashMap::new();

    q.push_back((sr, sc, 0));
    visited.insert((sr, sc), 0);

    let (minr, maxr) = (-2 * (dimr as isize), 3 * (dimr as isize));
    let (minc, maxc) = (-2 * (dimc as isize), 3 * (dimc as isize));

    while let Some((r, c, dep)) = q.pop_front() {
/*        if r < minr || r >= maxr  || c < minc || c >= maxc {
            continue;
    }*/
        if dep > steps {
            continue;
        }
        Direction::iter().for_each(|dir| {
            let (nr, nc) = dir.go(r, c);
            if *grid.get_with_wrap(nr, nc) != '#' && !visited.contains_key(&(nr, nc)) {
                visited.insert((nr, nc), dep + 1);
                q.push_back((nr, nc, dep + 1));
            }
        });
    }

    // partial implementation for general soln :(

    // find parity count in original grid
    // let mut parity_cnt = [0; 2];
    // grid.iter().for_each(|((r, c), _)| {
    //     if let Some(v) = visited.get(&(r as isize, c as isize)) {
    //         parity_cnt[v % 2] += 1;
    //     }
    // });

    // fn find_max(visited: &HashMap<(isize, isize), usize>, tor: isize, toc: isize, dimr: usize, dimc: usize) -> (isize, isize, usize) {
    //     let (mut maxr, mut maxc) = (tor, toc);
    //     for r in tor..tor + dimr as isize {
    //         for c in toc..toc + dimc as isize {
    //             if let Some(v) = visited.get(&(r, c)) {
    //                 if *v > visited[&(maxr, maxc)] {
    //                     maxr = r;
    //                     maxc = c;
    //                 }
    //             }
    //         }
    //     }
    //     (maxr, maxc, visited[&(maxr, maxc)])
    // }

    // let dir_jmps = Direction::iter().map(|dir| {
    //     // find number of moves for a grid in that dir
    //     let (dr, dc) = dir.as_delta();
    //     let (tor, toc) = (2 * dr * (dimr as isize), 2 * dc * (dimc as isize));
    //     let (maxr, maxc, _) = find_max(&visited, tor, toc, dimr, dimc);

    //     let opdir = dir.opp();
    //     let (dr, dc) = opdir.as_delta();
    //     Jmp {
    //         dir,
    //         r: maxr - tor,
    //         c: maxc - toc,
    //         skips: visited[&(maxr, maxc)] - visited[&(maxr + dr * (dimr as isize), maxc + dc * (dimc as isize))]
    //     }
    // }).collect::<Vec<_>>();

    let maxvisit = visited.iter().map(|(_, viscnt)| viscnt).max().unwrap();
    if steps > *maxvisit {
        // incomplete general soln :(
        todo!()
    } else {
        (visited.iter().filter(|(_, viscnt)| {
            **viscnt <= steps && **viscnt % 2 == steps % 2
        }).count(), visited)
    }
}
