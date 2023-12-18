use itertools::Itertools;
use rust_util::{grid::Grid, Day};
use std::{collections::HashMap, error::Error, fmt::Display};

#[derive(Debug)]
pub struct Solve {
  grid: Grid<Tile>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Tile {
  Round,
  Blank,
  Square,
}

impl Display for Tile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Tile::Round => 'O',
        Tile::Blank => '.',
        Tile::Square => '#',
      }
    )
  }
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let board = value
      .lines()
      .map(|l| {
        l.chars()
          .map(|c| match c {
            'O' => Tile::Round,
            '.' => Tile::Blank,
            '#' => Tile::Square,
            _ => unreachable!(),
          })
          .collect_vec()
      })
      .collect_vec();
    Ok(Solve {
      grid: Grid::new(board),
    })
  }
}

fn find_cycle(g: &mut Grid<Tile>, max: isize) -> (isize, isize, HashMap<Grid<Tile>, isize>) {
  let mut seen = HashMap::new();
  seen.insert(g.clone(), 0);
  let mut i = 1;
  loop {
    cycle_one(g);
    if seen.contains_key(&g) || i == max {
      return (*seen.get(&g).unwrap(), i - seen.get(&g).unwrap(), seen);
    }
    seen.insert(g.clone(), i);
    i += 1;
  }
}

fn total_load(grid: &Grid<Tile>) -> usize {
  grid
    .iter()
    .filter_map(|(y, _, t)| match t {
      Tile::Round => Some(y),
      _ => None,
    })
    .map(|y| grid.height() - y)
    .reduce(|acc, y| acc + y)
    .unwrap()
}

fn cycle_one(g: &mut Grid<Tile>) {
  g.rot90().rot90().rot90();
  shift(g);
  shift(g.rot90());
  shift(g.rot90());
  shift(g.rot90());
  g.rot90().rot90();
}

fn shift(g: &mut Grid<Tile>) {
  for (y, _) in g.left_side() {
    let mut left_most = 0;
    for (_, x) in g.top_side() {
      match g.at(y, x) {
        Some(Tile::Round) => {
          g.put(y, x, Tile::Blank);
          g.put(y, left_most, Tile::Round);
          left_most += 1;
        }
        Some(Tile::Square) => {
          left_most = x + 1;
        }
        _ => {}
      }
    }
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut g = self.grid.clone();
    g.rot90().rot90().rot90();
    shift(&mut g);
    g.rot90();
    Ok(Box::new(total_load(&g)))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let max = 1_000_000_000;
    let (start, period, grids) = find_cycle(&mut self.grid.clone(), max);
    let inverted: HashMap<_, _> = grids.iter().map(|(k, v)| (v, k)).collect();
    let end = inverted.get(&(start + (max - start) % period)).unwrap();
    Ok(Box::new(total_load(&end)))
  }
}
