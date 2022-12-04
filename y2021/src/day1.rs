use itertools::Itertools;
use rust_util::Day;
use std::{error::Error, fmt::Display};

pub struct Solve {
  input: String,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve { input: value })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .input
        .lines()
        .filter_map(|n| n.parse::<usize>().ok())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count(),
    ))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .input
        .lines()
        .filter_map(|n| n.parse::<usize>().ok())
        .tuple_windows::<(_, _, _)>()
        .map(|t| t.0 + t.1 + t.2)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count(),
    ))
  }
}
