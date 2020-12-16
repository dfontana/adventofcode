extern crate regex;
use regex::Captures;
use regex::Regex;

use crate::day::{Day, DayArg};
use crate::util::read_input;
use core::panic;
use std::{collections::HashSet, error::Error, ops::RangeInclusive};

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
    Ok(
      self
        .other_tickets
        .iter()
        .fold(0, |acc, ticket| {
          acc
            + ticket
              .iter()
              .filter(|v| {
                !self
                  .rules
                  .iter()
                  .any(|rule| rule.lower.contains(v) || rule.upper.contains(v))
              })
              .sum::<u64>()
        })
        .to_string(),
    )
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    let mut valid_tickets: Vec<Vec<u64>> = self
      .other_tickets
      .iter()
      .filter(|ticket| {
        ticket
          .iter()
          .filter(|v| {
            !self
              .rules
              .iter()
              .any(|rule| rule.lower.contains(v) || rule.upper.contains(v))
          })
          .next()
          .is_none()
      })
      .map(|v| v.clone())
      .collect();
    valid_tickets.push(self.my_ticket.clone());

    // Transpose so we look at indicies together
    let by_idx: Vec<Vec<u64>> = (0..valid_tickets[0].len())
      .map(|i| {
        valid_tickets
          .iter()
          .map(|inner| inner[i].clone())
          .collect::<Vec<u64>>()
      })
      .collect();

    // Test each index against all rules, assigning rule to index as we find them
    let mut res: Vec<(Rule, Vec<usize>)> = self
      .rules
      .iter()
      .map(|r| {
        (
          r.clone(),
          by_idx
            .iter()
            .enumerate()
            .filter(|(_, vals)| {
              vals
                .iter()
                .filter(|v| !r.lower.contains(v) && !r.upper.contains(v))
                .next()
                .is_none()
            })
            .map(|(i, _)| i)
            .collect::<Vec<usize>>(),
        )
      })
      .collect();

    let mut taken: HashSet<usize> = HashSet::new();
    let mut assigns: Vec<(Rule, usize)> = Vec::new();
    res.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

    res.iter().for_each(
      |(r, idxs)| match idxs.iter().filter(|v| !taken.contains(v)).next() {
        Some(i) => {
          assigns.push((r.clone(), *i));
          taken.insert(*i);
        }
        None => panic!("Failed to assign idx"),
      },
    );

    let ans = assigns
      .iter()
      .filter(|(r, _)| r.name.starts_with("departure"))
      .map(|(_, i)| self.my_ticket[*i])
      .product::<u64>();

    Ok(ans.to_string())
  }
}
