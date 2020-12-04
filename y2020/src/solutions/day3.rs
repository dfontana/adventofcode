use crate::day::{Day, DayArg};
use crate::util::read_input;

use std::error::Error;

const TREE: u8 = b'#';

pub struct Day3 {
  input: String,
}

impl Day for Day3 {
  fn new() -> Result<Day3, Box<dyn Error>> {
    Ok(Day3 {
      input: read_input(DayArg::D(3))?,
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    Ok(trees_hit(&self, 3, 1).to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok(
      [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(right, down)| trees_hit(self, *right, *down))
        .product::<usize>()
        .to_string(),
    )
  }
}

// Step by to skip N lines at at ime
// Enumerate to iterate by (Step Count we're on, the line we're at)
// Y coord = Step Count, X Coord = StepCount * RightStep; Modulo to wrap around
fn trees_hit(day: &Day3, right: usize, down: usize) -> usize {
  day
    .input
    .lines()
    .step_by(down)
    .enumerate()
    .filter(|(step, l)| l.as_bytes()[(*step * right) % l.len()] == TREE)
    .count()
}
