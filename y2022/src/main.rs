extern crate dotenv;
extern crate rust_util;

mod day1;
mod day2;
mod day3;
mod day4;

use rust_util::{AocDay, Day};
use std::{error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
  dotenv::dotenv().ok();
  let day = std::env::args()
    .nth(1)
    .and_then(|v| v.parse::<usize>().ok())
    .ok_or("No Day given to run")?;

  let now = Instant::now();
  print!("\n{}\n", run(AocDay::D(2022, day))?);
  let elapsed = now.elapsed();
  println!("Elapsed: {:.2?}", elapsed);
  Ok(())
}

fn run(day: AocDay) -> Result<String, Box<dyn Error>> {
  match day {
    AocDay::D(_, 1) => day1::Solve::new(day)?.run(),
    AocDay::D(_, 2) => day2::Solve::new(day)?.run(),
    AocDay::D(_, 3) => day3::Solve::new(day)?.run(),
    AocDay::D(_, 4) => day4::Solve::new(day)?.run(),
    _ => Err("Unknown day given".into()),
  }
}
