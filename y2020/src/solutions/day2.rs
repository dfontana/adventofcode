use crate::day::{Day, DayArg};
use crate::util::read_input;

use std::error::Error;

pub struct Day2 {
}

impl Day2 {
  pub fn new() -> Result<Day2, Box<dyn Error>> {
    let input = read_input(DayArg::D(2))?;
    Ok(Day2{})
  }
}

impl Day for Day2 {
  fn p1(&self) -> Result<String, Box<dyn Error>> {
    todo!()
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    todo!()
  }
}