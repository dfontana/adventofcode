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
      .map(|s| search(127, &s[..7]) * 8 + search(7, &s[7..]))
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

fn search(max_bound: usize, loc: &str) -> i32 {
  let (ops, selector) = (&loc[..loc.len() - 1], &loc[loc.len() - 1..]);
  let (min, max) = ops
    .chars()
    .fold((0.0, max_bound as f64), |(min, max), c| match c {
      'L' | 'F' => (min, (max - ((max - min) / 2.0)).floor()),
      'B' | 'R' => ((min + ((max - min) / 2.0)).ceil(), max),
      _ => (min, max),
    });
  match selector {
    "L" | "F" => min as i32,
    "B" | "R" => max as i32,
    _ => 0,
  }
}
