use itertools::Itertools;
use rust_util::Day;
use std::{error::Error, fmt::Display, str::FromStr};

pub struct Solve {
  input: Vec<Game>,
}

#[derive(Debug)]
struct Game {
  id: usize,
  sets: Vec<TileSet>,
}
#[derive(Debug)]
struct TileSet {
  green: usize,
  blue: usize,
  red: usize,
}
impl FromStr for Game {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if let Some((game, tsets)) = s.split_once(": ") {
      let id = game
        .strip_prefix("Game ")
        .map(|id| id.parse::<usize>().expect(&format!("NaN id: {}", id)))
        .unwrap();
      let sets = tsets
        .split("; ")
        .map(|set| {
          let mut green = 0;
          let mut blue = 0;
          let mut red = 0;
          set.split(", ").for_each(|s| {
            let (cnts, typ) = s.split_once(" ").expect(&format!("Not a Tile: {}", s));
            let cnt = cnts.parse::<usize>().expect(&format!("NaN cnt: {}", cnts));
            match typ {
              "green" => {
                green = cnt;
              }
              "blue" => {
                blue = cnt;
              }
              "red" => {
                red = cnt;
              }
              _ => unreachable!("Unknown type hit: {}", typ),
            }
          });
          TileSet { green, blue, red }
        })
        .collect_vec();
      Ok(Game { id, sets })
    } else {
      Err("Failed parsing".to_string())
    }
  }
}

impl Game {
  fn is_possible(&self, t: TileSet) -> bool {
    self
      .sets
      .iter()
      .all(|ts| ts.green <= t.green && ts.blue <= t.blue && ts.red <= t.red)
  }

  fn min_req(&self) -> TileSet {
    self.sets.iter().fold(
      TileSet {
        green: 0,
        blue: 0,
        red: 0,
      },
      TileSet::max_of,
    )
  }
}

impl TileSet {
  fn power(self) -> usize {
    self.red * self.green * self.blue
  }

  fn max_of(a: TileSet, b: &TileSet) -> TileSet {
    TileSet {
      green: a.green.max(b.green),
      blue: a.blue.max(b.blue),
      red: a.red.max(b.red),
    }
  }
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      input: value
        .lines()
        .map(Game::from_str)
        .map(|s| s.unwrap())
        .collect_vec(),
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .input
        .iter()
        .filter(|g| {
          g.is_possible(TileSet {
            green: 13,
            blue: 14,
            red: 12,
          })
        })
        .map(|g| g.id)
        .sum::<usize>(),
    ))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .input
        .iter()
        .map(Game::min_req)
        .map(TileSet::power)
        .sum::<usize>(),
    ))
  }
}
