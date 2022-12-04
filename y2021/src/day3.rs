use rust_util::{Day};
use std::{error::Error, fmt::Display};

pub struct Solve {
  input: Vec<u32>,
  bit_len: usize,
  mask_base: u32,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(input: String) -> Result<Self, Self::Error> {
    let bit_len = input.lines().next().unwrap().len();
    Ok(Solve {
      input: input
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect(),
      bit_len,
      mask_base: 1 << bit_len - 1,
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let majority = majority(&self.input);
    let gamma = (0..self.bit_len)
      .map(|shift| self.mask_base >> shift)
      .fold(0, |gamma, mask| {
        if count_ones(&self.input, mask) > majority {
          gamma ^ mask
        } else {
          gamma
        }
      });
    Ok(Box::new(gamma * (!gamma & (1 << self.bit_len) - 1)))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let oxygen = comp(true, self.input.clone(), self.bit_len, self.mask_base);
    let co2 = comp(false, self.input.clone(), self.bit_len, self.mask_base);
    Ok(Box::new(oxygen * co2))
  }
}

fn majority(inp: &Vec<u32>) -> usize {
  (inp.len() as f32 / 2.0).ceil() as usize
}

fn count_ones(inp: &Vec<u32>, mask: u32) -> usize {
  inp.iter().filter(|power| *power & mask > 0).count()
}

fn comp(high_wins: bool, inp: Vec<u32>, bit_len: usize, base: u32) -> u32 {
  let mut rem_view = inp;
  for pos in 0..bit_len {
    let mask = base >> pos;
    let is_majority = count_ones(&rem_view, mask) >= majority(&rem_view);

    rem_view = rem_view
      .iter()
      .filter(|power| {
        let pos_val = *power & mask;
        match (high_wins, is_majority) {
          (true, true) => pos_val > 0,
          (true, false) => pos_val == 0,
          (false, true) => pos_val == 0,
          (false, false) => pos_val > 0,
        }
      })
      .map(|pwr| *pwr)
      .collect();

    if rem_view.len() == 1 {
      return *rem_view.get(0).unwrap();
    }
  }
  0
}
