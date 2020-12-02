extern crate dotenv;

mod util;
mod day;
mod solutions;

use day::{Day,DayArg};
use solutions::{Day1, Day2};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  dotenv::dotenv().ok();
  let day = std::env::args()
    .nth(1)
    .and_then(|v| v.parse::<i32>().ok())
    .ok_or("No Day given to run")?;
  let runner = get_runner(DayArg::D(day))?;
  print!("\n{}\n", runner.run()?);
  Ok(())
}

fn get_runner(day: DayArg) -> Result<Box<dyn Day>, Box<dyn Error>> {
  match day {
    DayArg::D(1) => Day1::new().map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(2) => Day2::new().map(|s| Box::new(s) as Box<dyn Day>),
    DayArg::D(n) => Err(format!("Unknown Day Given: {}", n).into()),
  }
}