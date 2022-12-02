type Matrix = Vec<Vec<Vec<Vec<bool>>>>;
struct ConwayCube {
    x_range: usize,
    y_range: usize,
    z_range: usize,
    w_range: usize,
    actives: Matrix
}

impl<'a> ConwayCube {
    fn new(input: &str) -> Self {
        let actives = input.lines().map(|x| {
            x.chars().map(|y| y == '#').collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        let x_range = actives[0].len();
        let y_range = actives.len();
        let z_range = 1;
        let w_range = 1;
        let actives = vec![vec![actives]];

        ConwayCube { x_range, y_range, z_range, w_range, actives }
    }

    fn is_translate_active(&self, x: usize, y: usize, z: usize, w: usize) -> bool {
        if w >= self.w_range || z >= self.z_range || y == 0 || y > self.y_range || x == 0 || x > self.x_range  {
            false
        } else {
            self.actives[w][z][y - 1][x - 1]
        }
    }

    fn count(&self) -> u32 {
        let mut tr = 0;
        for w in 0..self.w_range {
            for z in 0..self.z_range {
                for y in 0..self.y_range {
                    for x in 0..self.x_range {
                        if self.actives[w][z][y][x] {
                            let mut to_add = 1;
                            if z > 0 { to_add *= 2; }
                            if w > 0 { to_add *= 2; }
                            tr += to_add;
                        }
                    }
                }
            }
        }
        tr
    }

    fn next(&mut self) {
        let x_range = self.x_range + 2;
        let y_range = self.y_range + 2;
        let z_range = self.z_range + 1;
        let mut actives = vec![vec![vec![vec![false; x_range]; y_range]; z_range]];

        for z in 0..z_range {
            for y in 0..y_range {
                for x in 0..x_range {
                    let is_active = self.is_translate_active(x, y, z, 0);
                    let besides = self.besides(x, y, z);

                    actives[0][z][y][x] = if is_active {
                        besides == 2 || besides == 3
                    } else {
                        besides == 3
                    }
                }
            }
        }

        self.x_range = x_range;
        self.y_range = y_range;
        self.z_range = z_range;
        self.actives = actives;
    }

    fn besides(&self, x: usize, y: usize, z: usize) -> u8 {
        let mut tr = 0;
        for dz in -1..=1 {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue;
                    }

                    let zp = if z == 0 && dz == -1 {
                        z + 1
                    } else {
                        (z as i32 + dz) as usize
                    };
                    let xp = (x as i32 + dx) as usize;
                    let yp = (y as i32 + dy) as usize;

                    if self.is_translate_active(xp, yp, zp, 0) {
                        tr += 1;
                    }
                }
            }
        }
        tr
    }

    fn next_2(&mut self) {
        let x_range = self.x_range + 2;
        let y_range = self.y_range + 2;
        let z_range = self.z_range + 1;
        let w_range = self.w_range + 1;
        let mut actives = vec![vec![vec![vec![false; x_range]; y_range]; z_range]; w_range];

        for w in 0..w_range {
            for z in 0..z_range {
                for y in 0..y_range {
                    for x in 0..x_range {
                        let is_active = self.is_translate_active(x, y, z, w);
                        let besides = self.besides_2(x, y, z, w);

                        actives[w][z][y][x] = if is_active {
                            besides == 2 || besides == 3
                        } else {
                            besides == 3
                        }
                    }
                }
            }
        }

        self.x_range = x_range;
        self.y_range = y_range;
        self.z_range = z_range;
        self.w_range = w_range;
        self.actives = actives;
    }

    fn besides_2(&self, x: usize, y: usize, z: usize, w: usize) -> u8 {
        let mut tr = 0;
        for dw in -1..=1 {
            for dz in -1..=1 {
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                            continue;
                        }

                        let wp = if w == 0 && dw == -1 {
                            w + 1
                        } else {
                            (w as i32 + dw) as usize
                        };
                        let zp = if z == 0 && dz == -1 {
                            z + 1
                        } else {
                            (z as i32 + dz) as usize
                        };
                        let xp = (x as i32 + dx) as usize;
                        let yp = (y as i32 + dy) as usize;

                        if self.is_translate_active(xp, yp, zp, wp) {
                            tr += 1;
                        }
                    }
                }
            }
        }
        tr
    }

}


pub fn day17_1(input: &str) -> u32 {
    let mut cube = ConwayCube::new(input);
    for _ in 0..6 {
        cube.next();
    }
    cube.count()
}

pub fn day17_2(input: &str) -> u32 {
    let mut cube = ConwayCube::new(input);
    for _ in 0..6 {
        cube.next_2();
    }
    cube.count()
}

#[test]
fn test_day17_1() {
    let inp = ".#.
..#
###";
    assert_eq!(day17_1(inp), 112);
}

#[test]
fn test_day17_2() {
    let inp = ".#.
..#
###";
    assert_eq!(day17_2(inp), 848);
}
