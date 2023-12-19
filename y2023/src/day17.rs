use rust_util::{grid::Grid, search::dijkstra, Day};
use std::{error::Error, fmt::Display};

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
      grid: Grid::new(board),
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
