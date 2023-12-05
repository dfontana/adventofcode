use itertools::Itertools;
use rust_util::Day;
use std::{
  collections::{HashMap, HashSet},
  error::Error,
  fmt::Display,
};

#[derive(Debug)]
pub struct Solve {
  cards: Vec<Card>,
}
#[derive(Debug)]
struct Card {
  id: usize,
  matching: usize,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      cards: value
        .lines()
        .map(|line| {
          let (id, wins, nums) = line
            .split_once(" | ")
            .and_then(|(card, nums)| {
              card
                .strip_prefix("Card ")
                .and_then(|c| c.split_once(": "))
                .map(|(id, wins)| (id, wins, nums))
            })
            .unwrap();
          let winners = wins
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<HashSet<usize>>();
          let numbers = nums
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<HashSet<usize>>();
          Card {
            id: id.trim().parse::<usize>().unwrap(),
            matching: numbers.intersection(&winners).count(),
          }
        })
        .collect_vec(),
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .cards
        .iter()
        .map(|c| (0..c.matching).fold(0.5, |acc, _| acc * 2.0) as usize)
        .sum::<usize>(),
    ))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .cards
        .iter()
        .map(|c| (c.id, c.matching))
        .fold(HashMap::new(), |mut acc, (id, mc)| {
          let cards: usize = *acc.entry(id).or_insert(1);
          for won in 1..=mc {
            acc
              .entry(id + won)
              .and_modify(|v| *v += cards)
              .or_insert(1 + cards);
          }
          acc
        })
        .into_values()
        .sum::<usize>(),
    ))
  }
}
