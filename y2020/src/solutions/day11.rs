use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::{error::Error, str::FromStr};

pub struct Solve {
  tiles: Vec<Tile>,
  width: usize,
}

type Trajectory = (i32, i32);

#[derive(Clone, Debug, PartialEq)]
enum Tile {
  Floor,
  Occupied,
  Empty,
}

impl FromStr for Tile {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "#" => Ok(Tile::Occupied),
      "L" => Ok(Tile::Empty),
      "." => Ok(Tile::Floor),
      _ => Err("Unknown tile type".to_string()),
    }
  }
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    let input = read_input(d)?;
    let width = input.lines().next().map(str::len).unwrap();
    Ok(Solve {
      tiles: input
        .lines()
        .map(|l| l.split("").map(Tile::from_str))
        .flatten()
        .flatten()
        .collect(),
      width,
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    let tile_update = update_tile(4, find_adjacent);
    Ok(run_sim(&self.tiles, self.width, &tile_update).to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    let tile_update = update_tile(5, find_first_non_floor);
    Ok(run_sim(&self.tiles, self.width, &tile_update).to_string())
  }
}

fn run_sim<F>(tiles: &Vec<Tile>, width: usize, tile_update: &F) -> usize
where
  F: Fn(&Vec<Tile>, i32, &Tile, i32) -> (Tile, bool),
{
  let mut state = tiles.clone();
  while let Some(next) = update_state(&state, width, tile_update) {
    state = next;
  }
  state.iter().filter(|f| **f == Tile::Occupied).count()
}

fn update_state<F>(state: &Vec<Tile>, width: usize, update_tile: F) -> Option<Vec<Tile>>
where
  F: Fn(&Vec<Tile>, i32, &Tile, i32) -> (Tile, bool),
{
  let (next_state, changed) = state
    .clone()
    .iter()
    .enumerate()
    .map(|(idx, t)| update_tile(state, width as i32, t, idx as i32))
    .fold((Vec::new(), false), |(mut acc, ch), (t, u)| {
      acc.push(t.clone());
      (acc, ch || u)
    });
  match changed {
    false => None,
    true => Some(next_state),
  }
}

fn update_tile<F>(
  seats: usize,
  update_trajectory: F,
) -> impl Fn(&Vec<Tile>, i32, &Tile, i32) -> (Tile, bool)
where
  F: Fn(&Vec<Tile>, i32, i32, Trajectory) -> Option<Tile>,
{
  move |state: &Vec<Tile>, width: i32, t: &Tile, idx: i32| {
    if *t == Tile::Floor {
      return (t.to_owned(), false);
    }
    let traj = [
      (0, -1),
      (0, 1),
      (-1, -1),
      (-1, 0),
      (-1, 1),
      (1, -1),
      (1, 0),
      (1, 1),
    ];
    let occupied = traj
      .iter()
      .filter_map(|f| update_trajectory(state, idx, width, *f))
      .filter(|t| *t == Tile::Occupied)
      .count();

    match (t, occupied) {
      (Tile::Occupied, _) if occupied >= seats => (Tile::Empty, true),
      (Tile::Empty, 0) => (Tile::Occupied, true),
      _ => (t.to_owned(), false),
    }
  }
}

fn adv(idx: i32, width: i32, (ty, tx): Trajectory) -> Option<i32> {
  let row = (idx / width) + ty;
  let col = (idx + tx) % width;
  let will_wrap = ((idx + tx) / width) != (idx / width);
  if will_wrap || row < 0 || col < 0 {
    return None;
  }
  Some(row * width + col)
}

fn find_adjacent(state: &Vec<Tile>, s: i32, w: i32, t: Trajectory) -> Option<Tile> {
  match adv(s, w, t) {
    None => None,
    Some(idx) => state.get(idx as usize).map(|t| t.to_owned()),
  }
}

fn find_first_non_floor(state: &Vec<Tile>, s: i32, w: i32, t: Trajectory) -> Option<Tile> {
  let mut s = s;
  while let Some(idx) = adv(s, w, t) {
    match state.get(idx as usize) {
      Some(Tile::Floor) => s = idx,
      None => return None,
      Some(t) => return Some(t.to_owned()),
    }
  }
  None
}
