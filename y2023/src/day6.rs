use itertools::Itertools;
use rust_util::Day;
use std::{error::Error, fmt::Display};

use crate::tokens::Parser;

#[derive(Debug)]
pub struct Solve {
  races: Vec<Race>,
  big_race: Race,
}

#[derive(Debug)]
struct Race {
  time: usize,
  distance: usize,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let (times, dists) = Parser::lazy()
      .consume("Time:")
      .take_usizes()
      .consume_whitespace()
      .consume("Distance:")
      .take_usizes()
      .apply::<(Vec<usize>, Vec<usize>)>(&value);

    println!("{:?}", times);
    println!("{:?}", dists);
    Ok(Solve {
      races: times
        .iter()
        .zip(dists.iter())
        .map(|(t, d)| Race { time: *t, distance: *d })
        .collect_vec(),
      big_race: Race {
        time: times
          .iter()
          .map(|t| t.to_string())
          .collect::<String>()
          .parse::<usize>()
          .unwrap(),
        distance: dists
          .iter()
          .map(|t| t.to_string())
          .collect::<String>()
          .parse::<usize>()
          .unwrap(),
      },
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
    Ok(Box::new(format!("{:?}", self.big_race.solve().len())))
  }
}
