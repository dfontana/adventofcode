mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
use {day1::Day1, day2::Day2, day3::Day3, day4::Day4, day5::Day5};

use crate::day::{Day, DayArg};

use std::error::Error;

pub fn get_runner(day: DayArg) -> Result<Box<dyn Day>, Box<dyn Error>> {
  match day {
    DayArg::D(1) => Day1::new().map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(2) => Day2::new().map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(3) => Day3::new().map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(4) => Day4::new().map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(5) => Day5::new().map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(n) => Err(format!("Unknown Day Given: {}", n).into()),
  }
}
