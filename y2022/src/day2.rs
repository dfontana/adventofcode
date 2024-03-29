use rust_util::Day;
use std::{error::Error, fmt::Display};

pub struct Solve {
  pairs: Vec<(String, String)>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      pairs: value
        .lines()
        .map(|l| {
          let mut split = l.split(' ');
          (
            split.next().unwrap().to_owned(),
            split.next().unwrap().to_owned(),
          )
        })
        .collect(),
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let score: usize = self
      .pairs
      .iter()
      .map(|(l, r)| {
        let value = match r.as_str() {
          "X" => 1,
          "Y" => 2,
          "Z" => 3,
          _ => unreachable!(),
        };
        let win = match (l.as_str(), r.as_str()) {
          ("A", "X") => 3,
          ("B", "Y") => 3,
          ("C", "Z") => 3,
          ("C", "X") => 6,
          ("A", "Y") => 6,
          ("B", "Z") => 6,
          _ => 0,
        };
        value + win
      })
      .sum();
    Ok(Box::new(score))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let score: usize = self
      .pairs
      .iter()
      .map(|(l, outcome)| match outcome.as_str() {
        "X" => match l.as_str() {
          "A" => 3,
          "B" => 1,
          "C" => 2,
          _ => unreachable!(),
        },
        "Y" => {
          3 + match l.as_str() {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => unreachable!(),
          }
        }
        "Z" => {
          6 + match l.as_str() {
            "A" => 2,
            "B" => 3,
            "C" => 1,
            _ => unreachable!(),
          }
        }
        _ => unreachable!(),
      })
      .sum();
    Ok(Box::new(score))
  }
}
