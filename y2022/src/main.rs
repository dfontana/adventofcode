extern crate dotenv;
extern crate rust_util;

use rust_util::{AocDay, Day};
use rust_util_macro::import_aoc_solutions;
use std::{error::Error, time::Instant};

import_aoc_solutions!();

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
