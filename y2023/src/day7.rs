use itertools::Itertools;
use rust_util::Day;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct Solve {
  hands: Vec<Hand>,
}

#[derive(Debug)]
struct Hand {
  typ: HandType,
  cards: [Card; 5],
  bid: usize,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
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

impl From<&[Card; 5]> for HandType {
  fn from(value: &[Card; 5]) -> Self {
    let mut cards = value.clone();
    cards.sort();
    let mut compressed = [0; 5];
    let mut i = 0;
    let mut last_seen: Option<Card> = None;
    for card in cards {
      match last_seen {
        None => compressed[i] += 1,
        Some(c) => {
          if c == card {
            compressed[i] += 1;
          } else {
            i += 1;
            compressed[i] += 1;
          }
        }
      }
      last_seen = Some(card);
    }
    compressed.sort();
    compressed.reverse();
    match compressed {
      [5, 0, 0, 0, 0] => HandType::Five,
      [4, 1, 0, 0, 0] => HandType::Four,
      [3, 2, 0, 0, 0] => HandType::Full,
      [3, 1, 1, 0, 0] => HandType::Three,
      [2, 2, 1, 0, 0] => HandType::Two,
      [2, 1, 1, 1, 0] => HandType::One,
      [1, 1, 1, 1, 1] => HandType::High,
      _ => unreachable!("{:?}\n{:?}", cards, compressed),
    }
  }
}

impl Eq for Hand {}
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
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    if self.typ > other.typ {
      return Some(std::cmp::Ordering::Greater);
    } else if self.typ < other.typ {
      return Some(std::cmp::Ordering::Less);
    }

    for i in 0..=4 {
      if self.cards[i] > other.cards[i] {
        return Some(std::cmp::Ordering::Greater);
      } else if self.cards[i] < other.cards[i] {
        return Some(std::cmp::Ordering::Less);
      }
    }
    Some(std::cmp::Ordering::Equal)
  }
}

impl Ord for Hand {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.partial_cmp(other).unwrap()
  }
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      hands: value
        .lines()
        .filter_map(|s| s.split_once(" "))
        .filter_map(|s| Hand::try_from(s).ok())
        .collect(),
    })
  }
}
impl TryFrom<(&str, &str)> for Hand {
  type Error = Box<dyn Error>;

  fn try_from((hand, bid): (&str, &str)) -> Result<Self, Self::Error> {
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
        'J' => Card::J,
        'Q' => Card::Q,
        'K' => Card::K,
        'A' => Card::A,
        _ => unreachable!(),
      })
      .collect_vec();
    let cards = [c[0], c[1], c[2], c[3], c[4]];
    Ok(Hand {
      typ: HandType::from(&cards),
      cards,
      bid: bid.parse()?,
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .hands
        .iter()
        .sorted()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum::<usize>(),
    ))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(1))
  }
}
