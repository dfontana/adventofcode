pub mod grid;
pub mod search;

use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::{env, fmt::Display};

use reqwest::{blocking::Client, header};

pub enum AocDay {
  D(usize, usize),
}

impl Display for AocDay {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      AocDay::D(year, day) => write!(f, "{}/day/{}", year, day),
    }
  }
}

pub trait Day: TryFrom<String, Error = Box<dyn std::error::Error>> {
  fn new(d: AocDay) -> Result<Self, Box<dyn Error>> {
    Self::try_from(read_input(d)?)
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>>;
  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>>;

  fn run(&self) -> Result<String, Box<dyn Error>> {
    let mut ans = match self.p1() {
      Ok(v) => format!("Part 1: {}", v),
      Err(e) => format!("Part 1: {:?}", e),
    };
    ans = match self.p2() {
      Ok(v) => format!("{}\nPart 2: {}", ans, v),
      Err(e) => format!("{}\nPart 2: {:?}", ans, e),
    };
    Ok(ans)
  }
}

fn read_input(day: AocDay) -> Result<String, Box<dyn Error>> {
  let path = match day {
    AocDay::D(_, v @ 1..=25) => PathBuf::from(format!("./input/d{}", v)),
    AocDay::D(_, _) => return Err("Unknown Day Provided".into()),
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

fn download_input(day: AocDay, out: &PathBuf) -> Result<(), Box<dyn Error>> {
  let session = env::var("AOC_SESSION")?;
  let dl_url = format!("https://adventofcode.com/{}/input", day);
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
