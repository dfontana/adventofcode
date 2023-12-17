use rust_util::Day;
use std::{collections::HashSet, error::Error, fmt::Display};

pub struct Solve {
  x_max: usize,
  y_max: usize,
  board: Vec<Vec<TileState>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tile {
  SplitV,
  SplitH,
  MirrorL,
  MirrorR,
  Blank,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Dir {
  L,
  R,
  U,
  D,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum TileState {
  Energized(Tile),
  Dormant(Tile),
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let board: Vec<Vec<_>> = value
      .lines()
      .map(|line| {
        line
          .chars()
          .map(|c| match c {
            '.' => Tile::Blank,
            '-' => Tile::SplitH,
            '|' => Tile::SplitV,
            '\\' => Tile::MirrorR,
            '/' => Tile::MirrorL,
            _ => unreachable!(),
          })
          .map(|t| TileState::Dormant(t))
          .collect()
      })
      .collect();
    Ok(Solve {
      x_max: board[0].len(),
      y_max: board.len(),
      board,
    })
  }
}

impl Dir {
  fn next(
    &self,
    (x, y): (usize, usize),
    x_max: usize,
    y_max: usize,
  ) -> Option<((usize, usize), Dir)> {
    let loc = match self {
      Dir::L => x.checked_sub(1).map(|x| (x, y)),
      Dir::U => y.checked_sub(1).map(|y| (x, y)),
      Dir::R => Some(x + 1).filter(|x| *x < x_max).map(|x| (x, y)),
      Dir::D => Some(y + 1).filter(|y| *y < y_max).map(|y| (x, y)),
    };
    loc.map(|l| (l, self.clone()))
  }
}

impl Tile {
  fn refract(&self, from_dir: Dir) -> Vec<Dir> {
    match (self, &from_dir) {
      (Tile::Blank, _) => vec![from_dir],
      (Tile::SplitV, Dir::D | Dir::U) | (Tile::SplitH, Dir::L | Dir::R) => vec![from_dir],
      (Tile::SplitV, Dir::L | Dir::R) => vec![Dir::D, Dir::U],
      (Tile::SplitH, Dir::D | Dir::U) => vec![Dir::L, Dir::R],
      (Tile::MirrorL, Dir::R) | (Tile::MirrorR, Dir::L) => vec![Dir::U],
      (Tile::MirrorL, Dir::L) | (Tile::MirrorR, Dir::R) => vec![Dir::D],
      (Tile::MirrorL, Dir::D) | (Tile::MirrorR, Dir::U) => vec![Dir::L],
      (Tile::MirrorL, Dir::U) | (Tile::MirrorR, Dir::D) => vec![Dir::R],
    }
  }
}

impl Solve {
  fn energize(&self, init: ((usize, usize), Dir)) -> Vec<Vec<TileState>> {
    let mut board = self.board.clone();

    let mut seen = HashSet::new();
    let mut edges = vec![init];
    while let Some(edge) = edges.pop() {
      if seen.contains(&edge) {
        continue;
      }
      seen.insert(edge.clone());

      let t = match board[edge.0 .1][edge.0 .0].clone() {
        TileState::Energized(t) => t,
        TileState::Dormant(t) => t,
      };
      board[edge.0 .1][edge.0 .0] = TileState::Energized(t.clone());
      t.refract(edge.1)
        .iter()
        .filter_map(|d| d.next(edge.0, self.x_max, self.y_max))
        .for_each(|edge| {
          edges.push(edge.clone());
        });
    }

    board
  }

  fn boarder(&self) -> Vec<((usize, usize), Dir)> {
    let mut v = Vec::new();
    for x in 0..self.x_max {
      v.push(((x, 0), Dir::D));
      v.push(((x, self.y_max - 1), Dir::U));
    }
    for y in 0..self.y_max {
      v.push(((0, y), Dir::R));
      v.push(((self.x_max - 1, y), Dir::L));
    }
    v
  }
}

fn count_energized(b: &Vec<Vec<TileState>>) -> usize {
  b.iter()
    .flat_map(|v| v)
    .filter(|t| match t {
      TileState::Energized(_) => true,
      _ => false,
    })
    .count()
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(count_energized(&self.energize(((0, 0), Dir::R)))))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(format!(
      "{:?}",
      self
        .boarder()
        .iter()
        .map(|e| self.energize(e.clone()))
        .map(|b| count_energized(&b))
        .max(),
    )))
  }
}
