use itertools::Itertools;
use rust_util::Day;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct Solve {
  input: Vec<(usize, usize)>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let len = value.lines().next().unwrap().len();

    let mut expanded: Vec<Vec<char>> = Vec::new();

    // Find the blanks and init our expanded vector
    let mut blank_cols = vec![true; len];
    for line in value.lines() {
      expanded.push(line.chars().collect_vec());
      let galaxy_idxs = line
        .chars()
        .enumerate()
        .filter_map(|(idx, char)| if char == '#' { Some(idx) } else { None })
        .collect_vec();
      if galaxy_idxs.is_empty() {
        expanded.push(vec!['.'; len]);
      }
      for idx in galaxy_idxs {
        blank_cols[idx] = false;
      }
    }

    // expand the columns now
    for row in expanded.iter_mut() {
      for (idx, _) in blank_cols.iter().enumerate().filter(|v| *v.1).rev() {
        // Reverse so we don't shift the original idx's wrongly
        row.insert(idx, '.');
      }
    }

    // Now scan for galaxies and get their coords as our input
    let input = expanded
      .iter()
      .enumerate()
      .flat_map(|(y, row)| {
        row
          .iter()
          .enumerate()
          .filter_map(|(x, char)| if *char == '#' { Some((y, x)) } else { None })
          .collect_vec()
      })
      .collect_vec();

    Ok(Solve { input })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut total: usize = 0;
    for i in 0..self.input.len() {
      let (ay, ax) = self.input[i];
      for j in i..self.input.len() {
        let (by, bx) = self.input[j];
        total += ax.abs_diff(bx) + ay.abs_diff(by);
      }
    }
    Ok(Box::new(total))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(1))
  }
}
