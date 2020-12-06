use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::error::Error;

pub struct Solve {
  seats: Vec<i32>,
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    let mut seats = read_input(d)?
      .lines()
      .map(|s| search(64, &s[..7]) * 8 + search(4, &s[7..]))
      .collect::<Vec<i32>>();
    seats.sort();
    Ok(Solve { seats })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    self
      .seats
      .last()
      .map(|s| s.to_string())
      .ok_or("No Seats Parsed".into())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    self
      .seats
      .windows(3)
      .filter(|win| win[0] + 1 != win[1] || win[1] + 1 != win[2])
      .next()
      .map(|win| (win[1] + 1).to_string())
      .ok_or("No Missing Seat Found".into())
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
