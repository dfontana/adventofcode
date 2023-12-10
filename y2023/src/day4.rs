use rust_util::Day;
use std::{
  collections::{HashMap, HashSet},
  error::Error,
  fmt::Display,
};

use crate::tokens::Parser;

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
      cards: Parser::new(&value)
        .lines(
          Parser::lazy()
          .consume("Card")
          .consume_whitespace()
          .take_usize()
          .consume(":")
          .take_usizes()
          .consume("|")
          .take_usizes()
        )
        .map(|(id,wins,nums): (usize, HashSet<usize>, HashSet<usize>)| {
          Card {
            id,
            matching: nums.intersection(&wins).count(),
          }
        })
        .collect(),
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
