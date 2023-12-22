use rust_util::{grid::Grid, search::dijkstra, Day};
use std::{error::Error, fmt::Display};

pub struct Solve {
  grid: Grid<usize>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      grid: Grid::new_from_map(value, |c| c.to_digit(10).unwrap() as usize),
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
