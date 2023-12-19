use rust_util::{
  grid::{Dir, Grid},
  Day,
};
use std::{collections::HashSet, error::Error, fmt::Display};

pub struct Solve {
  grid: Grid<TileState>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tile {
  SplitV,
  SplitH,
  MirrorL,
  MirrorR,
  Blank,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum TileState {
  Energized(Tile),
  Dormant(Tile),
}

impl Display for Tile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Tile::SplitV => "|",
        Tile::SplitH => "-",
        Tile::MirrorL => "/",
        Tile::MirrorR => "\\",
        Tile::Blank => ".",
      }
    )
  }
}
impl Display for TileState {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      TileState::Energized(_) => write!(f, "#"),
      TileState::Dormant(t) => write!(f, "{}", t),
    }
  }
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
      grid: Grid::new(board),
    })
  }
}

impl Tile {
  fn refract(&self, from_dir: Dir) -> Vec<Dir> {
    match (self, &from_dir) {
      (Tile::Blank, _) => vec![from_dir],
      (Tile::SplitV, Dir::S | Dir::N) | (Tile::SplitH, Dir::W | Dir::E) => vec![from_dir],
      (Tile::SplitV, Dir::W | Dir::E) => vec![Dir::S, Dir::N],
      (Tile::SplitH, Dir::S | Dir::N) => vec![Dir::W, Dir::E],
      (Tile::MirrorL, Dir::E) | (Tile::MirrorR, Dir::W) => vec![Dir::N],
      (Tile::MirrorL, Dir::W) | (Tile::MirrorR, Dir::E) => vec![Dir::S],
      (Tile::MirrorL, Dir::S) | (Tile::MirrorR, Dir::N) => vec![Dir::W],
      (Tile::MirrorL, Dir::N) | (Tile::MirrorR, Dir::S) => vec![Dir::E],
      _ => vec![],
    }
  }
}

impl Solve {
  fn energize(&self, init: ((usize, usize), Dir)) -> Grid<TileState> {
    let mut grid = self.grid.clone();

    let mut seen = HashSet::new();
    let mut edges = vec![init];
    while let Some(edge) = edges.pop() {
      if seen.contains(&edge) {
        continue;
      }
      seen.insert(edge.clone());

      let t = match grid.at(edge.0 .0, edge.0 .1) {
        Some(TileState::Energized(t)) => t.clone(),
        Some(TileState::Dormant(t)) => t.clone(),
        None => continue,
      };
      grid.put(edge.0 .0, edge.0 .1, TileState::Energized(t.clone()));
      t.refract(edge.1)
        .iter()
        .filter_map(|d| grid.step(edge.0 .0, edge.0 .1, 1, *d).map(|loc| (loc, *d)))
        .for_each(|edge| {
          edges.push(edge.clone());
        });
    }
    grid
  }

  fn boarder(&self) -> Vec<((usize, usize), Dir)> {
    self
      .grid
      .left_side()
      .map(|loc| (loc, Dir::E))
      .chain(self.grid.right_side().map(|loc| (loc, Dir::W)))
      .chain(self.grid.bottom_side().map(|loc| (loc, Dir::N)))
      .chain(self.grid.top_side().map(|loc| (loc, Dir::S)))
      .collect()
  }
}

fn count_energized(b: &Grid<TileState>) -> usize {
  b.iter()
    .filter(|(_, _, t)| match t {
      TileState::Energized(_) => true,
      _ => false,
    })
    .count()
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(count_energized(&self.energize(((0, 0), Dir::E)))))
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
