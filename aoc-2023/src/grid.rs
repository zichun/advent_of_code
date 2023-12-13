pub struct Grid<T>(pub Vec<Vec<T>>);

#[allow(dead_code)]
impl<T> Grid<T> {
    pub fn dimensions(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
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
where T: From<char>,
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
