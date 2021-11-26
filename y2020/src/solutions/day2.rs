use rust_util::{read_input, AocDay, Day};

use std::{error::Error, fmt::Display};

pub struct Solve {
  input: Vec<PasswordEntry>,
}

#[derive(Debug)]
struct PasswordEntry {
  bounds: (usize, usize),
  ch: char,
  pass: String,
}

fn parse_line(line: &str) -> PasswordEntry {
  let mut tokens = line.split_ascii_whitespace();
  PasswordEntry {
    bounds: tokens
      .next()
      .ok_or("Bounds Missing".into())
      .and_then(split_bounds)
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

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    let input: Vec<PasswordEntry> = read_input(2020, d)?.lines().map(parse_line).collect();
    Ok(Box::new(Solve { input }))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .input
        .iter()
        .filter(|pwd| {
          let cnt = pwd.pass.matches(pwd.ch).count();
          ((pwd.bounds.0)..(pwd.bounds.1) + 1).contains(&cnt)
        })
        .count()
        .to_string(),
    ))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
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
    ))
  }
}
