use std::collections::VecDeque;

type Coord = (usize, usize);
type Matrix<T> = Vec<Vec<T>>;

fn parse(input: &str) -> (Coord, Coord, Matrix<u32>) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let map = input.lines().enumerate().map(
        |(r, l)| l.chars().enumerate().map(|(c, ch)|
                          match ch {
                              'a'..='z' => ch as u32 - 'a' as u32,
                              'S' => {
                                  start = (r, c);
                                  0
                              },
                              'E' => {
                                  end = (r, c);
                                  'z' as u32 - 'a' as u32
                              },
                              _ => unimplemented!()
                          }).collect::<Vec<_>>()
    ).collect::<Vec<_>>();
    (start, end, map)
}

fn movable(coord: Coord, map: &Matrix<u32>, visited: &Matrix<bool>) -> Vec<Coord> {
    fn can_go(a: u32, b: u32) -> bool {
        b as i32 - a as i32 <= 1
    }
    let mut tr = Vec::new();
    let (row_size, col_size) = (map.len(), map[0].len());
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == dc || dr * -1 == dc {
                continue;
            }
            let (r, c) = (coord.0 as i32 + dr, coord.1 as i32 + dc);
            if r < 0 || c < 0 || r >= row_size as i32 || c >= col_size as i32{
                continue;
            }
            let (r, c) = (r as usize, c as usize);
            if visited[r][c] {
                continue;
            }
            if can_go(map[coord.0][coord.1], map[r][c]) {
                tr.push((r, c).clone());
            }
        }
    }
    tr
}

fn bfs(q: &mut VecDeque<(Coord, usize)>, map: &Matrix<u32>, end: Coord) -> usize {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    while !q.is_empty() {
        let (coord, dist) = q.pop_front().unwrap();
        if coord == end {
            return dist;
        }
        movable(coord, &map, &visited).iter().for_each(|(r, c)| {
            visited[*r][*c] = true;
            q.push_back(((*r, *c), dist + 1));
        });
    }
    panic!("cannot find end");
}

pub fn part1(input: &str) -> usize {
    let (start, end, map) = parse(input);
    let mut q: VecDeque<(Coord, usize)> = VecDeque::new();
    q.push_back((start, 0));
    bfs(&mut q, &map, end)
}

pub fn part2(input: &str) -> usize {
    let (start, end, map) = parse(input);
    let mut q: VecDeque<(Coord, usize)> = VecDeque::new();
    q.push_back((start, 0));
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == 0 {
                q.push_back(((r, c), 0));
            }
        }
    }
    bfs(&mut q, &map, end)
}

#[test]
fn test() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    assert_eq!(part1(input), 31);
    assert_eq!(part2(input), 29);
}
