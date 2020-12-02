use std::fs;
use std::path::PathBuf;
use std::error::Error;
use std::env;
use crate::day::DayArg;

use reqwest::{header, {blocking::Client}};

pub fn read_input(day: DayArg) -> Result<String, Box<dyn Error>> {
  let path = match day {
    DayArg::D(v @ 1..=25) => PathBuf::from(format!("./input/d{}", v)),
    DayArg::D(_) => return Err("Unknown Day Provided".into())
  };

  if !path.exists() {
    println!("Attempting input download...");
    download_input(day, &path)?;
  }

  match fs::read_to_string(path) {
    Err(err) => Err(err.into()),
    Ok(s) => Ok(s),
  }
}

fn download_input(day: DayArg, out: &PathBuf) -> Result<(), Box<dyn Error>> {
  let session = env::var("AOC_SESSION")?;
  let dl_url = format!("https://adventofcode.com/2020/day/{}/input", day);
  let client = Client::builder().gzip(true).build()?;
  let mut response = client
    .get(&dl_url)
    .header(header::COOKIE, format!("session={}", session))
    .send()?
    .error_for_status()?;
  if let Some(parent) = out.parent() {
    fs::create_dir_all(parent)?;
  }
  let mut file = std::fs::File::create(out)?;
  response.copy_to(&mut file)?;
  Ok(())
}