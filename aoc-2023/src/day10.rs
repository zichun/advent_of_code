use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}
impl Dir {
    fn complement(&self) -> Self {
        match self {
            Dir::Up => Self::Down,
            Dir::Right => Self::Left,
            Dir::Down => Self::Up,
            Dir::Left => Self::Right,
        }
    }
    fn include(&self, c: char) -> bool {
        match self {
            Dir::Up => ['S', '|', 'J', 'L'].contains(&c),
            Dir::Right => ['S', '-', 'F', 'L'].contains(&c),
            Dir::Down => ['S', '|', 'F', '7'].contains(&c),
            Dir::Left => ['S', '-', 'J', '7'].contains(&c),
        }
    }
    fn can_go(&self, r: usize, c: usize, mr: usize, mc: usize) -> bool {
        match self {
            Dir::Up => r > 0,
            Dir::Right => c + 1 < mc,
            Dir::Down => r + 1 < mr,
            Dir::Left => c > 0,
        }
    }
    fn go(&self, r: usize, c: usize, mr: usize, mc: usize) -> Option<(usize, usize)> {
        if !self.can_go(r, c, mr, mc) {
            None
        } else {
            Some(match self {
                Dir::Up => (r - 1, c),
                Dir::Right => (r, c + 1),
                Dir::Down => (r + 1, c),
                Dir::Left => (r, c - 1),
            })
        }
    }
}
struct Grid {
    max_row: usize,
    max_col: usize,
    start: (usize, usize),
    map: Vec<Vec<char>>,
}
impl Grid {
    fn can_go(&self, r0: usize, c0: usize, dir: Dir) -> Option<(usize, usize)> {
        match dir.go(r0, c0, self.max_row, self.max_col) {
            Some((nr, nc)) => {
                let from = self.map[r0][c0];
                let to = self.map[nr][nc];
                if dir.include(from) && dir.complement().include(to) {
                    Some((nr, nc))
                } else {
                    None
                }
            }
            None => None,
        }
    }

    // Expands the grid by 2 and add connecting pipes
    fn expand(&self) -> Self {
        let map = (0..self.max_row * 2).map(|r| {
            (0..self.max_col * 2).map(|c| {
                let c = if r % 2 == 0 && c % 2 == 0 {
                    self.map[r / 2][c / 2]
                } else if r % 2 == 1 && c % 2 == 1 || r + 1 == self.max_row * 2 || c + 1 == self.max_col * 2 {
                    '.'
                } else if r % 2 == 1 {
                    let (top, bot) = (self.map[r / 2][c / 2], self.map[r / 2 + 1][c / 2]);
                    if Dir::Down.include(top) && Dir::Up.include(bot) {
                        '|'
                    } else {
                        '.'
                    }
                } else {
                    let (left, right) = (self.map[r / 2][c / 2], self.map[r / 2][c / 2 + 1]);
                    if Dir::Right.include(left) && Dir::Left.include(right) {
                        '-'
                    } else {
                        '.'
                    }
                };
                c
            }).collect()
        }).collect();
        Self {
            max_row: self.max_row * 2,
            max_col: self.max_col * 2,
            start: (self.start.0 * 2, self.start.1 * 2),
            map,
        }
    }
}

#[aoc_generator(day10)]
fn parse(inp: &str) -> Grid {
    let mut start = (0, 0);
    let map: Vec<Vec<char>> = inp
        .lines()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == 'S' {
                        start = (row, col);
                    }
                    c
                })
                .collect::<Vec<char>>()
        })
        .collect();

    let (max_row, max_col) = (map.len(), map[0].len());
    Grid {
        max_row,
        max_col,
        start,
        map,
    }
}

#[aoc(day10, part1)]
fn part1(g: &Grid) -> i32 {
    let mut max_steps = 0;
    let mut q = VecDeque::new();
    let mut visited = vec![vec![false; g.max_col]; g.max_row];

    q.push_back((g.start, 0));
    visited[g.start.0 as usize][g.start.1 as usize] = true;
    while !q.is_empty() {
        let ((r, c), steps) = q.pop_front().unwrap();
        max_steps = max_steps.max(steps);
        for dir in [Dir::Up, Dir::Right, Dir::Down, Dir::Left] {
            match g.can_go(r, c, dir) {
                Some((nr, nc)) => {
                    if !visited[nr][nc] {
                        visited[nr][nc] = true;
                        q.push_back(((nr, nc), steps + 1));
                    }
                }
                None => continue,
            }
        }
    }
    max_steps
}

#[aoc(day10, part2)]
fn part2(g: &Grid) -> u32 {
    let mut q = VecDeque::new();
    let g = g.expand();
    let mut visited = vec![vec![false; g.max_col]; g.max_row];

    // copy of Part1 - fence finding
    q.push_back((g.start, 0));
    visited[g.start.0 as usize][g.start.1 as usize] = true;
    while !q.is_empty() {
        let ((r, c), steps) = q.pop_front().unwrap();
        for dir in [Dir::Up, Dir::Right, Dir::Down, Dir::Left] {
            match g.can_go(r, c, dir) {
                Some((nr, nc)) => {
                    if !visited[nr][nc] {
                        visited[nr][nc] = true;
                        q.push_back(((nr, nc), steps + 1));
                    }
                }
                None => continue,
            }
        }
    }

    fn flood_fill(g: &Grid, visited: &mut Vec<Vec<bool>>, r: usize, c: usize) -> Option<u32> {
        let mut q = VecDeque::new();
        let mut sum = 0;
        let mut edge = false;
        q.push_back((r, c));
        visited[r][c] = true;
        while !q.is_empty() {
            let (r, c) = q.pop_front().unwrap();
            if r % 2 == 0 && c % 2 == 0 {
                sum += 1;
            }
            for dir in [Dir::Up, Dir::Right, Dir::Down, Dir::Left] {
                match dir.go(r, c, g.max_row, g.max_col) {
                    Some((nr, nc)) => {
                        if visited[nr][nc] {
                            continue;
                        }
                        visited[nr][nc] = true;
                        q.push_back((nr, nc));
                    }
                    None => {
                        edge = true;
                    }
                }
            }
        }
        if edge {
            None
        } else {
            Some(sum)
        }
    }

    let mut sum = 0;
    for r in 0..g.max_row {
        for c in 0..g.max_col {
            if !visited[r][c] {
                sum += match flood_fill(&g, &mut visited, r, c) {
                    Some(s) => s,
                    None => 0
                };
            }
        }
    }
    sum
}
