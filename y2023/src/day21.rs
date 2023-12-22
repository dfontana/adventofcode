use rust_util::{
  grid::{Dir, Grid},
  Day,
};
use std::{
  collections::{HashSet, VecDeque},
  error::Error,
  fmt::Display,
};

#[derive(Debug)]
pub struct Solve {
  grid: Grid<Tile>,
  start: (usize, usize),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Tile {
  Rock,
  Plot,
}

impl Display for Tile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Tile::Rock => '#',
        Tile::Plot => '.',
      }
    )
  }
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let mut start = (0, 0);
    let board = value
      .lines()
      .enumerate()
      .map(|(y, l)| {
        l.chars()
          .enumerate()
          .filter_map(|(x, c)| match c {
            '.' => Some(Tile::Plot),
            '#' => Some(Tile::Rock),
            'S' => {
              start = (y, x);
              Some(Tile::Plot)
            }
            _ => unreachable!(),
          })
          .collect()
      })
      .collect();
    Ok(Solve {
      start,
      grid: Grid::new(board),
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let max_steps = 64;
    let mut dest: HashSet<(usize, usize)> = HashSet::new();
    let mut seen: HashSet<((usize, usize), usize)> = HashSet::new();

    let mut frontier = VecDeque::from_iter(vec![(self.start.clone(), 0)]);
    while let Some((loc, steps)) = frontier.pop_front() {
      if seen.contains(&(loc, steps)) {
        continue;
      }
      seen.insert((loc, steps));
      [Dir::N, Dir::E, Dir::S, Dir::W]
        .iter()
        .filter_map(|d| self.grid.at_step(loc.0, loc.1, 1, *d))
        .filter(|(_, t)| **t != Tile::Rock)
        .for_each(|(loc, _)| {
          if steps + 1 == max_steps {
            dest.insert(loc.clone());
          } else {
            frontier.push_back((loc.clone(), steps + 1));
          }
        });
    }

    Ok(Box::new(dest.len()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(1))
  }
}
