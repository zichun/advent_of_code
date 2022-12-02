use std::collections::VecDeque;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct TileData {
    index: u64,
    masks: Vec<(u32, u32)>,
    orientation: usize,
    flip_hor: bool,
    flip_ver: bool
}

struct Tile {
    tile: Vec<Vec<bool>>,
    data: TileData
}

fn mask(arr: &[bool]) -> (u32, u32) {
    let (mut fwd, mut bck) = (0, 0);
    for i in 0..arr.len() {
        fwd = (fwd << 1) | (arr[i] as u32);
        bck = (bck << 1) | (arr[arr.len() - 1 - i] as u32);
    }
    (fwd, bck)
}
impl Tile {
    fn parse(tile: &str) -> Self {
        let mut inp = tile.lines();
        let index = inp.next().unwrap().split(" ").skip(1).next().unwrap()
            .split(":").next().unwrap()
            .parse::<u64>().unwrap();
        let tile = inp.map(|line| {
            line.chars().map(|c| c == '#').collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        let top = mask(&tile[0]);
        let left = mask(&tile.iter().map(|x| x[0]).rev().collect::<Vec<_>>());
        let bot = mask(&tile[tile.len() - 1].iter().rev().map(|x| x.to_owned()).collect::<Vec<_>>());
        let right = mask(&tile.iter().map(|x| x[x.len() - 1]).collect::<Vec<_>>());

        Tile { tile,
               data: TileData { index, masks: vec![top, right, bot, left],
                                orientation: 0, flip_hor: false, flip_ver: false }}
    }

    fn flip(&self, orientation: u8, flip_hor: bool, flip_ver: bool) -> TileData {
        let mut new_mask = Vec::new();
        for i in 0..4 {
            new_mask.push(self.data.masks[((i + orientation) % 4) as usize]);
        }
        let swap = |el: &mut (u32, u32)| {
            let t = el.0;
            el.0 = el.1;
            el.1 = t;
        };
        if flip_hor {
            new_mask.swap(1, 3);
            new_mask.iter_mut().for_each(|x| swap(x));
        }
        if flip_ver {
            new_mask.swap(0, 2);
            new_mask.iter_mut().for_each(|x| swap(x));
        }

        TileData { index: self.data.index,
                   masks: new_mask,
                   orientation: orientation as usize,
                   flip_hor,
                   flip_ver }
    }
}
#[derive(PartialEq, Eq, Debug)]
enum Solution {
    Break,
    Tile(TileData)
}

struct TileSolver<'a> {
    tiles: &'a Vec<Tile>,
    solution: VecDeque<Solution>,
    used: Vec<bool>,
    n: usize,
    final_map: Vec<Vec<bool>>
}

impl<'a> TileSolver<'a> {
    pub fn solve(tiles: &'a Vec<Tile>) -> Option<Vec<TileData>> {
        let n  = (tiles.len() as f32).sqrt().floor() as usize;
        let mut solver = TileSolver {
            tiles,
            solution: VecDeque::new(),
            used: vec![false; tiles.len()],
            n,
            final_map: vec![]
        };

        if !solver.recur(0, 0) {
            None
        } else {
            let soln = solver.solution.iter().filter(|&x| {
                *x != Solution::Break
            }).map(|x| {
                match x {
                    Solution::Break => panic!("boo"),
                    Solution::Tile(t) => t.to_owned()
                }
            }).collect::<Vec<_>>();
            Some(soln)
        }
    }

