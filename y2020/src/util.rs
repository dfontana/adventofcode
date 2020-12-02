use std::fs;
use std::path::PathBuf;
use std::error::Error;
use crate::day::{DayArg, Part};

pub fn read_input(day: DayArg, part: Part) -> Result<String, Box<dyn Error>> {
  let path = match day {
    DayArg::D(v @ 1..=25) => PathBuf::from(format!("./input/d{}/p{}", v, part)),
    DayArg::D(_) => return Err("Unknown Day Provided".into())
  };
  match fs::read_to_string(path) {
    Err(err) => Err(err.into()),
    Ok(s) => Ok(s),
  }
}