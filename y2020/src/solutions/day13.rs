use rust_util::{read_input, AocDay, Day};
use std::{error::Error, fmt::Display};

pub struct Solve {
  target: usize,
  buses: Vec<(usize, usize)>,
}

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    let inp = read_input(2020, d)?;
    let mut lines = inp.lines();
    Ok(Box::new(Solve {
      target: lines.next().map(|i| i.parse::<usize>()).unwrap()?,
      buses: lines
        .next()
        .map(|l| {
          l.split(",")
            .enumerate()
            .filter(|(_, i)| *i != "x")
            .map(|(offset, bus)| (offset, bus.parse::<usize>().unwrap()))
            .collect()
        })
        .unwrap(),
    }))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let ans = self
      .buses
      .iter()
      .map(|(_, b)| (*b, ((self.target / b) + 1) * b))
      .min_by(|a, b| a.1.cmp(&b.1))
      .map(|(bus_id, time)| bus_id * (time - self.target))
      .unwrap();
    Ok(Box::new(ans.to_string()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    // Can't claim this one as mine :/ Not a numberphile, so the concept of
    // Chinese Remainder Theorem is new to me
    let (ans, _) = self
      .buses
      .iter()
      .fold((0, 1), |(mut sol, step), (t, bus_id)| {
        // 1. Walk common multiple of all prior bus's
        // 2. Stop at num whom adding the offset results in multiple for this bus
        // 3. Factor this bus into the CM, and repeat for the next until all are considered
        //    Starting with the number we found in 2.
        sol = (sol..)
          .step_by(step)
          .find(|x| (x + t) % bus_id == 0)
          .unwrap();
        (sol, step * bus_id)
      });
    Ok(Box::new(ans.to_string()))
  }
}
