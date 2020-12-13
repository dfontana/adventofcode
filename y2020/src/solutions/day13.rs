use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::error::Error;

pub struct Solve {
  target: i32,
  buses: Vec<i32>,
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    let inp = read_input(d)?;
    let mut lines = inp.lines();
    Ok(Solve {
      target: lines.next().map(|i| i.parse::<i32>()).unwrap()?,
      buses: lines
        .next()
        .map(|l| {
          l.split(",")
            .filter(|i| *i != "x")
            .map(|v| v.parse::<i32>().unwrap())
            .collect()
        })
        .unwrap(),
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    let ans = self
      .buses
      .iter()
      .map(|b| (*b, ((self.target / b) + 1) * b))
      .min_by(|a, b| a.1.cmp(&b.1))
      .map(|(bus_id, time)| bus_id * (time - self.target))
      .unwrap();
    Ok(ans.to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok("Impl".to_string())
  }
}
