use crate::day::{Day, DayArg};
use crate::util::read_input;

pub struct Day4 {
}

impl Day for Day4 {
  fn new() -> Result<Day4, Box<dyn Error>> {
    let input = read_input(DayArg::D(4))?;
    Ok(Day4 {})
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    Err("Not implemented".into())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Err("Not implemented".into())
  }
}