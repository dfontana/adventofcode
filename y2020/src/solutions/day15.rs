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
    Ok(play_game(2020, &self.numbers).to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok(play_game(30000000, &self.numbers).to_string())
  }
}

fn play_game(limit: usize, numbers: &Vec<usize>) -> usize {
  let mut ages: HashMap<usize, (usize, usize)> =
    numbers
      .iter()
      .enumerate()
      .fold(HashMap::new(), |mut acc, (t, v)| {
        acc.insert(*v, (t + 1, t + 1));
        acc
      });
  let mut turn = ages.len() + 1;
  let mut last_spoken = numbers[numbers.len() - 1];
  while turn <= limit {
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
  last_spoken
}
