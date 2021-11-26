use rust_util::{read_input, AocDay, Day};
use std::{error::Error, fmt::Display};

pub struct Solve {
  data: Vec<i32>,
}

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    Ok(Box::new(Solve {
      data: read_input(2020, d)?
        .lines()
        .map(|s| s.parse::<i32>())
        .flatten()
        .collect(),
    }))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    match find_no_sum_num(&self.data).map(|v| v.to_string()) {
      Some(ans) => Ok(Box::new(format!("{}", ans))),
      None => Err("No solve found".into()),
    }
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let target = find_no_sum_num(&self.data).ok_or("No invalid nums found")?;
    let sum_list = (0..self.data.len())
      .into_iter()
      .find_map(|f| has_contiguous_sum(&self.data[f..], target))
      .ok_or("No solution found")?;
    Ok(Box::new(
      (sum_list.iter().min().unwrap() + sum_list.iter().max().unwrap()).to_string(),
    ))
  }
}

fn find_no_sum_num(list: &Vec<i32>) -> Option<i32> {
  list
    .windows(26)
    .map(|win| (&win[..win.len() - 1], win[win.len() - 1]))
    .find(|(win, goal)| {
      !win
        .iter()
        .any(|w| (goal - w) != *w && win.contains(&(goal - w)))
    })
    .map(|(_, v)| v)
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
