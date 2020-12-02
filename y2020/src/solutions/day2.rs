use crate::day::{Day, DayArg};
use crate::util::read_input;

use std::error::Error;

pub struct Day2 {
  input: Vec<PasswordEntry>,
}

#[derive(Debug)]
struct PasswordEntry {
  bounds: (i32, i32),
  ch: String,
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

    let bounds = tokens
      .next()
      .ok_or("Bounds Missing".into())
      .and_then(Day2::split_bounds)
      .unwrap();

    let ch = tokens.next().and_then(|s| s.get(0..1)).unwrap().to_string();

    let pass = tokens.next().unwrap().to_string();

    PasswordEntry { bounds, ch, pass }
  }

  fn split_bounds(bounds: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let mut t = bounds.split("-");
    let min = t
      .next()
      .ok_or("Missing lower bound")
      .map(|s| s.parse::<i32>())??;
    let max = t
      .next()
      .ok_or("Missing upper bound")
      .map(|s| s.parse::<i32>())??;
    Ok((min, max))
  }
}

impl Day for Day2 {
  fn p1(&self) -> Result<String, Box<dyn Error>> {
    Ok(
      self
        .input
        .iter()
        .filter(|pwd| {
          let cnt = pwd.pass.matches(&pwd.ch).count() as i32;
          ((pwd.bounds.0)..(pwd.bounds.1) + 1).contains(&cnt)
        })
        .count()
        .to_string(),
    )
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok("".to_string())
  }
}
