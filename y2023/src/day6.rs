use itertools::Itertools;
use rust_util::Day;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct Solve {
  races: Vec<Race>,
}

#[derive(Debug)]
struct Race {
  time: usize,
  distance: usize,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      races: value
        .split_once("\n")
        .map(|(t, d)| {
          t.strip_prefix("Time:")
            .map(|s| s.split_whitespace())
            .unwrap()
            .map(|n| n.parse::<usize>().unwrap())
            .zip(
              d.strip_prefix("Distance:")
                .map(|s| s.split_whitespace())
                .unwrap()
                .map(|n| n.parse::<usize>().unwrap()),
            )
        })
        .unwrap()
        .map(|(time, distance): (usize, usize)| Race { time, distance })
        .collect_vec(),
    })
  }
}

impl Race {
  fn solve(&self) -> Vec<usize> {
    (1..self.time)
      .map(|t| t * (self.time - t))
      .filter(|d| *d > self.distance)
      .collect_vec()
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(format!(
      "{:?}",
      self
        .races
        .iter()
        .map(Race::solve)
        .map(|v| v.len())
        .reduce(|a, b| a * b),
    )))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new("t"))
  }
}
