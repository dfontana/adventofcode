use rust_util::Day;
use std::{collections::HashSet, error::Error, fmt::Display};

pub struct Solve {
  input: String,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve { input: value })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let bytes = self.input.as_bytes();
    match find_offset(bytes, 4) {
      Some(v) => Ok(Box::new(v)),
      None => Err("No solution found".into()),
    }
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let bytes = self.input.as_bytes();
    match find_offset(bytes, 14) {
      Some(v) => Ok(Box::new(v)),
      None => Err("No solution found".into()),
    }
  }
}

fn find_offset(bytes: &[u8], count: usize) -> Option<usize> {
  let mut unique = HashSet::new();
  for i in 0..bytes.len() {
    unique.clear();
    for k in 0..count {
      unique.insert(bytes[i + k]);
    }
    if unique.len() == count {
      return Some(i + count);
    }
  }
  None
}
