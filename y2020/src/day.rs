use std::error::Error;
use std::fmt;

pub trait Day {
  fn new(d: DayArg) -> Result<Self, Box<dyn Error>>
  where
    Self: Sized;
  fn p1(&self) -> Result<String, Box<dyn Error>>;
  fn p2(&self) -> Result<String, Box<dyn Error>>;
  fn run(&self) -> Result<String, Box<dyn Error>> {
    let p1 = self.p1()?;
    let p2 = self.p2()?;
    Ok(format!("Part1: {}\nPart2: {}", p1, p2))
  }
}

pub enum DayArg {
  D(i32),
}

impl fmt::Display for DayArg {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      DayArg::D(v) => write!(f, "{}", v),
    }
  }
}
