use itertools::Itertools;
use rust_util::{AocDay, Day};
use std::{error::Error, fmt::Display};

pub struct Solve {
  positions: Vec<u32>,
}

impl Solve {
  fn run_search<F>(&self, cost_func: F) -> (u32, i32)
  where
    F: Fn(u32, u32) -> i32,
  {
    search(
      &self.positions,
      cost_func,
      0,
      *self.positions.get(self.positions.len() - 1).unwrap(),
      (0, 0),
    )
  }
}

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    Ok(Box::new(Solve {
      positions: rust_util::read_input(2021, d)?
        .trim()
        .split(",")
        .map(|v| v.parse::<u32>().unwrap())
        .sorted()
        .collect(),
    }))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let fuel_cost = |src: u32, dest: u32| (src as i32 - dest as i32).abs();
    let (pos, cost) = self.run_search(fuel_cost);
    Ok(Box::new(format!("{} @ {} fuel", pos, cost)))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let fuel_cost = |src: u32, dest: u32| {
      let n = (src as i32 - dest as i32).abs();
      (n * (n + 1)) / 2
    };
    let (pos, cost) = self.run_search(fuel_cost);
    Ok(Box::new(format!("{} @ {} fuel", pos, cost)))
  }
}

fn search<F>(list: &Vec<u32>, cost_func: F, min: u32, max: u32, ans: (u32, i32)) -> (u32, i32)
where
  F: Fn(u32, u32) -> i32,
{
  if min >= max {
    return ans;
  }
  let middle = (min + max) / 2;
  let l_cost = list.iter().map(|p| cost_func(*p, middle)).sum::<i32>();
  let r_cost = list.iter().map(|p| cost_func(*p, middle + 1)).sum::<i32>();
  match l_cost - r_cost {
    v if v < 0 => search(list, cost_func, min, middle, (middle, l_cost)),
    v if v > 0 => search(list, cost_func, middle + 1, max, (middle + 1, r_cost)),
    _ => ans,
  }
}
