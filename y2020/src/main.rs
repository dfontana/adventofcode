#[macro_use]
extern crate lazy_static;
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
    AocDay::D(02) => Day2::new(day),
    AocDay::D(03) => Day3::new(day),
    AocDay::D(04) => Day4::new(day),
    AocDay::D(05) => Day5::new(day),
    AocDay::D(06) => Day6::new(day),
    AocDay::D(07) => Day7::new(day),
    AocDay::D(08) => Day8::new(day),
    AocDay::D(09) => Day9::new(day),
    AocDay::D(10) => Day10::new(day),
    AocDay::D(11) => Day11::new(day),
    AocDay::D(12) => Day12::new(day),
    AocDay::D(13) => Day13::new(day),
    AocDay::D(14) => Day14::new(day),
    AocDay::D(15) => Day15::new(day),
    AocDay::D(16) => Day16::new(day),
    AocDay::D(17) => Day17::new(day),
    AocDay::D(18) => Day18::new(day),
    AocDay::D(19) => Day19::new(day),
    AocDay::D(20) => Day20::new(day),
    _ => Err("Unknown day given".into()),
  }
}
