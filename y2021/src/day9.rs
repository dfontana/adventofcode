use itertools::Itertools;
use rust_util::{Day};
use std::{error::Error, fmt::Display};

const NEIGHBORS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

pub struct Solve {
  field: Vec<Vec<u32>>,
  lows: Vec<(usize, usize)>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let field = value 
      .lines()
      .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
      .collect();
    let lows = get_low_idxs(&field);
    Ok(Solve { field, lows })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let sum: u32 = self
      .lows
      .iter()
      .map(|(x, y)| {
        self
          .field
          .get(*y)
          .and_then(|r| r.get(*x))
          .map(|v| v + 1)
          .unwrap()
      })
      .sum();
    Ok(Box::new(sum))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut field = self.field.clone();
    Ok(Box::new(
      self
        .lows
        .iter()
        .map(|point| basin_size(&mut field, *point))
        .sorted()
        .rev()
        .take(3)
        .product::<usize>(),
    ))
  }
}

fn get_low_idxs(field: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
  field
    .iter()
    .enumerate()
    .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, v)| (y, x, *v)))
    .filter(|(y, x, val)| {
      NEIGHBORS.iter().all(|(xoff, yoff)| {
        field
          // Casting to usize will make it MAX val, which then DNE, so None.
          .get(y.overflowing_add(*yoff as usize).0)
          .and_then(|l| l.get(x.overflowing_add(*xoff as usize).0))
          .map(|n| val < n)
          .unwrap_or(true)
      })
    })
    .map(|(y, x, _)| (x, y))
    .collect()
}

fn basin_size(field: &mut Vec<Vec<u32>>, (x, y): (usize, usize)) -> usize {
  // Set current value to 9 (explored) & then explore all non-9 neighbors
  // Recursive is more efficient than imperitive, since less allocs
  field.get_mut(y).and_then(|l| l.get_mut(x)).map(|v| *v = 9);
  NEIGHBORS.iter().fold(1, |acc, (xoff, yoff)| {
    let xp = x.overflowing_add(*xoff as usize).0;
    let yp = y.overflowing_add(*yoff as usize).0;
    match field.get(yp).and_then(|l| l.get(xp)).map(|n| *n < 9) {
      Some(true) => acc + basin_size(field, (xp, yp)),
      _ => acc,
    }
  })
}
