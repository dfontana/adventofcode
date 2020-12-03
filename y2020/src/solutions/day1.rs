use crate::day::{Day, DayArg};
use crate::util::read_input;

use std::error::Error;

pub struct Day1 {
  numbers: Vec<i32>,
}

impl Day for Day1 {
  fn new() -> Result<Day1, Box<dyn Error>> {
    let numbers: Vec<i32> = read_input(DayArg::D(1))?
      .lines()
      .map(|f| f.parse::<i32>().unwrap())
      .collect();
    Ok(Day1 { numbers })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    for (idx, v) in self.numbers.iter().enumerate() {
      for ov in &self.numbers[idx + 1..] {
        let sum = v + ov;
        if sum == 2020 {
          return Ok(format!("{}", v * ov));
        }
      }
    }
    Err("No Solution Found".into())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    for (idx, v) in self.numbers.iter().enumerate() {
      for (odx, ov) in self.numbers[idx + 1..].iter().enumerate() {
        for ev in &self.numbers[odx + 1..] {
          let sum = v + ov + ev;
          if sum == 2020 {
            return Ok(format!("{}", v * ov * ev));
          }
        }
      }
    }
    Err("No Solution Found".into())
  }
}
