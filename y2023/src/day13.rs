use itertools::Itertools;
use rust_util::Day;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct Solve {
  input: Vec<String>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      input: value.split("\n\n").map(|s| s.to_string()).collect_vec(),
    })
  }
}

fn count_cols(pat: &String) -> (usize, usize) {
  let patv = pat.lines().map(|v| v.chars().collect_vec()).collect_vec();
  let path = rot90(patv.clone());
  (count_hsplit(path), count_hsplit(patv))
}

fn count_hsplit(patv: Vec<Vec<char>>) -> usize {
  for h in 1..=patv.len() / 2 {
    let pilf = patv[0..h].iter().collect_vec();
    let flip = patv[h..2 * h].iter().rev().collect_vec();
    if pilf == flip {
      return h;
    }
  }
  for h in (patv.len().div_ceil(2)..patv.len()).rev() {
    let pilf = patv[2 * h - patv.len()..h].iter().collect_vec();
    let flip = patv[h..].iter().rev().collect_vec();
    if pilf == flip {
      return h;
    }
  }
  0
}

fn rot90(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
  (0..v[0].len())
    .map(|i| {
      v.iter()
        .map(|inner| inner[i].clone())
        .rev()
        .collect::<Vec<_>>()
    })
    .collect()
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .input
        .iter()
        .map(count_cols)
        .fold(0, |acc, (v, h)| acc + v + h * 100),
    ))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(1))
  }
}
