use rust_util::Day;
use std::{collections::HashSet, error::Error, fmt::Display};

pub struct Solve {
  bags: Vec<String>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      bags: value.lines().map(str::to_string).collect(),
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let score: u32 = self
      .bags
      .iter()
      .map(|l| l.split_at(l.len() / 2))
      .map(|(a, b)| (a.to_owned(), b.to_owned()))
      .map(|(a, b)| {
        (
          a.chars().collect::<HashSet<char>>(),
          b.chars().collect::<HashSet<char>>(),
        )
      })
      .map(|(a, b)| *a.intersection(&b).next().unwrap())
      .map(|c| {
        if c.is_uppercase() {
          26 + (c as u32 - 64)
        } else {
          c as u32 - 96
        }
      })
      .sum();
    Ok(Box::new(score))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let score: u32 = self
      .bags
      .chunks_exact(3)
      .map(|ck| {
        (
          ck[0].chars().collect::<HashSet<char>>(),
          ck[1].chars().collect::<HashSet<char>>(),
          ck[2].chars().collect::<HashSet<char>>(),
        )
      })
      .map(|(a, b, c)| {
        let b1: HashSet<char> = a.intersection(&b).copied().collect();
        *c.intersection(&b1).next().unwrap()
      })
      .map(|c| {
        if c.is_uppercase() {
          26 + (c as u32 - 64)
        } else {
          c as u32 - 96
        }
      })
      .sum();
    Ok(Box::new(score))
  }
}
