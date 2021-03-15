mod day1;

use crate::day::{Day, DayArg};

use std::error::Error;

pub fn get_runner(d: DayArg) -> Result<Box<dyn Day>, Box<dyn Error>> {
  match d {
    DayArg::D(1) => day1::Solve::new(d).map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(n) => Err(format!("Unknown Day Given: {}", n).into()),
  }
}
