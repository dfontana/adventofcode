use rust_util::Day;
use std::{error::Error, fmt::Display};

type Pnt = (usize, usize);
type Path = Vec<Pnt>;
pub struct Solve {
  start: Pnt,
  paths: Vec<Path>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      start: (500, 0),
      paths: value
        .lines()
        .map(|l| {
          l.split(" -> ")
            .filter_map(|pt| pt.split_once(','))
            .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
            .collect()
        })
        .collect(),
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    // Recall: 0,0 is top left.
      println!("{:?}", self.paths);
    Ok(Box::new("y"))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new("y"))
  }
}
