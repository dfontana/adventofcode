mod day1;
mod day2;
use {day1::Day1, day2::Day2};

use crate::day::{Day, DayArg};

use std::error::Error;

pub fn get_runner(day: DayArg) -> Result<Box<dyn Day>, Box<dyn Error>> {
  match day {
    DayArg::D(1) => Day1::new().map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(2) => Day2::new().map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(n) => Err(format!("Unknown Day Given: {}", n).into()),
  }
}
