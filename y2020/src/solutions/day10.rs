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
    // 336ms
    let mut copy = self.jolts.clone();
    copy.sort();
    copy.push(copy[copy.len() - 1] + 3);
    copy.insert(0, 0);
    let counts =
      copy
        .windows(2)
        .map(|win| win[1] - win[0])
        .fold(HashMap::new(), |mut acc, diff| {
          acc.entry(diff).and_modify(|v| *v += 1).or_insert(1);
          acc
        });
    let ans = counts.get(&1).unwrap_or(&0) * counts.get(&3).unwrap_or(&0);
    Ok(ans.to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok("Impl".to_string())
  }
}
