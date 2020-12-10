use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::error::Error;

pub struct Solve {
  data: Vec<i32>,
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    Ok(Solve {data: read_input(d)?.lines().map(|s| s.parse::<i32>()).flatten().collect()})
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    let num = self.data
      .windows(26)
      .map(|win| (&win[..win.len()-1], win[win.len()-1]))
      .find(has_no_pair)
      .ok_or("No invalid nums found")?;
    Ok(num.1.to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok("Impl".to_string())
  }
}

fn has_no_pair(pair: &(&[i32], i32)) -> bool {
  for (i, v1) in pair.0.iter().enumerate() {
    for v2 in pair.0[i+1..].iter() {
      if v1 + v2 == pair.1 {
        return false;
      }
    }
  }
  true
}