extern crate dotenv;

mod day;
mod solutions;
mod util;

use day::DayArg;
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
  dotenv::dotenv().ok();
  let day = std::env::args()
    .nth(1)
    .and_then(|v| v.parse::<i32>().ok())
    .ok_or("No Day given to run")?;
  let runner = solutions::get_runner(DayArg::D(day))?;

  let now = Instant::now();
  print!("\n{}\n", runner.run()?);
  let elapsed = now.elapsed();
  println!("Elapsed: {:.2?}", elapsed);
  Ok(())
}
