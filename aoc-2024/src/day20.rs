use crate::prelude::*;

#[derive(Clone, Copy, Eq, PartialEq)]
enum El {
    Space,
    Wall
}

struct Inp {
    grid: Grid<El>,
    from_start: HashMap<(usize, usize), usize>,
    to_end: HashMap<(usize, usize), usize>,
    sr: usize, sc: usize,
}

fn bfs_from(grid: &Grid<El>, r: usize, c: usize) -> HashMap<(usize, usize), usize> {
    let mut bfs = Bfs::<(usize, usize)>::new();
    bfs.visit((r, c), 0);
    while let Some(((r, c), dist)) = bfs.pop() {
        grid.reachables(r, c, Direction::iter()).for_each(|(rr, cc)| {
            if *grid.get(rr, cc) != El::Wall {
                bfs.visit((rr, cc), dist + 1);
            }
        });
    }
    bfs.get_distance()
}
#[aoc_generator(day20)]
fn parse(inp: &str) -> Inp {
    let (mut sr, mut sc, mut er, mut ec) = (0, 0, 0, 0);
    let v = inp.lines().enumerate().map(|(r, l)| l.chars().enumerate().map(|(c, ch)| {
        if ch == 'S' {
            (sr, sc) = (r, c);
        } else if ch == 'E' {
            (er, ec) = (r, c);
        }
        if ch == '#' {
            El::Wall
        } else {
            El::Space
        }
    }).collect()).collect();
    let grid = Grid(v);

    let to_end = bfs_from(&grid, er, ec);
    let from_start = bfs_from(&grid, sr, sc);

    Inp {
        grid,
        from_start,
        to_end,
        sr, sc,
    }
}

fn solve(inp: &Inp, cheat_max: usize, savings: usize) -> usize {
    let max_dist = *inp.to_end.get(&(inp.sr, inp.sc)).unwrap() - savings;
    inp.grid.iter().map(|((r, c), el)| {
        let mut tr = 0;
        match inp.from_start.get(&(r, c)) {
            Some(start_dist) => {
                // the folowing could be made more generic, but expanding it out allows for short-circuiting
                [Direction::Left, Direction::Right].iter().enumerate().for_each(|(ind, leftright_dir)| {
                    for i in ind..=cheat_max {
                        let (mut rr, mut cc) = (r as isize, c as isize);
                        (rr, cc) = leftright_dir.go_n(rr, cc, i);
                        if !inp.grid.contains(rr, cc) {
                            break;
                        }
                        [Direction::Up, Direction::Down].iter().enumerate().for_each(|(ind, updown_dir)| {
                            let sind = (2 - i.min(2)).max(ind);
                            for j in sind..=cheat_max - i {
                                let (mut rr, mut cc) = (r as isize, c as isize);
                                (rr, cc) = leftright_dir.go_n(rr, cc, i);
                                (rr, cc) = updown_dir.go_n(rr, cc, j);
                                if !inp.grid.contains(rr, cc) {
                                    break;
                                }
                                if let Some(end_dist) = inp.to_end.get(&(rr as usize, cc as usize)) {
                                    if start_dist + end_dist + i + j <= max_dist {
                                        tr += 1;
                                    }
                                }
                            }
                        });
                    }
                });
                tr
            },
            None => 0,
        }
    }).sum()
}

#[aoc(day20, part1)]
fn part1(inp: &Inp) -> usize {
    solve(inp, 2, 100)
}

#[aoc(day20, part2)]
fn part2(inp: &Inp) -> usize {
    solve(inp, 20, 100)
}
