use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::collections::HashMap;
use std::error::Error;

// Part1: 6778
// Part2: 3406

pub struct Solve {
  answers: Vec<(usize, HashMap<char, i32>)>,
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    let answers = read_input(d)?
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
      .collect();
    Ok(Solve { answers })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    let result = self
      .answers
      .iter()
      .map(|(_, map)| map.len())
      .sum::<usize>()
      .to_string();
    Ok(result)
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    let result = self
      .answers
      .iter()
      .map(|(group_size, map)| {
        map
          .values()
          .filter(|ct| **ct == *group_size as i32)
          .fold(0, |acc, _| acc + 1)
      })
      .sum::<usize>()
      .to_string();
    Ok(result)
  }
}
