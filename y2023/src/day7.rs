use itertools::Itertools;
use rust_util::Day;
use std::{cmp::Ordering, error::Error, fmt::Display};

use crate::tokens::Parser;

#[derive(Debug)]
pub struct Solve {
  input: Vec<(String, usize)>,
}

#[derive(Debug, Eq, Ord)]
struct Hand {
  typ: HandType,
  cards: [Card; 5],
  bid: usize,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
  JK,
  N2,
  N3,
  N4,
  N5,
  N6,
  N7,
  N8,
  N9,
  T,
  J,
  Q,
  K,
  A,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
  High,
  One,
  Two,
  Three,
  Full,
  Four,
  Five,
}

impl PartialEq for Hand {
  fn eq(&self, other: &Self) -> bool {
    for i in 0..=4 {
      if self.cards[i] != other.cards[i] {
        return false;
      }
    }
    return true;
  }
}

impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match self.typ.cmp(&other.typ) {
      Ordering::Equal => (0..=4)
        .map(|i| self.cards[i].cmp(&other.cards[i]))
        .filter(|cmp| *cmp != Ordering::Equal)
        .next()
        .or(Some(Ordering::Equal)),
      c => Some(c),
    }
  }
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      input: Parser::new(&value)
        .lines(
          Parser::lazy()
          .take_non_whitespace()
          .take_usize()
        )
        .collect(),
    })
  }
}

impl From<&[Card; 5]> for HandType {
  fn from(value: &[Card; 5]) -> Self {
    let mut cards = value.clone();
    cards.sort();
    let mut compressed = [0; 5];
    let mut i = 0;
    let mut last_seen: Option<Card> = None;
    let mut jokers = 0;
    for card in cards {
      if card == Card::JK {
        jokers += 1;
        continue;
      }
      if let Some(c) = last_seen {
        if c != card {
          i += 1;
        }
      }
      compressed[i] += 1;
      last_seen = Some(card);
    }
    compressed.sort();
    compressed.reverse();
    let mut hand = match compressed {
      [5, 0, 0, 0, 0] => HandType::Five,
      [4, 1 | 0, 0, 0, 0] => HandType::Four,
      [3, 2, 0, 0, 0] => HandType::Full,
      [3, 1 | 0, 1 | 0, 0, 0] => HandType::Three,
      [2, 2, 1 | 0, 0, 0] => HandType::Two,
      [2, 1 | 0, 1 | 0, 1 | 0, 0] => HandType::One,
      [1 | 0, 1 | 0, 1 | 0, 1 | 0, 1 | 0] => HandType::High,
      _ => unreachable!("\n{:?}\n{:?}", cards, compressed),
    };
    for _ in 0..jokers {
      hand = match hand {
        HandType::High => HandType::One,
        HandType::One => HandType::Three,
        HandType::Two => HandType::Full,
        HandType::Three | HandType::Full => HandType::Four,
        HandType::Four | HandType::Five => HandType::Five,
      }
    }
    hand
  }
}

impl Solve {
  fn total_winnings(&self, with_jokers: bool) -> usize {
    self
      .input
      .iter()
      .map(|(hand, bid)| {
        let c = hand
          .chars()
          .map(|c| match c {
            '2' => Card::N2,
            '3' => Card::N3,
            '4' => Card::N4,
            '5' => Card::N5,
            '6' => Card::N6,
            '7' => Card::N7,
            '8' => Card::N8,
            '9' => Card::N9,
            'T' => Card::T,
            'J' if !with_jokers => Card::J,
            'J' if with_jokers => Card::JK,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => unreachable!(),
          })
          .collect_vec();
        let cards = [c[0], c[1], c[2], c[3], c[4]];
        Hand {
          typ: HandType::from(&cards),
          cards,
          bid: *bid,
        }
      })
      .sorted()
      .enumerate()
      .map(|(rank, hand)| (rank + 1) * hand.bid)
      .sum::<usize>()
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(self.total_winnings(false)))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(self.total_winnings(true)))
  }
}
