use rust_util::{Day};

use std::{error::Error, fmt::Display};

const TREE: u8 = b'#';

pub struct Solve {
  input: String,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      input: value,
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(trees_hit(&self, 3, 1).to_string()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(right, down)| trees_hit(self, *right, *down))
        .product::<usize>()
        .to_string(),
    ))
  }
}

// Step by to skip N lines at at ime
// Enumerate to iterate by (Step Count we're on, the line we're at)
// Y coord = Step Count, X Coord = StepCount * RightStep; Modulo to wrap around
fn trees_hit(day: &Solve, right: usize, down: usize) -> usize {
  day
    .input
    .lines()
    .step_by(down)
    .enumerate()
    .filter(|(step, l)| l.as_bytes()[(*step * right) % l.len()] == TREE)
    .count()
}
