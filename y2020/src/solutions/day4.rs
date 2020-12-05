use crate::day::{Day, DayArg};
use crate::util::read_input;

use std::collections::HashMap;
use std::error::Error;

const REQUIRED_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

pub struct Day4 {
  passports: Vec<HashMap<String, String>>,
}

impl Day for Day4 {
  fn new() -> Result<Day4, Box<dyn Error>> {
    let passports = read_input(DayArg::D(4))?
      .split("\n\n")
      .map(|passport| {
        passport
          .split_whitespace()
          .map(|token| token.splitn(2, ":"))
          .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
          .map(|(k, v)| (k.to_string(), v.to_string()))
          .collect()
      })
      .collect::<Vec<HashMap<String, String>>>();
    Ok(Day4 { passports })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    Ok(
      self
        .passports
        .iter()
        .filter(|f| {
          REQUIRED_KEYS
            .iter()
            .filter(|k| !f.contains_key(**k))
            .count()
            == 0
        })
        .count()
        .to_string(),
    )
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok("Not implemented".to_string())
  }
}
