use rust_util::{read_input, AocDay, Day};
use std::{error::Error, fmt::Display};

pub struct Solve {
  seats: Vec<i32>,
}

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    let mut seats = read_input(2020, d)?
      .lines()
      .map(|s| search(64, &s[..7]) * 8 + search(4, &s[7..]))
      .collect::<Vec<i32>>();
    seats.sort();
    Ok(Box::new(Solve { seats }))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let ans = self
      .seats
      .last()
      .map(|s| s.to_string())
      .ok_or("No Seats Parsed".into());
    match ans {
      Ok(v) => Ok(Box::new(v)),
      Err(e) => Err(e),
    }
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let ans = self
      .seats
      .windows(3)
      .filter(|win| win[0] + 1 != win[1] || win[1] + 1 != win[2])
      .next()
      .map(|win| (win[1] + 1).to_string())
      .ok_or("No Missing Seat Found".into());
    match ans {
      Ok(v) => Ok(Box::new(v)),
      Err(e) => Err(e),
    }
  }
}

fn search(init_mask: i32, loc: &str) -> i32 {
  loc
    .chars()
    .fold((0, init_mask), |(num, mask), c| match c {
      'B' | 'R' => (num | mask, mask >> 1),
      _ => (num, mask >> 1),
    })
    .0
}
