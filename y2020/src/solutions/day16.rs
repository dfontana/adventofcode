extern crate regex;
use regex::Captures;
use regex::Regex;

use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::{error::Error, ops::RangeInclusive};

lazy_static! {
  static ref CAPTURE_RULE: Regex =
    Regex::new("^([a-zA-Z\\s]+): (\\d+)-(\\d+) or (\\d+)-(\\d+)$").unwrap();
}

#[derive(Debug)]
pub struct Solve {
  rules: Vec<Rule>,
  my_ticket: Vec<u64>,
  other_tickets: Vec<Vec<u64>>,
}

#[derive(Debug, Clone)]
struct Rule {
  name: String,
  lower: RangeInclusive<u64>,
  upper: RangeInclusive<u64>,
}

fn parse_range(caps: &Captures, l: usize, u: usize) -> RangeInclusive<u64> {
  RangeInclusive::new(
    caps[l].parse::<u64>().unwrap(),
    caps[u].parse::<u64>().unwrap(),
  )
}

fn read_ticket(inp: &str) -> Vec<Vec<u64>> {
  inp
    .lines()
    .skip(1)
    .map(|l| l.split(',').map(|v| v.parse::<u64>().unwrap()).collect())
    .collect()
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    let input = read_input(d)?;
    let mut sections = input.split("\n\n");
    Ok(Solve {
      rules: sections
        .next()
        .unwrap()
        .lines()
        .map(|l| {
          let caps = CAPTURE_RULE.captures(l).unwrap();
          Rule {
            name: caps[1].to_string(),
            lower: parse_range(&caps, 2, 3),
            upper: parse_range(&caps, 4, 5),
          }
        })
        .collect(),
      my_ticket: read_ticket(sections.next().unwrap())[0].clone(),
      other_tickets: read_ticket(sections.next().unwrap()),
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    println!("{:?}", self);
    Ok("Impl".to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok("Impl".to_string())
  }
}
