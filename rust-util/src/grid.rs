use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid<T> {
  board: Vec<Vec<T>>,
  x_max: usize,
  y_max: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Dir {
  N,
  E,
  S,
  W,
  Idle,
}

impl Dir {
  pub fn rev(&self) -> Dir {
    match self {
      Dir::N => Dir::S,
      Dir::E => Dir::W,
      Dir::S => Dir::N,
      Dir::W => Dir::E,
      Dir::Idle => Dir::Idle,
    }
  }
}

impl<T: Display> Grid<T> {
  pub fn print(&self) {
    for y in 0..self.y_max {
      for x in 0..self.x_max {
        print!("{}", self.board[y][x]);
      }
      println!();
    }
  }
}

impl<T> Grid<T> {
  pub fn new(board: Vec<Vec<T>>) -> Self {
    Grid {
      x_max: board[0].len(),
      y_max: board.len(),
      board,
    }
  }

  pub fn bottom_right(&self) -> (usize, usize) {
    (self.y_max - 1, self.x_max - 1)
  }

  pub fn left_side(&self) -> RangeIter {
    RangeIter {
      range: (0..self.y_max),
      idx: 0,
      into_loc: Box::new(|y| (y, 0)),
    }
  }

  pub fn right_side(&self) -> RangeIter {
    let x_max = self.x_max - 1;
    RangeIter {
      range: (0..self.y_max),
      idx: 0,
      into_loc: Box::new(move |y| (y, x_max)),
    }
  }

  pub fn top_side(&self) -> RangeIter {
    RangeIter {
      range: (0..self.x_max),
      idx: 0,
      into_loc: Box::new(|x| (0, x)),
    }
  }

  pub fn bottom_side(&self) -> RangeIter {
    let y_max = self.y_max - 1;
    RangeIter {
      range: (0..self.x_max),
      idx: 0,
      into_loc: Box::new(move |x| (y_max, x)),
    }
  }

  pub fn in_bounds(&self, y: usize, x: usize) -> bool {
    y < self.y_max && x < self.x_max
  }

  pub fn at(&self, y: usize, x: usize) -> Option<&T> {
    match self.in_bounds(y, x) {
      true => Some(&self.board[y][x]),
      false => None,
    }
  }

  pub fn step(&self, y: usize, x: usize, step: usize, dir: Dir) -> Option<(usize, usize)> {
    match dir {
      Dir::W => x.checked_sub(step).map(|x| (y, x)),
      Dir::N => y.checked_sub(step).map(|y| (y, x)),
      Dir::E => Some(x + step).filter(|x| *x < self.x_max).map(|x| (y, x)),
      Dir::S => Some(y + step).filter(|y| *y < self.y_max).map(|y| (y, x)),
      Dir::Idle => Some((y, x)),
    }
  }

  pub fn at_step(&self, y: usize, x: usize, step: usize, dir: Dir) -> Option<((usize, usize), &T)> {
    self
      .step(y, x, step, dir)
      .and_then(|loc| self.at(loc.0, loc.1).map(|t| (loc, t)))
  }

  pub fn put(&mut self, y: usize, x: usize, val: T) {
    self.board[y][x] = val;
  }

  pub fn iter<'a>(&'a self) -> GridIter<'a, T> {
    GridIter {
      grid: self,
      loc: (0, 0),
    }
  }
}

pub struct GridIter<'a, T> {
  grid: &'a Grid<T>,
  loc: (usize, usize),
}
impl<'a, T> Iterator for GridIter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    match self.grid.at(self.loc.0, self.loc.1) {
      Some(t) => {
        self.loc = (self.loc.0, self.loc.1 + 1);
        Some(t)
      }
      None => match self.grid.at(self.loc.0 + 1, 0) {
        Some(t) => {
          self.loc = (self.loc.0 + 1, 1);
          Some(t)
        }
        None => None,
      },
    }
  }
}

pub struct RangeIter {
  range: std::ops::Range<usize>,
  idx: usize,
  into_loc: Box<dyn Fn(usize) -> (usize, usize)>,
}
impl Iterator for RangeIter {
  type Item = (usize, usize);

  fn next(&mut self) -> Option<Self::Item> {
    let idx = self.idx;
    match self.range.contains(&idx) {
      true => {
        self.idx += 1;
        Some((self.into_loc)(idx))
      }
      false => None,
    }
  }
}
