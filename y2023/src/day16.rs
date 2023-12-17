use rust_util::Day;
use std::{
  collections::{HashSet, VecDeque},
  error::Error,
  fmt::Display,
};

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
      Dir::R => Some(x + 1).filter(|x| *x < x_max).map(|x| (x, y)),
      Dir::L => x.checked_sub(1).map(|x| (x, y)),
      Dir::U => y.checked_sub(1).map(|y| (x, y)),
      Dir::D => Some(y + 1).filter(|y| *y < y_max).map(|y| (x, y)),
    };
    loc.map(|l| (l, self.clone()))
  }
}

impl Tile {
  fn refract(
    &self,
    from: (usize, usize),
    from_dir: Dir,
    x_max: usize,
    y_max: usize,
  ) -> Vec<((usize, usize), Dir)> {
    let dirs = match (self, &from_dir) {
      (Tile::Blank, _) | (Tile::SplitV, Dir::D | Dir::U) | (Tile::SplitH, Dir::L | Dir::R) => {
        vec![from_dir]
      }
      (Tile::SplitV, Dir::L | Dir::R) => vec![Dir::D, Dir::U],
      (Tile::SplitH, Dir::D | Dir::U) => vec![Dir::L, Dir::R],
      (Tile::MirrorL, Dir::R) | (Tile::MirrorR, Dir::L) => vec![Dir::U],
      (Tile::MirrorL, Dir::L) | (Tile::MirrorR, Dir::R) => vec![Dir::D],
      (Tile::MirrorL, Dir::D) | (Tile::MirrorR, Dir::U) => vec![Dir::L],
      (Tile::MirrorL, Dir::U) | (Tile::MirrorR, Dir::D) => vec![Dir::R],
    };
    dirs
      .iter()
      .filter_map(|d| d.next(from, x_max, y_max))
      .collect()
  }
}

impl Solve {
  fn energize(&self) -> Vec<Vec<TileState>> {
    let mut board = self.board.clone();

    let mut seen = HashSet::new();
    let mut edges = vec![((0, 0), Dir::R)];
    while let Some(edge) = edges.pop() {
      if seen.contains(&edge) {
        continue;
      }
      seen.insert(edge.clone());

      energize(&mut board, edge.0);

      let t = match board[edge.0 .1][edge.0 .0].clone() {
        TileState::Energized(t) => t,
        TileState::Dormant(t) => t,
      };
      t.refract(edge.0, edge.1, self.x_max, self.y_max)
        .iter()
        .for_each(|edge| {
          edges.push(edge.clone());
        });
    }
    board
  }
}

fn energize(board: &mut Vec<Vec<TileState>>, (x, y): (usize, usize)) {
  let t = match &board[y][x] {
    TileState::Energized(t) => t,
    TileState::Dormant(t) => t,
  };
  board[y][x] = TileState::Energized(t.clone());
}

// fn print(board: &Vec<Vec<TileState>>) {
//   for b in board.iter() {
//     for t in b.iter() {
//       match t {
//         TileState::Energized(_) => print!("#"),
//         TileState::Dormant(t) => match t {
//           Tile::SplitV => print!("|"),
//           Tile::SplitH => print!("-"),
//           Tile::MirrorL => print!("/"),
//           Tile::MirrorR => print!("\\"),
//           Tile::Blank => print!("."),
//         },
//       }
//     }
//     println!();
//   }
// }

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
    Ok(Box::new(count_energized(&self.energize())))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(1))
  }
}
