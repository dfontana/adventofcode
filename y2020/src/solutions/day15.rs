use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::collections::HashMap;
use std::error::Error;

pub struct Solve {
  numbers: Vec<usize>,
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    Ok(Solve {
      numbers: read_input(d)?
        .split(',')
        .map(|c| c.parse::<usize>())
        .flatten()
        .collect(),
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    let mut ages: HashMap<usize, (usize, usize)> =
      self
        .numbers
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (t, v)| {
          acc.insert(*v, (t + 1, t + 1));
          acc
        });
    let mut turn = ages.len() + 1;
    let mut last_spoken = self.numbers[self.numbers.len() - 1];
    while turn <= 2020 {
      last_spoken = match ages.get(&last_spoken) {
        Some((old, new)) => *new - *old,
        None => 0,
      };
      ages.insert(
        last_spoken,
        match ages.get(&last_spoken) {
          Some((_, new)) => (*new, turn),
          None => (turn, turn),
        },
      );
      turn += 1;
    }
    Ok(last_spoken.to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    let mut ages: HashMap<usize, (usize, usize)> =
      self
        .numbers
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (t, v)| {
          acc.insert(*v, (t + 1, t + 1));
          acc
        });
    let mut turn = ages.len() + 1;
    let mut last_spoken = self.numbers[self.numbers.len() - 1];
    while turn <= 30000000 {
      last_spoken = match ages.get(&last_spoken) {
        Some((old, new)) => *new - *old,
        None => 0,
      };
      ages.insert(
        last_spoken,
        match ages.get(&last_spoken) {
          Some((_, new)) => (*new, turn),
          None => (turn, turn),
        },
      );
      turn += 1;
    }
    Ok(last_spoken.to_string())
  }
}
