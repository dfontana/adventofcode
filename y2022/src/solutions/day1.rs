use itertools::Itertools;
use rust_util::{AocDay, Day};
use std::{error::Error, fmt::Display};

pub struct Solve {
  input: Vec<usize>,
}

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    let instr = rust_util::read_input(2022, d)?;
    let mut input: Vec<usize> = Vec::new();
    let mut acc: usize = 0;
    for line in instr.lines() {
      if line.is_empty() {
        input.push(acc);
        acc = 0;
      } else {
        acc += line.parse::<usize>().expect("Failed to parse line");
      }
    }
    input.push(acc);

    Ok(Box::new(Solve { input }))
  }

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
