use rust_util::{ Day};
use std::{
  collections::{HashMap, HashSet},
  error::Error,
  fmt::Display,
  str::FromStr,
};

#[derive(Debug, Clone, PartialEq)]
enum Cell {
  ACTIVE,
  INACTIVE,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Coord {
  x: i32,
  y: i32,
  z: i32,
  w: i32,
}

impl Coord {
  pub fn new(x: i32, y: i32, z: i32) -> Coord {
    Coord { x, y, z, w: 0 }
  }

  pub fn new4d(x: i32, y: i32, z: i32, w: i32) -> Coord {
    Coord { x, y, z, w }
  }
}

impl FromStr for Cell {
  type Err = String;
  fn from_str(inp: &str) -> std::result::Result<Self, Self::Err> {
    Ok(match inp {
      "#" => Cell::ACTIVE,
      "." => Cell::INACTIVE,
      _ => unreachable!(),
    })
  }
}

pub struct Solve {
  state: HashMap<Coord, Cell>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      state:value 
        .lines()
        .enumerate()
        .map(|(y, l)| {
          l.chars().enumerate().filter_map(move |(x, c)| {
            match Cell::from_str(&c.to_string()).unwrap() {
              Cell::ACTIVE => Some((x as i32, y as i32, Cell::ACTIVE)),
              _ => None,
            }
          })
        })
        .flatten()
        .fold(HashMap::new(), |mut acc, (x, y, cell)| {
          acc.insert(Coord::new(x, y, 0), cell);
          acc
        }),
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut state = self.state.clone();
    for _ in 0..6 {
      state = update_state(&state, expand_3d);
    }
    Ok(Box::new(
      state
        .values()
        .filter(|c| **c == Cell::ACTIVE)
        .count()
        .to_string(),
    ))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut state = self.state.clone();
    for _ in 0..6 {
      state = update_state(&state, expand_4d);
    }
    Ok(Box::new(
      state
        .values()
        .filter(|c| **c == Cell::ACTIVE)
        .count()
        .to_string(),
    ))
  }
}

fn update_state<F>(state: &HashMap<Coord, Cell>, expand: F) -> HashMap<Coord, Cell>
where
  F: Fn(&Coord) -> Vec<Coord>,
{
  let items_to_check: HashSet<Coord> = state
    .keys()
    .map(|coor| {
      let mut nbs: Vec<Coord> = expand(coor);
      nbs.push(coor.clone());
      nbs
    })
    .flatten()
    .collect();

  items_to_check
    .iter()
    .filter_map(|nc| map_cell(&expand, state, nc))
    .fold(HashMap::new(), |mut acc, (coor, cell)| {
      acc.insert(coor, cell);
      acc
    })
}

fn map_cell<F>(expand: F, state: &HashMap<Coord, Cell>, coor: &Coord) -> Option<(Coord, Cell)>
where
  F: Fn(&Coord) -> Vec<Coord>,
{
  let nbs: Vec<Coord> = expand(coor);
  let cell = state.get(&coor).unwrap_or(&Cell::INACTIVE);
  let count = nbs
    .iter()
    .filter(|c| state.get(*c).filter(|ce| **ce == Cell::ACTIVE).is_some())
    .count();
  match count {
    2 | 3 if *cell == Cell::ACTIVE => Some((coor.clone(), Cell::ACTIVE)),
    3 if *cell == Cell::INACTIVE => Some((coor.clone(), Cell::ACTIVE)),
    _ => None,
  }
}

fn expand_3d(coor: &Coord) -> Vec<Coord> {
  let mut nbs = Vec::new();
  for x in coor.x - 1..coor.x + 2 {
    for y in coor.y - 1..coor.y + 2 {
      for z in coor.z - 1..coor.z + 2 {
        let c = Coord::new(x, y, z);
        if c == *coor {
          continue;
        }
        nbs.push(c);
      }
    }
  }
  nbs
}

fn expand_4d(coor: &Coord) -> Vec<Coord> {
  let mut nbs = Vec::new();
  for x in coor.x - 1..coor.x + 2 {
    for y in coor.y - 1..coor.y + 2 {
      for z in coor.z - 1..coor.z + 2 {
        for w in coor.w - 1..coor.w + 2 {
          let c = Coord::new4d(x, y, z, w);
          if c == *coor {
            continue;
          }
          nbs.push(c);
        }
      }
    }
  }
  nbs
}