    fn recur(&mut self, r: usize, c: usize) -> bool {
        if r >= self.n {
            return true;
        }

        let (mut nr, mut nc) = (r, c + 1);
        if nc >= self.n {
            nc = 0;
            nr += 1;
        }
        for i in 0..self.tiles.len() {
            if self.used[i] {
                continue;
            }
            for orientation in 0..4 {
                for &flip_hor in [false, true].iter() {
                    for &flip_ver in [false, true].iter() {
                        let new_tile = self.tiles[i].flip(orientation, flip_hor, flip_ver);

                        if self.can_place_front(&new_tile) {
                            self.used[i] = true;
                            self.solution.push_front(Solution::Tile(new_tile.clone()));

                            let (pf, pb) = if self.solution.len() == self.n {
                                self.solution.push_front(Solution::Break);
                                self.solution.push_back(Solution::Break);
                                (1, 1)
                            }
                            else if self.solution.len() > self.n && self.solution.iter().take(self.n).filter(|&x| *x == Solution::Break).count() == 0 {
                                self.solution.push_front(Solution::Break);
                                (1, 0)
                            }
                            else {
                                (0, 0)
                            };

                            if self.recur(nr, nc) {
                                return true;
                            }
                            self.solution.pop_front();
                            for _ in 0..pf { self.solution.pop_front(); }
                            for _ in 0..pb { self.solution.pop_back(); }
                            self.used[i] = false;
                        }

                        if self.can_place_back(&new_tile) {
                            self.used[i] = true;
                            self.solution.push_back(Solution::Tile(new_tile));

                            let (pf, pb) = if self.solution.len() == self.n {
                                self.solution.push_front(Solution::Break);
                                self.solution.push_back(Solution::Break);
                                (1, 1)
                            }
                            else if self.solution.len() > self.n && self.solution.iter().rev().take(self.n).filter(|&x| *x == Solution::Break).count() == 0 {
                                self.solution.push_back(Solution::Break);
                                (0, 1)
                            }
                            else {
                                (0, 0)
                            };

                            if self.recur(nr, nc) {
                                return true;
                            }
                            self.solution.pop_front();
                            for _ in 0..pf { self.solution.pop_front(); }
                            for _ in 0..pb { self.solution.pop_back(); }
                            self.used[i] = false;
                        }

                    }
                }
            }
        }
        return false;
    }

    fn can_place_front(&self, new_tile: &TileData) -> bool {
        if self.solution.len() > 0 {
            if let Solution::Tile(t) = &self.solution[0] {
                let left_mask = t.masks[3].1;
                let right_mask = new_tile.masks[1].0;
                if right_mask != left_mask {
                    return false;
                }
            }
        }
        if self.solution.len() > self.n {
            if let Solution::Tile(t) = &self.solution[self.n] {
                let bottom_mask = new_tile.masks[2].1;
                let top_mask = t.masks[0].0;
                if bottom_mask != top_mask {
                    return false
                }
            } else {
                panic!("Unexpected");
            }
        }

        true
    }

