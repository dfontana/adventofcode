use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::error::Error;

struct Tile {
  id: usize,
  img: Vec<String>,
}

pub struct Solve {
  tiles: Vec<Tile>,
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    Ok(Solve {
      tiles: read_input(d)?
        .split("\n\n")
        .map(|t| {
          let mut lines = t.lines();
          let id: usize = lines
            .next()
            .unwrap()
            .trim_start_matches("Tile ")
            .trim_end_matches(':')
            .parse()
            .unwrap();
          let img = lines.map(str::to_owned).collect();
          Tile { id, img }
        })
        .collect(),
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    Ok("Impl".to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok("Impl".to_string())
  }
}
