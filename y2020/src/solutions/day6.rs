use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::collections::HashMap;
use std::error::Error;

// Part1: 6778
// Part2: 3406

pub struct Solve {
  answers: Vec<Vec<String>>,
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    let answers = read_input(d)?
      .split("\n\n")
      .map(|records| records.split_whitespace().map(str::to_string).collect())
      .collect();
    Ok(Solve { answers })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    let result = self
      .answers
      .iter()
      .map(|set| {
        set.iter().fold(HashMap::new(), |mut acc, ans| {
          ans.chars().for_each(|c| {
            acc.insert(c, true);
          });
          acc
        })
      })
      .map(|map| map.len())
      .sum::<usize>()
      .to_string();
    Ok(result)
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    let result = self
      .answers
      .iter()
      .map(|set| {
        let set_size = set.len() as i32;
        let mut counts = set.iter().fold(HashMap::new(), |mut acc, ans| {
          ans.chars().for_each(|c| {
            acc.entry(c).and_modify(|e| *e += 1).or_insert(1);
          });
          acc
        });
        counts.retain(|_, ct| *ct == set_size);
        counts.len()
      })
      .sum::<usize>()
      .to_string();
    Ok(result)
  }
}
