use crate::prelude::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct Coord(usize, usize, usize);
impl Coord {
    fn from(inp: &str) -> Self {
        let mut inp = inp.split(",");
        Coord(inp.next_token(), inp.next_token(), inp.next_token())
    }
}
impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
impl Ord for Coord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.2.cmp(&other.2)
            .then_with(|| self.0.cmp(&other.0))
            .then_with(|| self.1.cmp(&other.1))
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct Bricks {
    from: Coord,
    to: Coord,
}

fn overlap(from0: usize, to0: usize, from1: usize, to1: usize) -> bool {
    (from0 <= from1 && from1 <= to0) ||
        (from0 <= to1 && to1 <= to0) ||
        (from1 <= from0 && to1 >= to0)
}

#[derive(Clone, Debug)]
struct Map {
    bricks: BTreeMap<usize, Vec<Bricks>>,
}
impl Map {
    fn from(b: Vec<Bricks>) -> Map {
        let mut bricks: BTreeMap<usize, Vec<Bricks>> = BTreeMap::new();
        b.iter().enumerate().for_each(|(ind, brick)| {
            bricks.entry(brick.to.2).or_default().push(*brick);
        });
        Self {
            bricks,
        }
    }
    fn drop(&mut self) -> usize {
        let mut fall = 0;
        loop {
            let mut changed = false;
            let all_z = self.bricks.iter().map(|(z, _)| *z).collect::<Vec<_>>();
            for curz in all_z {
                let mut i = 0;
                while i < self.bricks[&curz].len() {
                    let brick = &self.bricks[&curz][i];
                    if brick.from.2 == 1 {
                        i += 1;
                        continue;
                    }

                    let (x0, y0) = (brick.from.0, brick.from.1);
                    let (x1, y1) = (brick.to.0, brick.to.1);
                    let mut z = brick.from.2;
                    for nz in (1..=(z-1)).rev() {
                        if self.has(x0, y0, x1, y1, nz) {
                            break;
                        }
                        z = nz;
                    }
                    i += 1;
                    if z != brick.from.2 {
                        let mut brick = brick.clone();
                        i -= 1;
                        self.bricks.get_mut(&curz).unwrap().remove(i);
                        fall += 1;
                        changed = true;
                        brick.to.2 -= brick.from.2.abs_diff(z);
                        brick.from.2 = z;
                        self.bricks.entry(brick.to.2).or_default().push(brick);
                    }
                }
            }
            if !changed {
                break;
            }
        }
        fall
    }
    fn has(&self, x0: usize, y0: usize, x1: usize, y1: usize, z: usize) -> bool {
        match self.bricks.get(&z) {
            Some(bricks) => bricks.iter().find(|brick| {
                brick.from.2 <= z && brick.to.2 >= z &&
                    overlap(x0, x1, brick.from.0, brick.to.0) &&
                    overlap(y0, y1, brick.from.1, brick.to.1)
            }).is_some(),
            None => false
        }
    }
}

#[aoc_generator(day22)]
fn parse(inp: &str) -> Vec<Bricks> {
    inp.lines().map(|l| {
        let mut l = l.split("~");
        let (from, to) = (Coord::from(l.next().unwrap()),
                          Coord::from(l.next().unwrap()));
        Bricks {
            from: from.min(to),
            to: from.max(to),
        }
    }).collect::<Vec<_>>()
}

#[aoc(day22, part1)]
fn part1(inp: &[Bricks]) -> usize {
    let mut inp = Map::from(inp.to_vec());
    inp.drop();

    inp.bricks.iter().map(|(z, bricks)| {
        (0..bricks.len()).filter(|i| {
            let mut inp = inp.clone();
            inp.bricks.get_mut(z).unwrap().remove(*i);
            inp.drop() == 0
        }).count()
    }).sum()
}

#[aoc(day22, part2)]
fn part2(inp: &[Bricks]) -> usize {
    let mut inp = Map::from(inp.to_vec());
    inp.drop();

    inp.bricks.iter().map(|(z, bricks)| {
        (0..bricks.len()).map(|i| {
            let mut inp = inp.clone();
            inp.bricks.get_mut(z).unwrap().remove(i);
            inp.drop()
        }).sum::<usize>()
    }).sum::<usize>()
}
