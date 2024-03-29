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
    let (x, y) = self
      .input
      .lines()
      .map(|l| l.splitn(2, " ").collect_tuple().unwrap())
      .map(|(dir, amt)| (dir, amt.parse::<i32>().unwrap()))
      .fold((0, 0), |(x, y), (dir, amt)| match dir {
        "forward" => (x + amt, y),
        "down" => (x, y + amt),
        "up" => (x, y - amt),
        _ => unreachable!("Invalid direction"),
      });

    Ok(Box::new(x * y))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let (_, x, y) = self
      .input
      .lines()
      .map(|l| l.splitn(2, " ").collect_tuple().unwrap())
      .map(|(dir, amt)| (dir, amt.parse::<i32>().unwrap()))
      .fold((0, 0, 0), |(aim, x, y), (dir, amt)| match dir {
        "forward" => (aim, x + amt, y + (aim * amt)),
        "down" => (aim + amt, x, y),
        "up" => (aim - amt, x, y),
        _ => unreachable!("Invalid direction"),
      });

    Ok(Box::new(x * y))
  }
}
