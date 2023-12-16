use itertools::Itertools;
use rust_util::Day;
use std::{collections::HashMap, error::Error, fmt::Display};

#[derive(Debug)]
pub struct Solve {
  grid: Grid,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Tile {
  Round,
  Blank,
  Square,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Grid {
  board: Vec<Vec<Tile>>,
  y_max: usize,
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
      grid: Grid {
        y_max: board.len(),
        board,
      },
    })
  }
}

fn find_cycle(g: &mut Grid, max: isize) -> (isize, isize, HashMap<Grid, isize>) {
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

fn total_load(board: &Vec<Vec<Tile>>, y_max: usize) -> usize {
  board
    .iter()
    .enumerate()
    .flat_map(|(y, v)| {
      v.iter().filter_map(move |t| match t {
        Tile::Round => Some(y),
        _ => None,
      })
    })
    .map(|y| y_max - y)
    .reduce(|acc, y| acc + y)
    .unwrap()
}

impl Grid {
  fn rot90(&mut self) -> &mut Self {
    self.board = (0..self.board[0].len())
      .map(|i| {
        self
          .board
          .iter()
          .map(|inner| inner[i].clone())
          .rev()
          .collect::<Vec<_>>()
      })
      .collect();
    self
  }
}

fn cycle_one(g: &mut Grid) {
  g.rot90();
  g.rot90();
  g.rot90();
  shift(g);
  shift(g.rot90());
  shift(g.rot90());
  shift(g.rot90());
  g.rot90();
  g.rot90();
}

fn shift(g: &mut Grid) {
  for row in g.board.iter_mut() {
    let mut left_most = 0;
    for x in 0..row.len() {
      match row[x] {
        Tile::Round => {
          row[x] = Tile::Blank;
          row[left_most] = Tile::Round;
          left_most += 1;
        }
        Tile::Square => {
          left_most = x + 1;
        }
        Tile::Blank => {}
      }
    }
  }
}
impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut g = self.grid.clone();
    g.rot90();
    g.rot90();
    g.rot90();
    shift(&mut g);
    g.rot90();
    Ok(Box::new(total_load(&g.board, g.y_max)))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let max = 1_000_000_000;
    let (start, period, grids) = find_cycle(&mut self.grid.clone(), max);
    let inverted: HashMap<_, _> = grids.iter().map(|(k, v)| (v, k)).collect();
    let end = inverted.get(&(start + (max - start) % period)).unwrap();
    Ok(Box::new(total_load(&end.board, end.y_max)))
  }
}
