use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::collections::HashMap;
use std::error::Error;

pub struct Solve {
  jolts: Vec<i32>,
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    Ok(Solve {
      jolts: read_input(d)?
        .lines()
        .map(|i| i.parse::<i32>())
        .flatten()
        .collect(),
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    Ok(p1_mut(&self.jolts).to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok("Impl".to_string())
  }
}

fn p1_imm(jolts: &Vec<i32>) -> i32 {
  // ~400-600us
  let mut copy = jolts.clone();
  copy.sort();
  copy.push(copy[copy.len() - 1] + 3);
  copy.insert(0, 0);
  let counts = copy
    .windows(2)
    .map(|win| win[1] - win[0])
    .fold(HashMap::new(), |mut acc, diff| {
      acc.entry(diff).and_modify(|v| *v += 1).or_insert(1);
      acc
    });
  counts.get(&1).unwrap_or(&0) * counts.get(&3).unwrap_or(&0)
}

fn p1_mut(jolts: &Vec<i32>) -> i32 {
  // ~200-300us
  let mut copy = jolts.clone();
  copy.sort();
  copy.push(copy[copy.len() - 1] + 3);
  let mut c1 = 0;
  let mut c3 = 0;
  let mut prev = 0;
  for i in copy.iter() {
    match i - prev {
      1 => c1 += 1,
      3 => c3 += 1,
      _ => (),
    }
    prev = *i;
  }
  c1 * c3
}
