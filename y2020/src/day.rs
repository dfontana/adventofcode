use std::fmt;
use std::error::Error;

pub trait Day {
  fn p1(&self) -> Result<String, Box<dyn Error>>;
  fn p2(&self) -> Result<String, Box<dyn Error>>;
  fn run(&self) -> Result<String, Box<dyn Error>> {
    let p1 = self.p1()?;
    let p2 = self.p2()?;
    Ok(format!("Part1: {}\nPart2: {}",p1,p2))
  }
}

pub enum DayArg {
  D(i32)
}

pub enum Part {
  P1,
  P2,
}

impl fmt::Display for Part {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let val = match self {
      Part::P1 => 1,
      Part::P2 => 2,
    };
    write!(f, "{}", val)
  }
}