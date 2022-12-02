use std::fmt;

#[derive(PartialEq)]
enum Grid
{
    SeatEmpty, SeatOccupied, Floor
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Grid::SeatEmpty => write!(f, "L"),
            Grid::SeatOccupied => write!(f, "#"),
            Grid::Floor => write!(f, ".")
        }
    }
}

impl Grid
{
    fn from(c: char) -> Self {
        match c {
            '.' => Grid::Floor,
            'L' => Grid::SeatEmpty,
            '#' => Grid::SeatOccupied,
            _ => panic!("Invalid Grid char")
        }
    }
}

struct SeatMap
{
    map: Vec<Vec<Grid>>,
    tolerant_adj: Vec<Vec<Vec<(usize, usize)>>>
}

impl SeatMap
{
    fn from(input: &str) -> Self
    {
        let map: Vec<Vec<Grid>> = input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| Grid::from(c))
                        .collect()
                }).collect();

        let mut tolerant_adj = Vec::new();
        let rows = map.len();
        let cols = map[0].len();

        for r in 0..rows {
            let mut row = Vec::new();
            for c in 0..cols {
                let mut adj = Vec::new();
                for rd in -1..=1 {
                    for cd in -1..=1 {
                        if rd == 0 && cd == 0 { continue; }
                        let mut nr = r as i32 + rd;
                        let mut nc = c as i32 + cd;
                        while nr >= 0 && nc >= 0 &&
                            nr < rows as i32 && nc < cols as i32
                        {
                            if map[nr as usize][nc as usize] == Grid::SeatEmpty {
                                adj.push((nr as usize, nc as usize));
                                break;
                            }
                            nr += rd;
                            nc += cd;
                        }
                    }
                }
                row.push(adj);
            }
            tolerant_adj.push(row);
        }

        SeatMap { map, tolerant_adj }
    }

    fn get_adj(&self, row: usize, col: usize) -> u32
    {
        let mut tr = 0;
        for r in -1..=1 {
            for c in -1..=1 {
                let nr = row as i32 + r;
                let nc = col as i32 + c;
                if (r == 0 && c == 0) ||
                    nr < 0 || nc < 0 ||
                    nr as usize >= self.map.len() || nc as usize >= self.map[nr as usize].len()
                {
                    continue;
                }
                if self.map[nr as usize][nc as usize] == Grid::SeatOccupied {
                    tr += 1;
                }
            }
        }
        tr
    }

    fn get_adj_part2(&self, row: usize, col: usize) -> u32 {
        self.tolerant_adj[row][col]
            .iter()
            .map(|(r, c)|
                 if self.map[*r][*c] == Grid::SeatOccupied {
                     1
                 } else {
                     0
                 })
            .sum()
    }

    fn next(&mut self) -> bool
    {
        let mut changed = false;
        let rows = self.map.len();
        let cols = self.map[0].len();

        let mut new_map = Vec::new();
        for r in 0..rows
        {
            let mut row = Vec::new();
            for c in 0..cols
            {
                row.push(
                    match self.map[r][c] {
                        Grid::Floor => Grid::Floor,
                        Grid::SeatEmpty => if self.get_adj(r, c) == 0 {
                            Grid::SeatOccupied
                        } else {
                            Grid::SeatEmpty
                        },
                        Grid::SeatOccupied => if self.get_adj(r, c) >= 4 {
                            Grid::SeatEmpty
                        } else {
                            Grid::SeatOccupied
                        }
                    });
                changed = changed || (row[row.len() - 1] != self.map[r][c]);
            }
            new_map.push(row);
        }
        self.map = new_map;
        changed
    }

    fn next_part2(&mut self) -> bool
    {
        let mut changed = false;
        let rows = self.map.len();
        let cols = self.map[0].len();

        let mut new_map = Vec::new();
        for r in 0..rows
        {
            let mut row = Vec::new();
            for c in 0..cols
            {
                row.push(
                    match self.map[r][c] {
                        Grid::Floor => Grid::Floor,
                        Grid::SeatEmpty => if self.get_adj_part2(r, c) == 0 {
                            Grid::SeatOccupied
                        } else {
                            Grid::SeatEmpty
                        },
                        Grid::SeatOccupied => if self.get_adj_part2(r, c) >= 5 {
                            Grid::SeatEmpty
                        } else {
                            Grid::SeatOccupied
                        }
                    });
                changed = changed || (row[row.len() - 1] != self.map[r][c]);
            }
            new_map.push(row);
        }
        self.map = new_map;
        changed
    }

    fn seats(&self) -> u32
    {
        self.map.iter().map(|r| {
            r.iter().map(|c| if *c == Grid::SeatOccupied { 1 } else { 0 }).sum::<u32>()
        }).sum()
    }
}

pub fn day11_1(input: &str) -> u32
{
    let mut map = SeatMap::from(input);
    while map.next() {
    }
    map.seats()
}

pub fn day11_2(input: &str) -> u32
{
    let mut map = SeatMap::from(input);
    while map.next_part2() {
    }
    map.seats()
}

#[test]
fn test_day11_1()
{
    let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    assert_eq!(day11_1(&input), 37);
}

#[test]
fn test_day11_2()
{
    let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    assert_eq!(day11_2(&input), 26);
}
