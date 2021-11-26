use rust_util::{AocDay, Day};
use std::{error::Error, fmt::Display};

pub struct Solve {}

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    let input = rust_util::read_input(2021, d)?;
    Ok(Box::new(Solve {}))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    todo!()
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    todo!()
  }
}
