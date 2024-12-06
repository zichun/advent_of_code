use enum_iterator::{all, first, last, next, previous, All, Sequence};

#[derive(Clone, Copy, PartialEq, Eq, Sequence, Debug)]
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
    pub fn from_ind(ind: usize) -> Direction {
        match ind % 4{
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => unreachable!(),
        }
    }
}

impl std::str::FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "E" | "R" | "e" | "r" | ">" => Ok(Direction::Right),
            "N" | "U" | "n" | "u" | "^" => Ok(Direction::Up),
            "W" | "L" | "w" | "l" | "<" => Ok(Direction::Left),
            "S" | "D" | "s" | "d" | "v" => Ok(Direction::Down),
            _ => Err(())
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

    pub fn expand_default(&self) -> Self
    where
        T: Copy + Default,
    {
        self.expand(T::default())
    }

    pub fn expand(&self, el: T) -> Self
    where
        T: Copy
    {
        let (_rr, cc) = self.dimensions();
        let mut rows = Vec::new();
        rows.push(vec![el; cc + 2]);
        self.rows().for_each(|r| {
            let mut row = Vec::new();
            row.push(el);
            row.append(&mut r.copied().collect::<Vec<_>>());
            row.push(el);
            rows.push(row);
        });
        rows.push(vec![el; cc + 2]);
        Grid(rows)
    }

    pub fn mirror_rows(&self) -> Self
    where
        T: Copy,
    {
        let mut rows = self.rows().map(|r| r.copied().collect()).collect::<Vec<_>>();
        rows.reverse();
        Grid(rows)
    }
    pub fn mirror_cols(&self) -> Self
    where
        T: Copy,
    {
        Grid(self.rows().map(|r| {
            let mut cols = r.copied().collect::<Vec<_>>();
            cols.reverse();
            cols
        }).collect())
    }

    pub fn rotate(&self, cw_times: usize) -> Self
    where
        T: Copy,
    {
       let (rr, cc) = self.dimensions_with_rot(cw_times);
        Grid((0..rr).map(|r| {
            (0..cc).map(|c| *self.get_with_rot(r, c, cw_times)).collect()
        }).collect())
    }

    pub fn rotate_cw(&self) -> Self
    where
        T: Copy,
    {
        self.rotate(1)
    }

    pub fn rotate_ccw(&self) -> Self
    where
        T: Copy,
    {
        self.rotate(3)
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

    pub fn get(&self, r: usize, c: usize) -> &T {
        &self.0[r][c]
    }
    pub fn get_with_wrap<'a>(&'a self, r: isize, c: isize) -> &'a T {
        let (mr, mc) = self.dimensions();
        &self.0[r.rem_euclid(mr as isize) as usize][c.rem_euclid(mc as isize) as usize]
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
        let (mr, mc) = self.dimensions_with_rot(cw_times);
        match cw_times {
            0 => (r, c),
            1 => (mc - c - 1, r),
            2 => (mr - r - 1, mc - c - 1),
            3 => (c, mr - r - 1),
            _ => unreachable!(),
        }
    }
    pub fn get_with_rot(&self, r: usize, c: usize, cw_times: usize) -> &T {
        let (r, c) = self.coord_with_rot(r, c, cw_times);
        &self.0[r][c]
    }

    pub fn iter(&self) -> GridIter<T> {
        GridIter {
            g: self,
            r: 0,
            c: 0,
        }
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

    pub fn into<U>(&self) -> Grid<U>
    where T: Into<U> + Copy {
        Grid(self.0.iter().map(|r| r.iter().map(|c| U::from((*c).into())).collect()).collect())
    }

    pub fn into_f<F, U>(&self, f: F) -> Grid<U>
    where F: Fn(T) -> U, T: Copy {
        Grid(self.0.iter().map(|r| r.iter().map(|c| f(*c)).collect()).collect())
    }
}

impl<T> std::str::FromStr for Grid<T>
where
    T: From<char> + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grid(
            s.lines()
                .map(|l| l.chars().map(|c| c.to_string().parse::<T>().unwrap()).collect())
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
        } else if self.i >= cc {
            None
        } else {
            self.i += 1;
            Some(self.g.col(self.i - 1))
        }
    }
}
pub struct GridIter<'a, T> {
    g: &'a Grid<T>,
    r: usize,
    c: usize,
}
impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let (rr, cc) = self.g.dimensions();
        let (prev_r, prev_c) = (self.r, self.c);
        if prev_r >= rr {
            None
        } else {
            self.c += 1;
            if self.c >= cc {
                self.c = 0;
                self.r += 1;
            }
            Some(((prev_r, prev_c), self.g.get(prev_r, prev_c)))
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
        } else if self.r >= rr {
            None
        } else {
            self.r += 1;
            Some(&self.g.0[self.r - 1][self.c])
        }
    }
}
