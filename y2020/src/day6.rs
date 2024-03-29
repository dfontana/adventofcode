use rust_util::{Day};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

pub struct Solve {
  answers: Vec<(i32, HashMap<char, i32>)>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      answers: value 
        .split("\n\n")
        .map(|records| {
          records
            .split_whitespace()
            .fold((0, HashMap::new()), |(group_count, mut acc), ans| {
              ans.chars().for_each(|c| {
                acc.entry(c).and_modify(|e| *e += 1).or_insert(1);
              });
              (group_count + 1, acc)
            })
        })
        .collect(),
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let result = self
      .answers
      .iter()
      .map(|(_, map)| map.len())
      .sum::<usize>()
      .to_string();
    Ok(Box::new(result))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let result = self
      .answers
      .iter()
      .map(|(group_size, map)| map.values().filter(|ct| **ct == *group_size).count())
      .sum::<usize>()
      .to_string();
    Ok(Box::new(result))
  }
}
