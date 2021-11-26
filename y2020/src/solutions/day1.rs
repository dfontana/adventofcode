use rust_util::{read_input, AocDay, Day};
use std::{error::Error, fmt::Display};

pub struct Solve {
  numbers: Vec<i32>,
}

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    let numbers: Vec<i32> = read_input(2020, d)?
      .lines()
      .map(|f| f.parse::<i32>().unwrap())
      .collect();
    Ok(Box::new(Solve { numbers }))
  }

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
