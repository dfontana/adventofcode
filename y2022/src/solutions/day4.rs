use rust_util::{AocDay, Day};
use std::{error::Error, fmt::Display};

type Range = (usize, usize);
pub struct Solve {
  pairs: Vec<(Range, Range)>,
}

fn into_range(s: &str) -> Range {
  let mut split = s.splitn(2, '-');
  (
    split.next().unwrap().parse().unwrap(),
    split.next().unwrap().parse().unwrap(),
  )
}

fn into_range_pair(s: &str) -> (Range, Range) {
  let mut split = s.splitn(2, ',');
  (
    into_range(split.next().unwrap()),
    into_range(split.next().unwrap()),
  )
}

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    let instr = rust_util::read_input(2022, d)?;
    let pairs: Vec<(Range, Range)> = instr.lines().map(into_range_pair).collect();
    Ok(Box::new(Solve { pairs }))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let fully_contained = self
      .pairs
      .iter()
      .filter(|((b1a, b1b), (b2a, b2b))| (b1a >= b2a && b1b <= b2b) || (b2a >= b1a && b2b <= b1b))
      .count();
    Ok(Box::new(fully_contained))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let partly_contained = self
      .pairs
      .iter()
      .filter(|((a1, a2), (b1, b2))| {
        (a1 <= b1 && a2 >= b1)
          || (a1 <= b2 && a2 >= b2)
          || (b1 <= a1 && b2 >= a1)
          || (b1 <= a2 && b2 >= a2)
      })
      .count();
    Ok(Box::new(partly_contained))
  }
}
