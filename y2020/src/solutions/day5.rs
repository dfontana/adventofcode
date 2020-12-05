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
      .map(|s| (&s[..7], &s[7..]))
      .map(|(row, col)| search(127, &row) * 8 + search(7, &col))
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

fn search(max_bound: i32, loc: &str) -> i32 {
  let mut min = 0;
  let mut max = max_bound;
  for (idx, c) in loc.chars().enumerate() {
    if idx == loc.len() - 1 {
      return match c {
        'L' | 'F' => min,
        'B' | 'R' => max,
        _ => 0,
      };
    }
    match c {
      'L' | 'F' => max = (max as f32 - ((max - min) as f32 / 2.0)).floor() as i32,
      'B' | 'R' => min += ((max - min) as f32 / 2.0).ceil() as i32,
      _ => (),
    }
  }
  0
}
