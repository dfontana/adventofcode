use rust_util::{read_input, AocDay, Day};
use std::{error::Error, fmt::Display};

pub struct Solve {
  numbers: Vec<usize>,
}

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    Ok(Box::new(Solve {
      numbers: read_input(2020, d)?
        .split(',')
        .map(|c| c.parse::<usize>())
        .flatten()
        .collect(),
    }))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(play_game(2020, &self.numbers).to_string()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(play_game(30000000, &self.numbers).to_string()))
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
