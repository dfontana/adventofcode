use itertools::Itertools;
use rust_util::Day;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct Solve {
  gp1: Vec<(usize, usize)>,
  gp2: Vec<(usize, usize)>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let len = value.lines().next().unwrap().len();

    let mut gp1: Vec<(usize, usize)> = Vec::new();
    let mut gp2: Vec<(usize, usize)> = Vec::new();

    let mut blank_cols = vec![true; len];
    let mut y_basis_p1 = 0;
    let mut y_basis_p2 = 0;
    for (y, line) in value.lines().enumerate() {
      let galaxy_idxs = line
        .chars()
        .enumerate()
        .filter_map(|(idx, char)| if char == '#' { Some(idx) } else { None })
        .collect_vec();
      if galaxy_idxs.is_empty() {
        y_basis_p1 += 1;
        y_basis_p2 += 1_000_000;
      } else {
        for x in galaxy_idxs {
          blank_cols[x] = false;
          gp1.push((y + y_basis_p1, x));
          gp2.push((y + y_basis_p2, x));
        }
      }
    }

    let x_to_expand =  blank_cols.iter().enumerate().filter_map(|v| if *v.1 { Some(v.0)} else {None}).collect_vec();

    for g in gp1.iter_mut() {
      let num_expands_before = x_to_expand.iter().filter(|v| **v < g.1).collect_vec().len();
      g.1 +=1*num_expands_before;
    }

    for g in gp2.iter_mut() {
      let num_expands_before = x_to_expand.iter().filter(|v| **v < g.1).collect_vec().len();
      g.1 +=1_000_000*num_expands_before;
    }
    Ok(Solve { gp1, gp2 })
  }
}

impl Solve {
  fn m_dist(&self, input: &Vec<(usize, usize)>) -> usize {
   let mut total: usize = 0;
    for i in 0..input.len() {
      let (ay, ax) = input[i];
      for j in i..input.len() {
        let (by, bx) = input[j];
        total += ax.abs_diff(bx) + ay.abs_diff(by);
      }
    }
    total
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(self.m_dist(&self.gp1)))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(self.m_dist(&self.gp2)))
  }
}
