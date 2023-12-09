use itertools::Itertools;
use rust_util::Day;
use std::{error::Error, fmt::Display};

pub struct Solve {
  readings: Vec<Vec<i64>>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      readings: value
        .lines()
        .map(|s| {
          s.split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect()
        })
        .collect(),
    })
  }
}

impl Solve {
  fn predict_next(reading: &Vec<i64>) -> i64 {
    let mut diffs: Vec<Vec<i64>> = vec![reading.clone()];
    let mut finals: Vec<i64> = Vec::new();

    // Reduce the diffs
    let mut diff_0 = false;
    while !diff_0 {
      let mut di = true;
      let next = diffs.pop().unwrap();
      let d = next
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .map(|d| {
          if d != 0 {
            di = false;
          }
          d
        })
        .collect();
      finals.push(*next.last().unwrap());
      diff_0 = di;
      diffs.push(d);
    }
    finals.iter().rev().fold(0, |delta, b| delta + b)
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(format!(
      "{:?}",
      self
        .readings
        .iter()
        .map(Solve::predict_next)
        .reduce(|a, b| a + b),
    )))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(1))
  }
}
