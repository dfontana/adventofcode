use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::{error::Error, ops::RangeInclusive, slice::Iter};

pub struct Solve {
  lines: Vec<String>,
}
struct Seat {
  row_loc: String,
  col_loc: String,
  row: i32,
  col: i32,
  id: i32,
}

impl Seat {
  pub fn new(str_split: (&str, &str)) -> Seat {
    Seat {
      row_loc: str_split.0.to_string(),
      col_loc: str_split.1.to_string(),
      row: 0,
      col: 0,
      id: 0,
    }
  }

  pub fn compute(self) -> Seat {
    let row = search(127, 'F', 'B', 6, &self.row_loc);
    let col = search(7, 'L', 'R', 2, &self.col_loc);
    Seat {
      row_loc: self.row_loc,
      col_loc: self.col_loc,
      row,
      col,
      id: row * 8 + col,
    }
  }

  pub fn id(&self) -> i32 {
    self.id
  }
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    Ok(Solve {
      lines: read_input(d)?.lines().map(str::to_string).collect(),
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    let mut seats = self
      .lines
      .iter()
      .map(|s| (&s[..7], &s[7..]))
      .map(Seat::new)
      .map(Seat::compute)
      .collect::<Vec<Seat>>();
    seats.sort_by_key(Seat::id);
    seats
      .last()
      .map(|s| s.id.to_string())
      .ok_or("No Seats Parsed".into())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok("Impl".to_string())
  }
}

fn search(max_bound: i32, lower: char, upper: char, stop: usize, loc: &str) -> i32 {
  let mut min = 0;
  let mut max = max_bound;
  for (idx, c) in loc.chars().enumerate() {
    if idx == stop {
      return match c {
        c if c == lower => min,
        c if c == upper => max,
        _ => 0,
      };
    }

    match c {
      c if c == lower => max = (max as f32 - ((max - min) as f32 / 2.0)).floor() as i32,
      c if c == upper => min += ((max - min) as f32 / 2.0).ceil() as i32,
      _ => (),
    }
  }
  0
}
