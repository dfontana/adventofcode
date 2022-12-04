use itertools::Itertools;
use rust_util::Day;
use std::{error::Error, fmt::Display};

pub struct Solve {
  input: Vec<usize>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let mut input: Vec<usize> = Vec::new();
    let mut acc: usize = 0;
    for line in value.lines() {
      if line.is_empty() {
        input.push(acc);
        acc = 0;
      } else {
        acc += line.parse::<usize>().expect("Failed to parse line");
      }
    }
    input.push(acc);

    Ok(Solve { input })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self.input.iter().sorted().rev().take(1).sum::<usize>(),
    ))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self.input.iter().sorted().rev().take(3).sum::<usize>(),
    ))
  }
}
