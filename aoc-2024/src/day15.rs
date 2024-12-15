use crate::prelude::*;

struct Input {
    grid: Grid<char>,
    cmds: Vec<Direction>
}
#[aoc_generator(day15)]
fn parse(inp: &str) -> Input {
    let mut tok = inp.split("\n\n");
    let grid = Grid::from_str(tok.next().unwrap()).unwrap();
    let cmds = tok.next().unwrap().lines().collect::<String>().chars().map(|c| Direction::from_str(&c.to_string()).unwrap()).collect();
    Input { grid, cmds }
}

//
// Part 1
//

#[aoc(day15, part1)]
fn part1(inp: &Input) -> usize {
    let mut grid = inp.grid.clone();
    let ((mut r, mut c), _) = grid.iter().find(|(_, ch)| **ch == '@').unwrap();

    inp.cmds.iter().for_each(|dir| {
        let (mut nr, mut nc) = (r, c);
        for _ in 1.. {
            match grid.coord_with_dir(nr, nc, *dir) {
                Some((nnr, nnc)) => {
                    let ch = *grid.get(nnr, nnc);
                    if ch == '#' {
                        break;
                    } else if ch == '.' || ch == '@' {
                        grid.set(nnr, nnc, 'O');
                        (r, c) = grid.coord_with_dir(r, c, *dir).unwrap();
                        grid.set(r, c, '.');
                        break;
                    } else {
                        (nr, nc) = (nnr, nnc);
                    }
                },
                None => break
            }
        }
    });

    grid.iter().filter_map(|((r, c), ch)| {
        if *ch == 'O' {
            Some(100 * r + c)
        } else {
            None
        }
    }).sum()
}

//
// Part 2
//

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(i8)]
enum El {
    Empty,
    BoxLeft,
    BoxRight,
    Wall,
}
impl El {
    fn beside(&self, c: usize) -> usize {
        match self {
            El::BoxLeft => c + 1,
            El::BoxRight => c - 1,
            _ => unreachable!()
        }
    }
}

impl std::fmt::Display for El { // for printing
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            El::Empty => ".",
            El::Wall => "#",
            El::BoxLeft => "[",
            El::BoxRight => "]",
        })
    }
}

#[aoc(day15, part2)]
fn part2(inp: &Input) -> usize {
    let ((mut r, mut c), _) = inp.grid.iter().find(|(_, ch)| **ch == '@').unwrap();
    c *= 2;

    let mut grid = Grid(inp.grid.rows().map(|r| {
        r.flat_map(|c| {
            if *c == '@' || *c == '.' {
                vec![El::Empty, El::Empty]
            } else if *c == '#' {
                vec![El::Wall, El::Wall]
            } else {
                vec![El::BoxLeft, El::BoxRight]
            }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>());

    fn simulate(grid: &mut Grid<El>, dir: Direction, r: usize, c: usize) -> bool {
        let (nr, nc) = grid.coord_with_dir(r, c, dir).unwrap();
        let next = *grid.get(nr, nc);
        let tr = match next {
            El::Empty => true,
            El::Wall => false,
            El::BoxLeft | El::BoxRight => if dir.is_updown() {
                simulate(grid, dir, nr, nc) && simulate(grid, dir, nr, next.beside(nc))
            } else {
                simulate(grid, dir, nr, nc)
            },
        };
        grid.set(nr, nc, *grid.get(r, c));
        if dir.is_updown() && (next == El::BoxLeft || next == El::BoxRight) {
            grid.set(nr, next.beside(nc), El::Empty);
        }
        tr
    }
    inp.cmds.iter().for_each(|dir| {
        let mut simul_grid = grid.clone();
        if simulate(&mut simul_grid, *dir, r, c) {
            grid = simul_grid;
            (r, c) = grid.coord_with_dir(r, c, *dir).unwrap();
        }
    });

    grid.iter().filter_map(|((r, c), el)| {
        if *el == El::BoxLeft {
            Some(100 * r + c)
        } else {
            None
        }
    }).sum()
}
