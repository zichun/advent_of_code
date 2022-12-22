const DT: &[(isize, isize)] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];

fn mv(r: isize, c: isize, dir: usize) -> (isize, isize) {
    (r + DT[dir].0, c + DT[dir].1)
}
fn get<T: Copy>(arr: &[Vec<T>], r: isize, c: isize) -> Option<T> {
    if r < 0 || r >= arr.len() as isize ||
        c < 0 || c >= arr[r as usize].len() as isize {
        None
    } else {
        Some(arr[r as usize][c as usize])
    }
}

enum Movement {
    Move(isize),
    Right,
    Left,
}
struct Map{
    map: Vec<Vec<char>>,
    cube: Cube
}

struct Cube {
    faces: Vec<(usize, usize, Vec<Option<(usize, usize)>>)>,
    matrix: Vec<Vec<Option<usize>>>,
    dimension: usize,
}

impl Cube {
    fn new(map: &Vec<Vec<char>>) -> Self {
        let max_r = map.len();
        let max_c = map[0].len(); // assumed map has been normalized
        let dimension = ((map.iter().map(|l| l.iter().filter(|c| **c != ' ').count()).sum::<usize>() / 6) as f64).sqrt() as usize;

        let mut faces = Vec::new();
        let mut matrix = Vec::new();
        for r in (0..max_r).step_by(dimension) {
            let mut inner = Vec::new();
            for c in (0..max_c).step_by(dimension) {
                inner.push(if map[r][c] == ' ' {
                    None
                } else {
                    Some(faces.len())
                });
                if map[r][c] != ' ' {
                    faces.push((r / dimension, c / dimension, vec![None, None, None, None]));
                }
            }
            matrix.push(inner);
        }

        fn find(matrix: &[Vec<Option<usize>>], r: usize, c: usize, dir: usize) -> Option<(usize, usize)> {
            // try finding one adj away
            for step in vec![1, 3] {
                let inner_dir = (dir + step) % 4;
                let mut cnt = 0;
                let (mut rr, mut cc) = (r as isize, c as isize);
                loop {
                    let (irr, icc) = mv(rr as isize, cc as isize, dir);
                    if irr < 0 || irr >= matrix.len() as isize || icc < 0 || icc >= matrix[irr as usize].len() as isize {
                        break;
                    }
                    if let Some(Some(ind)) = get(matrix, irr, icc) {
                        return Some((ind, (dir + cnt) % 4));
                    }
                    (rr, cc) = mv(rr, cc, inner_dir);
                    cnt += step;
                }
            }
            None
        }

        // find initial adjacency using direct connections
        for i in 0..faces.len() {
            for dir in 0..4 {
                let (r, c, _) = faces[i];
                let connector = find(&matrix, r, c, dir);
                faces[i].2[dir] = connector;
                if let Some(conn) = connector {
                    faces[conn.0].2[(conn.1 + 2) % 4] = Some((i, (dir + 2) % 4));
                }
            }
        }

        // find adjacency through transitivity (e.g right of a face == bottom right or top right with rotation)
        for i in 0..faces.len() {
            for dir in 0..4 {
                let conn = faces[i].2[dir];
                if conn.is_none() {
                    if let Some(down_conn) = faces[i].2[(dir + 1) % 4] {
                        let nextdir = (down_conn.1 + 3) % 4;
                        if let Some(right_conn) = faces[down_conn.0].2[nextdir] {
                            faces[i].2[dir] = Some((right_conn.0, (right_conn.1 + 1) % 4));
                            continue;
                        }
                    }
                    if let Some(up_conn) = faces[i].2[(dir + 3) % 4] {
                        let nextdir = (up_conn.1 + 1) % 4;
                        if let Some(right_conn) = faces[up_conn.0].2[nextdir] {
                            faces[i].2[dir] = Some((right_conn.0, (right_conn.1 + 3) % 4));
                            continue;
                        }
                    }
                    panic!("can't find");
                }
            }
        }

        Cube { faces, matrix, dimension }
    }

    fn get_offset(&self, r: isize, c: isize, dir: usize) -> usize {
        let (cube_r, cube_c) = (r / self.dimension as isize, c / self.dimension as isize);
        let (top_r, top_c) = (cube_r * self.dimension as isize, cube_c * self.dimension as isize);

        (match dir {
            0 => r - top_r, // right
            1 => top_c + self.dimension as isize - 1 - c, // down
            2 => top_r + self.dimension as isize - 1 - r, // left
            3 => c - top_c, // up
            _ => unimplemented!()
        }) as usize
    }
    fn get_new_coord(&self, cube_ind: usize, offset: usize, dir: usize) -> (usize, usize) {
        let (cube_r, cube_c, _) = self.faces[cube_ind];
        let (top_r, top_c) = (cube_r * self.dimension, cube_c * self.dimension);
        match dir {
            0 => (top_r + offset, top_c), // right
            1 => (top_r, top_c + self.dimension - 1 - offset), // down
            2 => (top_r + self.dimension - 1 - offset, top_c + self.dimension - 1), // left
            3 => (top_r + self.dimension - 1, top_c + offset), // up
            _ => unimplemented!()
        }
    }

