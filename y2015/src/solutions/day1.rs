use crate::day::{Day, DayArg};
use crate::util::read_input;

use std::error::Error;

pub struct Solve {
  input: String,
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    Ok(Solve {
      input: read_input(d)?,
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    let sum = self
      .input
      .chars()
      .map(|c| match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
      })
      .sum::<i32>();
    Ok(sum.to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    let (idx, _) = self
      .input
      .chars()
      .enumerate()
      .map(|(idx, c)| match c {
        '(' => (idx, 1),
        ')' => (idx, -1),
        _ => (idx, 0),
      })
      .fold((None, 0), |(idx, lvl), (i, dist)| {
        if idx.is_some() {
          return (idx, lvl);
        }
        let level = lvl + dist;
        if level == -1 {
          return (Some(i + 1), level);
        }
        (None, level)
      });
    idx.map(|i| i.to_string()).ok_or("No Solve Found".into())
  }
}
