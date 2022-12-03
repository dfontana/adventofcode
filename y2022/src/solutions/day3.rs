use rust_util::{AocDay, Day};
use std::{collections::HashSet, error::Error, fmt::Display};

pub struct Solve {
  bags: Vec<String>,
}

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    let instr = rust_util::read_input(2022, d)?;
    let bags: Vec<String> = instr.lines().map(str::to_string).collect();
    Ok(Box::new(Solve { bags }))
  }

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
