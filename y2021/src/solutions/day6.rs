use rust_util::{AocDay, Day};
use std::{collections::HashMap, error::Error, fmt::Display};

pub struct Solve {
  ans: (usize, usize),
}

const SPAWN_DAY: i32 = -1;

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    Ok(Box::new(Solve {
      ans: simulate(
        rust_util::read_input(2021, d)?
          .trim()
          .split(",")
          .map(|v| (v.parse::<i32>().unwrap(), 1))
          .fold(HashMap::new(), |mut acc, (timer, count)| {
            acc
              .entry(timer)
              .and_modify(|v| *v += count)
              .or_insert(count);
            acc
          }),
        80,
        256,
      ),
    }))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(self.ans.0))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(self.ans.1))
  }
}

fn simulate(env: HashMap<i32, usize>, d1: usize, d2: usize) -> (usize, usize) {
  let mut d1v = 0;
  let d2v = (0..d2)
    .fold(env, |acc, d| {
      if d == d1 {
        // First day hit, record the current spawn count
        d1v = acc.iter().fold(0, |acc, fish| acc + fish.1);
      }
      // Tick down timers & spawn the new fishies
      // Being sure to remove the amount spawned from the prior count
      let mut tmp = acc.clone();
      acc.iter().for_each(|(timer, count)| {
        tmp.entry(*timer).and_modify(|v| *v -= count).or_insert(0);
        let next_spawn = timer - 1;
        if next_spawn == SPAWN_DAY {
          tmp.entry(6).and_modify(|v| *v += count).or_insert(*count);
          tmp.entry(8).and_modify(|v| *v += count).or_insert(*count);
        } else {
          tmp
            .entry(next_spawn)
            .and_modify(|v| *v += count)
            .or_insert(*count);
        }
      });
      tmp
    })
    .iter()
    .fold(0, |acc, fish| acc + fish.1);
  (d1v, d2v)
}
