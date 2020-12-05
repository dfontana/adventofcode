#[macro_use]
extern crate lazy_static;

extern crate dotenv;

mod day;
mod solutions;
mod util;

use day::DayArg;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  dotenv::dotenv().ok();
  let day = std::env::args()
    .nth(1)
    .and_then(|v| v.parse::<i32>().ok())
    .ok_or("No Day given to run")?;
  let runner = solutions::get_runner(DayArg::D(day))?;
  print!("\n{}\n", runner.run()?);
  Ok(())
}
