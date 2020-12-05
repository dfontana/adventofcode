use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::{error::Error, ops::RangeInclusive};

pub struct Solve {
  seats: Vec<Seat>,
}
struct Seat {
  row_loc: String,
  col_loc: String,
  id: i32,
}

impl Seat {
  pub fn new(str_split: (&str, &str)) -> Seat {
    Seat {
      row_loc: str_split.0.to_string(),
      col_loc: str_split.1.to_string(),
      id: 0,
    }
  }

  pub fn compute(self) -> Seat {
    Seat {
      row_loc: self.row_loc.clone(),
      col_loc: self.col_loc.clone(),
      id: search(127, 'F', 'B', 6, &self.row_loc) * 8 + search(7, 'L', 'R', 2, &self.col_loc),
    }
  }

  pub fn id(&self) -> i32 {
    self.id
  }
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    let mut seats = read_input(d)?
      .lines()
      .map(|s| (&s[..7], &s[7..]))
      .map(Seat::new)
      .map(Seat::compute)
      .collect::<Vec<Seat>>();
    seats.sort_by_key(Seat::id);
    Ok(Solve { seats })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    self
      .seats
      .last()
      .map(|s| s.id.to_string())
      .ok_or("No Seats Parsed".into())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    self
      .seats
      .windows(3)
      .filter(|win| win[0].id + 1 != win[1].id || win[1].id + 1 != win[2].id)
      .map(|win| win[1].id + 1)
      .next()
      .map(|s| s.to_string())
      .ok_or("No Missing Seat Found".into())
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
