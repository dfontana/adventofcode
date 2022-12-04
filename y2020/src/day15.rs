use rust_util::{ Day};
use std::{error::Error, fmt::Display};

pub struct Solve {
  numbers: Vec<usize>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      numbers: value 
        .split(',')
        .map(|c| c.parse::<usize>())
        .flatten()
        .collect(),
    })
  }
}

impl Day for Solve {
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
