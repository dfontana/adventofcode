use crate::day::{Day, DayArg};
use crate::util::read_input;
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
  let mut ages = vec![0; limit];
  numbers
    .iter()
    .enumerate()
    .for_each(|(i, v)| ages[*v] = i + 1);
  let mut last_spoken = numbers[numbers.len() - 1];
  for turn in numbers.len()..limit {
    let next_spoken = ages[last_spoken];
    ages[last_spoken] = turn;
    last_spoken = match next_spoken {
      0 => 0,
      _ => turn - next_spoken,
    };
  }
  last_spoken
}
