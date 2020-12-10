use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::error::Error;

pub struct Solve {
  data: Vec<i32>,
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    Ok(Solve {
      data: read_input(d)?
        .lines()
        .map(|s| s.parse::<i32>())
        .flatten()
        .collect(),
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    find_no_sum_num(&self.data)
      .map(|v| v.to_string())
      .ok_or("No invalid nums found".into())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    let target = find_no_sum_num(&self.data).ok_or("No invalid nums found")?;
    for start in 0..self.data.len() {
      match has_contiguous_sum(&self.data[start..], target) {
        None => continue,
        Some(list) => return Ok(sum_min_max(&list).to_string()),
      }
    }
    Err("No solution found".into())
  }
}

fn find_no_sum_num(list: &Vec<i32>) -> Option<i32> {
  list
    .windows(26)
    .map(|win| (&win[..win.len() - 1], win[win.len() - 1]))
    .find(has_no_pair)
    .map(|(_, v)| v)
}

fn sum_min_max(list: &Vec<i32>) -> i32 {
  let min = match list.iter().min() {
    Some(v) => v,
    None => &0,
  };
  let max = match list.iter().max() {
    Some(v) => v,
    None => &0,
  };
  min + max
}

fn has_no_pair(pair: &(&[i32], i32)) -> bool {
  for (i, v1) in pair.0.iter().enumerate() {
    for v2 in pair.0[i + 1..].iter() {
      if v1 + v2 == pair.1 {
        return false;
      }
    }
  }
  true
}

fn has_contiguous_sum(list: &[i32], val: i32) -> Option<Vec<i32>> {
  let mut sum = 0;
  let mut idx = 0;
  while sum != val && sum < val {
    sum += list[idx];
    idx += 1;
  }
  match sum == val {
    false => None,
    true => Some(list[..idx].to_vec()),
  }
}
