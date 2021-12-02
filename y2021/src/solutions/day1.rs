use itertools::Itertools;
use rust_util::{AocDay, Day};
use std::{error::Error, fmt::Display};

pub struct Solve {
  input: String,
}

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    let input = rust_util::read_input(2021, d)?;
    Ok(Box::new(Solve {input}))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(self.input.lines()
      .filter_map(|n| n.parse::<usize>().ok())
      .tuple_windows()
      .filter(|(a, b)| b > a)
      .count()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(self.input.lines()
      .filter_map(|n| n.parse::<usize>().ok())
      .tuple_windows::<(_,_,_)>()
      .map(|t| t.0 + t.1 + t.2)
      .tuple_windows()
      .filter(|(a, b)| b > a)
      .count()))  
  }
} 
