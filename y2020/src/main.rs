mod util;
mod day;
mod solutions;

use day::{Day,DayArg};
use solutions::Day1;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let day = std::env::args()
    .nth(1)
    .and_then(|v| v.parse::<i32>().ok())
    .ok_or("No Day given to run")?;
  let runner = match DayArg::D(day) {
    DayArg::D(1) => Day1::new(),
    DayArg::D(n) => Err(format!("Unknown Day Given: {}", n).into()),
  }?;
  print!("\n{}\n", runner.run()?);
  Ok(())
}