    fn mv(&self, r: isize, c: isize, dir: usize) -> (isize, isize, usize) {
        let (rr, cc) = mv(r, c, dir);
        let (cube_r, cube_c) = (r / self.dimension as isize, c / self.dimension as isize);
        let cube_ind = self.matrix[cube_r as usize][cube_c as usize].unwrap();

        if rr < 0 || cc < 0 || rr / self.dimension as isize != cube_r || cc / self.dimension as isize != cube_c {
            let (nind, ndir) = self.faces[cube_ind].2[dir].unwrap();
            let offset = self.get_offset(r, c, dir);
            let (rr, cc) = self.get_new_coord(nind, offset, ndir);
            (rr as isize, cc as isize, ndir)
        } else {
            (rr, cc, dir)
        }
    }
}

impl Map {
    fn print_debug(&self) {
        self.map.iter().for_each(|row| {
            println!("{}", row.iter().collect::<String>());
        });
        println!("");
    }
    fn new(mut map: Vec<Vec<char>>) -> Self {
        // normalize map (expand width to match max width)
        let max_c = map.iter().map(|l| l.len()).max().unwrap();
        map.iter_mut().for_each(|row| while row.len() < max_c { row.push(' ') });

        let cube = Cube::new(&map);
        Map {
            map,
            cube
        }
    }
    fn oob_cube(&self, r: isize, c: isize, dir: usize) -> (isize, isize, usize) {
        self.cube.mv(r, c, dir)
    }
    fn oob(&self, mut mr: isize, mut mc: isize, dir: usize) -> (isize, isize) {
        if DT[dir].0 != 0 && (mr < 0 || mr >= self.map.len() as isize) {
            mr = mr.rem_euclid(self.map.len() as isize);
        }
        if DT[dir].1 != 0 && (mc < 0 || mc >= self.map[mr as usize].len() as isize) {
            mc = mc.rem_euclid(self.map[mr as usize].len() as isize);
        }
        while mr < 0 || mr >= self.map.len() as isize || mc < 0 || mc >= self.map[mr as usize].len() as isize || self.map[mr as usize][mc as usize] == ' ' {
            (mr, mc) = mv(mr, mc, dir);
            if DT[dir].0 != 0 {
                mr = mr.rem_euclid(self.map.len() as isize);
            } else {
                mc = mc.rem_euclid(self.map[mr as usize].len() as isize);
            }
        }
        (mr, mc)
    }
}

fn parse(input: &str) -> (Map, Vec<Movement>) {
    let mut iter = input.split("\n\n");
    let map = iter.next().unwrap()
        .lines().map(|l| l.chars().collect()).collect();

    let mut movements = Vec::new();
    let mut acc = 0;
    iter.next().unwrap()
        .chars().for_each(|c| {
            if c.is_alphabetic() {
                if acc > 0 {
                    movements.push(Movement::Move(acc));
                }
                acc = 0;
                movements.push(if c == 'R' {
                    Movement::Right
                } else {
                    Movement::Left
                });
            } else if c.is_numeric() {
                acc = acc * 10 + ((c as u8 - '0' as u8) as isize);
            }
        });
    if acc > 0 {
        movements.push(Movement::Move(acc));
    }
    (Map::new(map), movements)
}

pub fn part1(input: &str) -> u64 {
    let (mut map, movements) = parse(input);

    let dirc = vec!['>', 'V', '<', '^'];
    let (mut r, mut c, mut dir) = (0 as isize, map.map[0].iter().position(|c| *c == '.').unwrap() as isize, 0);

    movements.iter().for_each(|m| {
        match m {
            Movement::Move(times) => {
                for _ in 0..*times {
                    let (mr, mc) = mv(r, c, dir);
                    let (mr, mc) = map.oob(mr, mc, dir);
                    if map.map[mr as usize][mc as usize] == '#' {
                        break;
                    }
                    map.map[r as usize][c as usize] = dirc[dir];
                    (r, c) = (mr, mc);
                }
            },
            Movement::Right => {
                dir = (dir + 1) % 4;
            },
            Movement::Left => {
                dir = (dir + 3) % 4;
            }
        }
    });
    1000 * (r as u64 + 1) + 4 * (c as u64 + 1) + dir as u64
}

pub fn part2(input: &str) -> u64 {
    let (mut map, movements) = parse(input);
    let (mut r, mut c, mut dir) = (0 as isize, map.map[0].iter().position(|c| *c == '.').unwrap() as isize, 0);

    let dirc = vec!['>', 'V', '<', '^'];

    movements.iter().for_each(|m| {
        match m {
            Movement::Move(times) => {
                for _ in 0..*times {
                    let (mr, mc, ndir) = map.oob_cube(r, c, dir);
                    if map.map[mr as usize][mc as usize] == '#' {
                        break;
                    }
                    map.map[r as usize][c as usize] = dirc[dir];
                    (r, c) = (mr, mc);
                    map.map[r as usize][c as usize] = '*';
                    dir = ndir;
                }
            },
            Movement::Right => {
                dir = (dir + 1) % 4;
            },
            Movement::Left => {
                dir = (dir + 3) % 4;
            }
        }
    });
    1000 * (r as u64 + 1) + 4 * (c as u64 + 1) + dir as u64
}

#[test]
fn test() {
    let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
    assert_eq!(part1(input), 6032);
    assert_eq!(part2(input), 5031);
}
