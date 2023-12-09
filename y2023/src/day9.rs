use itertools::Itertools;
use rust_util::Day;
use std::{error::Error, fmt::Display};

pub struct Solve {
  firsts: Vec<Vec<i64>>,
  lasts: Vec<Vec<i64>>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let (firsts, lasts) = value
      .lines()
      .map(|s| {
        s.split_whitespace()
          .map(|s| s.parse::<i64>().unwrap())
          .collect_vec()
      })
      .map(Solve::ends_diffs)
      .fold((Vec::new(), Vec::new()), |(mut afs, mut als), (fs, ls)| {
        afs.push(fs);
        als.push(ls);
        (afs, als)
      });
    Ok(Solve { firsts, lasts })
  }
}

impl Solve {
  fn ends_diffs(reading: Vec<i64>) -> (Vec<i64>, Vec<i64>) {
    let mut diffs: Vec<Vec<i64>> = vec![reading.clone()];
    let (mut firsts, mut lasts) = (Vec::new(), Vec::new());

    while let Some(next) = diffs.pop() {
      lasts.push(*next.last().unwrap());
      firsts.push(*next.first().unwrap());
      let (is_zero, diff) = next.iter().tuple_windows().map(|(a, b)| b - a).fold(
        (true, Vec::new()),
        |(is_zero, mut diff), d| {
          diff.push(d);
          (is_zero && d == 0, diff)
        },
      );
      if !is_zero {
        diffs.push(diff);
      }
    }

    (firsts, lasts)
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(format!(
      "{:?}",
      self
        .lasts
        .iter()
        .map(|r| r.iter().rev().fold(0, |delta, b| delta + b))
        .reduce(|a, b| a + b),
    )))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(format!(
      "{:?}",
      self
        .firsts
        .iter()
        .map(|r| r.iter().rev().fold(0, |delta, b| b - delta))
        .reduce(|a, b| a + b),
    )))
  }
}
