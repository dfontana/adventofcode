use rust_util::{ Day};
use std::{error::Error, fmt::Display};

pub struct Solve {
  numbers: Vec<i32>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
        let numbers: Vec<i32> =value 
      .lines()
      .map(|f| f.parse::<i32>().unwrap())
      .collect();
    Ok(Solve { numbers })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    for (idx, v) in self.numbers.iter().enumerate() {
      for ov in &self.numbers[idx + 1..] {
        let sum = v + ov;
        if sum == 2020 {
          return Ok(Box::new(format!("{}", v * ov)));
        }
      }
    }
    Err("No Solution Found".into())
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    for (idx, v) in self.numbers.iter().enumerate() {
      for (odx, ov) in self.numbers[idx + 1..].iter().enumerate() {
        for ev in &self.numbers[odx + 1..] {
          let sum = v + ov + ev;
          if sum == 2020 {
            return Ok(Box::new(format!("{}", v * ov * ev)));
          }
        }
      }
    }
    Err("No Solution Found".into())
  }
}