    fn can_place_back(&self, new_tile: &TileData) -> bool {
        if self.solution.len() > 0 {
            if let Solution::Tile(t) = &self.solution[self.solution.len() - 1] {
                let left_mask = new_tile.masks[3].1;
                let right_mask = t.masks[1].0;
                if right_mask != left_mask {
                    return false;
                }
            }
        }
        if self.solution.len() > self.n {
            if let Solution::Tile(t) = &self.solution[self.solution.len() - 1 - self.n] {
                let bottom_mask = t.masks[2].1;
                let top_mask = new_tile.masks[0].0;
                if bottom_mask != top_mask {
                    return false
                }
            } else {
                panic!("Unexpected");
            }
        }

        true
    }
}

pub fn day20_1(input: &str) -> u64 {
    let tiles = input.split("\n\n")
        .flat_map(|x| x.split("\r\n\r\n"))
        .map(|chunk| {
            Tile::parse(chunk)
        }).collect::<Vec<_>>();

    let soln = TileSolver::solve(&tiles).unwrap();
    let n = soln.len();
    let m  = (n as f32).sqrt().floor() as usize;
    soln[0].index * soln[m - 1].index *
        soln[n - 1].index * soln[n - m].index

}

fn rotate_vector(vec: &Vec<(usize, usize)>, rotate: usize, flip_ver: bool, flip_hor: bool) -> Vec<(usize, usize)> {
    fn rotate_coord(coord: (usize, usize), rotate: usize, flip_ver: bool, flip_hor: bool) -> (i32, i32) {
        let mut tr = (coord.0 as i32, coord.1 as i32);
        for _ in 0..rotate {
            tr = (-tr.1, tr.0);
        }
        if flip_hor {
            tr = (tr.0, -tr.1);
        }
        if flip_ver {
            tr = (-tr.0, tr.1);
        }
        tr
    }
    fn normalize_coords(vec: Vec<(i32, i32)>) -> Vec<(usize, usize)> {
        let (min_y, min_x) = vec.iter()
            .fold((std::i32::MAX, std::i32::MAX),
                  |(min_y, min_x), el| {
                      (min_y.min(el.0), min_x.min(el.1))});
        vec.iter().map(|(y, x)| ((y - min_y) as usize, (x - min_x) as usize)).collect::<Vec<_>>()
    }
    let tr = vec.iter().map(|coord| rotate_coord(*coord, rotate, flip_ver, flip_hor)).collect::<Vec<_>>();
    normalize_coords(tr)
}

fn find_monsters(map: &Vec<Vec<bool>>) -> u32 {
    let monsters = "                  #
#    ##    ##    ###
 #  #  #  #  #  #   ";
    let monsters = monsters.lines()
        .enumerate()
        .flat_map(|(r, line)|
                  line.chars().enumerate().filter_map(move |(c, ch)|
                                                      if ch == '#' { Some((r, c)) } else { None }))
        .collect::<Vec<_>>();
    let cnt = map.iter().flat_map(|row| row.iter().filter(|x| **x)).count();
    fn find(monsters: &Vec<(usize, usize)>, map: &Vec<Vec<bool>>) -> u32 {
        let (max_y, max_x) = monsters.iter()
            .fold((0 as usize, 0 as usize),
                  |(max_y, max_x), el| {
                      (max_y.max(el.0), max_x.max(el.1))});
        let is_monster = |y: usize, x: usize| -> bool {
            monsters.iter().all(|(m_y, m_x)|
                                map[y + m_y][x + m_x] == true)
        };

        let n = map.len();
        let mut tr = 0;
        for y in 0..(n - max_y) {
            for x in 0..(n - max_x) {
                if is_monster(y, x) {
                    tr += 1;
                }
            }
        }
        tr
    }

    for rotate in 0..4 {
        for &flip_ver in vec![false, true].iter() {
            for &flip_hor in vec![false, true].iter() {
                let monsters_rotate = rotate_vector(&monsters, rotate, flip_ver, flip_hor);
                let v = find(&monsters_rotate, map);
                if v > 0 {
                    return cnt as u32 - (v * monsters.len() as u32);
                }
            }
        }
    }
    0
}
pub fn day20_2(input: &str) -> u32 {
    let tiles = input.split("\n\n")
        .flat_map(|x| x.split("\r\n\r\n"))
        .map(|chunk| {
            Tile::parse(chunk)
        }).collect::<Vec<_>>();

    //
    // Stitch map together.
    //

    let soln = TileSolver::solve(&tiles).unwrap();
    let n = soln.len();
    let m  = (n as f32).sqrt().floor() as usize;
    let grid_size = tiles[0].tile.len() - 2;
    let mut index = 0;

    let mut final_map = vec![vec![false; grid_size * m]; grid_size * m];

    for r in 0..m {
        for c in 0..m {
            let tile = tiles.iter().find(|x| x.data.index == soln[index].index).unwrap();
            // copy tile.tile into [r * grid_size][c * grid_size]
            let rotate_tile = rotate_grid(&tile.tile, soln[index].orientation, soln[index].flip_hor, soln[index].flip_ver);
            for rr in 0..grid_size {
                for cc in 0..grid_size {
                    final_map[r * grid_size + rr][c * grid_size + cc] = rotate_tile[rr + 1][cc + 1];
                }
            }
            index += 1;
        }
    }

    find_monsters(&final_map)
}

fn rotate_grid(vec: &Vec<Vec<bool>>, rotate: usize, flip_hor: bool, flip_ver: bool) -> Vec<Vec<bool>> {
    let grid_rotate = |n: usize, r: usize, c: usize, orientation: usize, flip_hor: bool, flip_ver: bool| -> (usize, usize)
    {
        let (mut rr, mut cc) = (r, c);
        if flip_hor {
            cc = n - 1 - cc;
        }
        if flip_ver {
            rr = n - 1 - rr;
        }
        for _ in 0..orientation {
            let trr = rr;
            rr = cc;
            cc = n - 1 - trr;
        }
        (rr, cc)
    };
    let n = vec.len();
    let mut tr = vec![vec![false; n]; n];
    for r in 0..n {
        for c in 0..n {
            let (rr, cc) = grid_rotate(n, r, c, rotate, flip_hor, flip_ver);
            tr[r][c] = vec[rr][cc];
        }
    }
    tr
}
fn print_grid(vec: &Vec<Vec<bool>>)
{
    let n = vec.len();
    for r in 0..n {
        for c in 0..n {
            print!("{}", if vec[r][c] { "#" } else { "." });
        }
        println!("");
    }
    println!("");
}

#[test]
fn test_day20() {
    let inp = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
    assert_eq!(day20_1(inp), 20899048083289);
    assert_eq!(day20_2(inp), 273);
}
