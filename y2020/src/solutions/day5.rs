use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::error::Error;

pub struct Day5 {}

impl Day for Day5 {
  fn new() -> Result<Day5, Box<dyn Error>> {
    let inp = read_input(DayArg::D(5))?;
    Ok(Day5 {})
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    Ok("Not impl".to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok("Not impl".to_string())
  }
}
