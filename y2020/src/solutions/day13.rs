use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::error::Error;

pub struct Solve {
  target: usize,
  buses: Vec<(usize, usize)>,
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    let inp = read_input(d)?;
    let mut lines = inp.lines();
    Ok(Solve {
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
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    let ans = self
      .buses
      .iter()
      .map(|(_, b)| (*b, ((self.target / b) + 1) * b))
      .min_by(|a, b| a.1.cmp(&b.1))
      .map(|(bus_id, time)| bus_id * (time - self.target))
      .unwrap();
    Ok(ans.to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    let mut n = 1;
    let mut all_equal = false;
    while !all_equal {
      all_equal = true;
      let mut buses = self.buses.iter();

      let test: usize = buses.next().map(|(_, bus)| bus * n).unwrap();

      for (t, bus) in buses {
        if (test + t) % bus != 0 {
          all_equal = false;
          n += 1;
          break;
        }
      }

      if all_equal {
        break;
      }
    }
    Ok(
      self
        .buses
        .iter()
        .next()
        .map(|(_, bus)| bus * n)
        .unwrap()
        .to_string(),
    )
  }
}
