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

fn count_cols(pat: String, p1_ans: &(usize, usize)) -> (usize, usize) {
  let path = pat.lines().map(|v| v.chars().collect_vec()).collect_vec();
  let patv = rot90(path.clone());

  count_hsplit(path, p1_ans.1)
    .map(|v| (0, v))
    .or_else(|| count_hsplit(patv, p1_ans.0).map(|v| (v, 0)))
    .unwrap_or((0, 0))
}

fn count_hsplit(patv: Vec<Vec<char>>, not_this: usize) -> Option<usize> {
  for h in 1..=patv.len() / 2 {
    let pilf = patv[0..h].iter().collect_vec();
    let flip = patv[h..2 * h].iter().rev().collect_vec();
    if pilf == flip && h != not_this {
      return Some(h);
    }
  }
  for h in (patv.len().div_ceil(2)..patv.len()).rev() {
    let pilf = patv[2 * h - patv.len()..h].iter().collect_vec();
    let flip = patv[h..].iter().rev().collect_vec();
    if pilf == flip && h != not_this {
      return Some(h);
    }
  }
  None
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

fn possible_smudges(s: &String) -> Vec<String> {
  let mut out = Vec::new();
  let sv = s.chars().collect_vec();
  for (i, c) in sv.iter().enumerate() {
    let flipped = if *c == '#' { '.' } else { '#' };
    if *c == '#' || *c == '.' {
      let mut ns = sv.clone();
      ns[i] = flipped;
      out.push(ns.iter().collect());
    }
  }
  out
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .input
        .iter()
        .map(|v| v.to_owned())
        .map(|v| count_cols(v, &(0, 0)))
        .fold(0, |acc, (v, h)| acc + v + h * 100),
    ))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .input
        .iter()
        .map(|v| {
          let p1_ans = count_cols(v.clone(), &(0, 0));
          possible_smudges(v)
            .iter()
            .find_map(|v| match count_cols(v.clone(), &p1_ans) {
              (0, 0) => None,
              x => Some(x),
            })
            .unwrap()
        })
        .fold(0, |acc, (v, h)| acc + v + h * 100),
    ))
  }
}
