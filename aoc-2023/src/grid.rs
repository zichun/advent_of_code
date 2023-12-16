use enum_iterator::{all, first, last, next, previous, All, Sequence};

#[derive(Clone, Copy, PartialEq, Eq, Sequence)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    pub fn go(&self, r: isize, c: isize) -> (isize, isize) {
        let (dr, dc) = self.as_delta();
        (r + dr, c + dc)
    }
    pub fn iter() -> All<Direction> {
        all::<Self>()
    }
    pub fn opp(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
    pub fn next(&self) -> Self {
        match next(self) {
            None => first::<Self>().unwrap(),
            Some(d) => d,
        }
    }
    pub fn prev(&self) -> Self {
        match previous(self) {
            None => last::<Self>().unwrap(),
            Some(d) => d,
        }
    }
    pub fn is_updown(&self) -> bool {
        *self == Direction::Up || *self == Direction::Down
    }
    pub fn is_leftright(&self) -> bool {
        *self == Direction::Left || *self == Direction::Right
    }
    pub fn as_delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
    pub fn ind(&self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T>(pub Vec<Vec<T>>);

#[allow(dead_code)]
impl<T> Grid<T> {
    pub fn dimensions(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }
    pub fn dimensions_with_rot(&self, cw_times: usize) -> (usize, usize) {
        let (mr, mc) = self.dimensions();
        if cw_times % 2 == 0 {
            (mr, mc)
        } else {
            (mc, mr)
        }
    }

    pub fn print(&self)
    where
        T: std::fmt::Display,
    {
        self.0.iter().for_each(|r| {
            r.iter().for_each(|c| print!("{}", c));
            println!();
        });
    }

    pub fn transpose(&self) -> Self
    where
        T: Copy,
    {
        Grid(self.cols().map(|r| r.copied().collect()).collect())
    }

    pub fn set(&mut self, r: usize, c: usize, t: T) {
        self.0[r][c] = t;
    }
    pub fn set_with_rot(&mut self, r: usize, c: usize, cw_times: usize, t: T) {
        let (r, c) = self.coord_with_rot(r, c, cw_times);
        self.0[r][c] = t;
    }

    pub fn get<'a>(&'a self, r: usize, c: usize) -> &'a T {
        &self.0[r][c]
    }
    pub fn coord_with_dir(&self, r: usize, c: usize, dir: Direction) -> Option<(usize, usize)> {
        let (r, c) = dir.go(r as isize, c as isize);
        let (mr, mc) = self.dimensions();
        if r < 0 || c < 0 || r >= mr as isize || c >= mc as isize {
            None
        } else {
            Some((r as usize, c as usize))
        }
    }
    pub fn coord_with_rot(&self, r: usize, c: usize, cw_times: usize) -> (usize, usize) {
        let cw_times = cw_times % 4;
        let (mr, _) = self.dimensions_with_rot(cw_times);
        match cw_times {
            0 => (r, c),
            1 => (c, r),
            2 => (mr - r - 1, c),
            3 => (c, mr - r - 1),
            _ => unreachable!(),
        }
    }
    pub fn get_with_rot<'a>(&'a self, r: usize, c: usize, cw_times: usize) -> &'a T {
        let (r, c) = self.coord_with_rot(r, c, cw_times);
        &self.0[r][c]
    }

    pub fn cols(&self) -> GridLinesIter<T> {
        GridLinesIter {
            g: self,
            i: 0,
            row_iter: false,
        }
    }
    pub fn rows(&self) -> GridLinesIter<T> {
        GridLinesIter {
            g: self,
            i: 0,
            row_iter: true,
        }
    }
    pub fn col(&self, c: usize) -> GridLineIter<T> {
        let max_c = self.dimensions().1;
        assert!(c < max_c);
        GridLineIter {
            g: self,
            r: 0,
            c,
            row_iter: false,
        }
    }
    pub fn row(&self, r: usize) -> GridLineIter<T> {
        let max_r = self.dimensions().0;
        assert!(r < max_r);
        GridLineIter {
            g: self,
            r,
            c: 0,
            row_iter: true,
        }
    }
}

impl<T> std::str::FromStr for Grid<T>
where
    T: From<char>,
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grid(
            s.lines()
                .map(|l| l.chars().map(|c| c.into()).collect())
                .collect(),
        ))
    }
}

pub struct GridLinesIter<'a, T> {
    g: &'a Grid<T>,
    i: usize,
    row_iter: bool,
}
impl<'a, T> Iterator for GridLinesIter<'a, T> {
    type Item = GridLineIter<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (rr, cc) = self.g.dimensions();
        if self.row_iter {
            if self.i >= rr {
                None
            } else {
                self.i += 1;
                Some(self.g.row(self.i - 1))
            }
        } else {
            if self.i >= cc {
                None
            } else {
                self.i += 1;
                Some(self.g.col(self.i - 1))
            }
        }
    }
}

pub struct GridLineIter<'a, T> {
    g: &'a Grid<T>,
    r: usize,
    c: usize,
    row_iter: bool,
}
impl<'a, T> Iterator for GridLineIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let (rr, cc) = self.g.dimensions();
        if self.row_iter {
            if self.c >= cc {
                None
            } else {
                self.c += 1;
                Some(&self.g.0[self.r][self.c - 1])
            }
        } else {
            if self.r >= rr {
                None
            } else {
                self.r += 1;
                Some(&self.g.0[self.r - 1][self.c])
            }
        }
    }
}
