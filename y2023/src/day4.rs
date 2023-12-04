use itertools::Itertools;
use regex::Regex;
use rust_util::Day;
use std::{collections::HashSet, error::Error, fmt::Display};

#[derive(Debug)]
pub struct Solve {
  cards: Vec<Card>,
}
#[derive(Debug)]
struct Card {
  id: usize,
  winners: HashSet<usize>,
  numbers: HashSet<usize>,
}

impl Card {
  fn points(&self) -> usize {
    self
      .numbers
      .intersection(&self.winners)
      .fold(0.5, |acc, _| acc * 2.0) as usize
  }
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let re =
      Regex::new(r"Card\s+(?<id>[0-9]+):\s+(?<winners>(?:\d+\s*)+)+\|\s+(?<numbers>(?:\d+\s*)+)+")
        .unwrap();

    Ok(Solve {
      cards: value
        .lines()
        .map(|line| {
          let caps = re.captures(line).unwrap();
          let id = caps.name("id").unwrap().as_str().parse::<usize>().unwrap();
          let winners = caps
            .name("winners")
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
          let numbers = caps
            .name("numbers")
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
          Card {
            id,
            winners,
            numbers,
          }
        })
        .collect(),
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(self.cards.iter().map(Card::points).sum::<usize>()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new("todo"))
  }
}
