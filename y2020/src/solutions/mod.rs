mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
    DayArg::D(9) => day9::Solve::new(d).map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(10) => day10::Solve::new(d).map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(11) => day11::Solve::new(d).map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(12) => day12::Solve::new(d).map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(13) => day13::Solve::new(d).map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(14) => day14::Solve::new(d).map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(15) => day15::Solve::new(d).map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(16) => day16::Solve::new(d).map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(n) => Err(format!("Unknown Day Given: {}", n).into()),
  }
}
