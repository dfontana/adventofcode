use crate::day::{Day, DayArg};
use crate::util::read_input;

use std::error::Error;

pub struct Day2 {
  input: Vec<PasswordEntry>,
}

#[derive(Debug)]
struct PasswordEntry {
  bounds: (usize, usize),
  ch: char,
  pass: String,
}

impl Day2 {
  pub fn new() -> Result<Day2, Box<dyn Error>> {
    let input: Vec<PasswordEntry> = read_input(DayArg::D(2))?
      .lines()
      .map(Day2::parse_line)
      .collect();
    Ok(Day2 { input })
  }

  fn parse_line(line: &str) -> PasswordEntry {
    let mut tokens = line.split_ascii_whitespace();
    PasswordEntry {
      bounds: tokens
        .next()
        .ok_or("Bounds Missing".into())
        .and_then(Day2::split_bounds)
        .unwrap(),
      ch: tokens.next().and_then(|s| s.chars().nth(0)).unwrap(),
      pass: tokens.next().unwrap().to_string(),
    }
  }

  fn split_bounds(bounds: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let mut t = bounds.split("-");
    Ok((
      t.next()
        .ok_or("Missing lower bound")
        .map(|s| s.parse::<usize>())??,
      t.next()
        .ok_or("Missing upper bound")
        .map(|s| s.parse::<usize>())??,
    ))
  }
}

impl Day for Day2 {
  fn p1(&self) -> Result<String, Box<dyn Error>> {
    Ok(
      self
        .input
        .iter()
        .filter(|pwd| {
          let cnt = pwd.pass.matches(pwd.ch).count();
          ((pwd.bounds.0)..(pwd.bounds.1) + 1).contains(&cnt)
        })
        .count()
        .to_string(),
    )
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok(
      self
        .input
        .iter()
        .filter(|pwd| {
          let mut chars = pwd.pass.chars();
          let lower = pwd.bounds.0 - 1;
          let upper = pwd.bounds.1 - pwd.bounds.0 - 1;
          chars
            .nth(lower)
            .filter(|s| s == &pwd.ch)
            .xor(chars.nth(upper).filter(|s| s == &pwd.ch))
            .is_some()
        })
        .count()
        .to_string(),
    )
  }
}
