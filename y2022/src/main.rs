extern crate dotenv;
extern crate rust_util;

mod solutions;

use solutions::*;

use rust_util::{AocDay, Day};
use std::{error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
  dotenv::dotenv().ok();
  let day = std::env::args()
    .nth(1)
    .and_then(|v| v.parse::<usize>().ok())
    .ok_or("No Day given to run")?;

  let now = Instant::now();
  print!("\n{}\n", get_runner(AocDay::D(day))?.run()?);
  let elapsed = now.elapsed();
  println!("Elapsed: {:.2?}", elapsed);
  Ok(())
}

fn get_runner(day: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>> {
  match day {
    AocDay::D(01) => Day1::new(day),
    _ => Err("Unknown day given".into()),
  }
}
