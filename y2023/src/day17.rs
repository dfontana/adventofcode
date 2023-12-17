use rust_util::Day;
use std::{
  cmp::Reverse,
  collections::{BinaryHeap, HashMap},
  error::Error,
  fmt::Display,
};

pub struct Solve {
  grid: Grid<usize>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let board: Vec<Vec<usize>> = value
      .lines()
      .map(|l| {
        l.chars()
          .filter_map(|c| c.to_digit(10))
          .map(|n| n as usize)
          .collect()
      })
      .collect();

    Ok(Solve {
      grid: Grid {
        // target: (board.len() - 1, board[0].len() - 1),
        x_max: board[0].len(),
        y_max: board.len(),
        board,
      },
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(dijkstra(self.grid.clone(), 1, 3)))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(dijkstra(self.grid.clone(), 4, 10)))
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Grid<T> {
  board: Vec<Vec<T>>,
  x_max: usize,
  y_max: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
  N,
  E,
  S,
  W,
  Idle,
}

impl Dir {
  fn rev(&self) -> Dir {
    match self {
      Dir::N => Dir::S,
      Dir::E => Dir::W,
      Dir::S => Dir::N,
      Dir::W => Dir::E,
      Dir::Idle => Dir::Idle,
    }
  }
}

impl<T> Grid<T> {
  fn bottom_right(&self) -> (usize, usize) {
    (self.y_max - 1, self.x_max - 1)
  }

  fn in_bounds(&self, y: usize, x: usize) -> bool {
    y < self.y_max && x < self.x_max
  }

  fn at(&self, y: usize, x: usize) -> Option<&T> {
    match self.in_bounds(y, x) {
      true => Some(&self.board[y][x]),
      false => None,
    }
  }

  fn step(&self, y: usize, x: usize, step: usize, dir: Dir) -> Option<(usize, usize)> {
    match dir {
      Dir::W => x.checked_sub(step).map(|x| (y, x)),
      Dir::N => y.checked_sub(step).map(|y| (y, x)),
      Dir::E => Some(x + step).filter(|x| *x < self.x_max).map(|x| (y, x)),
      Dir::S => Some(y + step).filter(|y| *y < self.y_max).map(|y| (y, x)),
      Dir::Idle => Some((y, x)),
    }
  }

  fn at_step(&self, y: usize, x: usize, step: usize, dir: Dir) -> Option<((usize, usize), &T)> {
    self
      .step(y, x, step, dir)
      .and_then(|loc| self.at(loc.0, loc.1).map(|t| (loc, t)))
  }
}

fn dijkstra(grid: Grid<usize>, minstep: usize, maxstep: usize) -> usize {
  let mut dists = HashMap::new();
  let mut q = BinaryHeap::from_iter([Reverse((0, (0, 0, Dir::Idle)))]);

  while let Some(Reverse((cost, (y, x, d)))) = q.pop() {
    // Goal Checking
    if (y, x) == grid.bottom_right() {
      return cost;
    }

    // Skip this path if we've found a shorter path to here
    if dists.get(&(y, x, d.clone())).is_some_and(|&c| cost > c) {
      continue;
    }

    for dir in [Dir::N, Dir::S, Dir::W, Dir::E] {
      // Skip going in same direction or reverse direction
      if dir == d || dir == d.rev() {
        continue;
      }

      let mut next_cost = cost;
      for dist in 1..=maxstep {
        let Some(((rr, cc), cost)) = grid.at_step(y, x, dist, dir.clone()) else {
          continue;
        };
        next_cost += cost;
        if dist < minstep {
          continue;
        }

        let key = (rr, cc, dir.clone());
        if next_cost < *dists.get(&key).unwrap_or(&usize::MAX) {
          dists.insert(key.clone(), next_cost);
          q.push(Reverse((next_cost, key.clone())));
        }
      }
    }
  }
  unreachable!("Did not find path to target")
}
