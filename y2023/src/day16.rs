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
  fn next(&self, (x, y): (usize, usize), x_max: usize, y_max: usize) -> Option<(usize, usize)> {
    // TODO needs OOB checking
    match self {
      Dir::L => (x + 1, y),
      Dir::R => (x - 1, y),
      Dir::U => (x, y - 1),
      Dir::D => (x, y + 1),
    };
    todo!()
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
    // TODO surly we can reduce the branching going on here, but maybe get a working thing first
    match self {
      Tile::SplitV => todo!(),
      Tile::SplitH => match from_dir {
        Dir::L | Dir::R => {
          if let Some(e) = from_dir.next(from, x_max, y_max) {
            vec![(e, from_dir)]
          } else {
            vec![]
          }
        }
        Dir::U => todo!(),
        Dir::D => {
          let mut es = Vec::new();
          if let Some(e) = Dir::L.next(from, x_max, y_max) {
            es.push((e, Dir::L));
          }
          if let Some(e) = Dir::R.next(from, x_max, y_max) {
            es.push((e, Dir::R));
          }
          es
        }
      },
      Tile::MirrorL => match from_dir {
        // => /
        Dir::L => {
          if let Some(e) = Dir::U.next(from, x_max, y_max) {
            vec![(e, Dir::U)]
          } else {
            vec![]
          }
        }
        Dir::R => {
          if let Some(e) = Dir::D.next(from, x_max, y_max) {
            vec![(e, Dir::D)]
          } else {
            vec![]
          }
        }
        Dir::U => {
          if let Some(e) = Dir::L.next(from, x_max, y_max) {
            vec![(e, Dir::L)]
          } else {
            vec![]
          }
        }
        Dir::D => {
          if let Some(e) = Dir::R.next(from, x_max, y_max) {
            vec![(e, Dir::R)]
          } else {
            vec![]
          }
        }
      },
      Tile::MirrorR => match from_dir {
        // => \
        Dir::L => {
          if let Some(e) = Dir::D.next(from, x_max, y_max) {
            vec![(e, Dir::D)]
          } else {
            vec![]
          }
        }
        Dir::R => {
          if let Some(e) = Dir::U.next(from, x_max, y_max) {
            vec![(e, Dir::U)]
          } else {
            vec![]
          }
        }
        Dir::U => {
          if let Some(e) = Dir::R.next(from, x_max, y_max) {
            vec![(e, Dir::R)]
          } else {
            vec![]
          }
        }
        Dir::D => {
          if let Some(e) = Dir::L.next(from, x_max, y_max) {
            vec![(e, Dir::L)]
          } else {
            vec![]
          }
        }
      },
      _ => vec![],
    }
  }
}

impl Solve {
  fn energize(&self) -> Vec<Vec<TileState>> {
    // Beam starts in top left and refracts until all ends out of bounds
    // There will be cycles to detect. I don't think we can just check for
    // energized state as we don't know the direction it was energized from.
    // Perhaps energized state should track direction(s) beam came from &
    // that would be enough to detect if we've covered it.
    //
    // Example: Two beams hitting a tile that energizes with diff results.
    //
    //   ---> \ <----

    let mut board = self.board.clone();
    let mut seen: HashSet<((usize, usize), Dir)> = HashSet::new();
    let mut edges: VecDeque<((usize, usize), Dir)> = VecDeque::new();
    let edge = ((0, 0), Dir::R);
    edges.push_front(edge.clone());
    seen.insert(edge.clone());
    energize(&mut board, (0, 0));

    while let Some(edge) = edges.pop_front() {
      let d = edge.1.clone();

      // Travel one step in direction from loc
      let next_loc: (usize, usize) = match d.next(edge.0, self.x_max, self.y_max) {
        Some(loc) => loc,
        None => continue,
      };
      let next = board[next_loc.1][next_loc.0].clone();
      let next_t = match &next {
        TileState::Energized(t) => t,
        TileState::Dormant(t) => t,
      };

      // Cycle check
      if let TileState::Energized(_) = next {
        if seen.contains(&edge) {
          continue;
        } else {
          seen.insert(edge.clone());
        }
      }

      // Ensure next is energized
      energize(&mut board, next_loc);

      // Take action on this step
      match next_t {
        Tile::Blank => {
          edges.push_back((next_loc, d.clone()));
        }
        _ => {
          next_t
            .refract(next_loc, d.clone(), self.x_max, self.y_max)
            .iter()
            .for_each(|edge| {
              energize(&mut board, edge.0);
              edges.push_back(edge.clone());
            });
        }
      };
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
