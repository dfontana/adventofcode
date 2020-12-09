mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use crate::day::{Day, DayArg};

use std::error::Error;

pub fn get_runner(d: DayArg) -> Result<Box<dyn Day>, Box<dyn Error>> {
  match d {
    DayArg::D(1) => day1::Solve::new(d).map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(2) => day2::Solve::new(d).map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(3) => day3::Solve::new(d).map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(4) => day4::Solve::new(d).map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(5) => day5::Solve::new(d).map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(6) => day6::Solve::new(d).map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(7) => day7::Solve::new(d).map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(8) => day8::Solve::new(d).map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(n) => Err(format!("Unknown Day Given: {}", n).into()),
  }
}
