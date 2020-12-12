use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::{error::Error, fmt::Display, str::FromStr};

pub struct Solve {
  tiles: Vec<Tile>,
  width: usize,
}

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

impl Display for Tile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let ch = match self {
      Tile::Floor => '.',
      Tile::Occupied => '#',
      Tile::Empty => 'L',
    };
    write!(f, "{}", ch)
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
    let mut state = self.tiles.clone();
    while let Some(next) = update_state(&state, self.width) {
      state = next;
    }
    Ok(
      state
        .iter()
        .filter(|f| **f == Tile::Occupied)
        .count()
        .to_string(),
    )
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    let mut state = self.tiles.clone();
    // print_state(&state, self.width);
    while let Some(next) = update_state2(&state, self.width) {
      // print_state(&next, self.width);
      state = next;
    }
    Ok(
      state
        .iter()
        .filter(|f| **f == Tile::Occupied)
        .count()
        .to_string(),
    )
  }
}

fn update_state(state: &Vec<Tile>, width: usize) -> Option<Vec<Tile>> {
  let (next_state, changed) = state
    .clone()
    .iter()
    .enumerate()
    .map(|(idx, t)| update_tile(idx as i32, t, width as i32, state))
    .fold((Vec::new(), false), |(mut acc, ch), (t, u)| {
      acc.push(t.clone());
      (acc, ch || u)
    });
  match changed {
    false => None,
    true => Some(next_state),
  }
}

fn update_tile(idx: i32, t: &Tile, width: i32, state: &Vec<Tile>) -> (Tile, bool) {
  if *t == Tile::Floor {
    return (t.to_owned(), false);
  }
  fn wrap(idx: i32, width: i32, row_adj: i32, col_adj: i32) -> i32 {
    let row = (idx / width) + row_adj;
    let col = (idx + col_adj) % width;
    let will_wrap = ((idx + col_adj) / width) != (idx / width);
    if will_wrap {
      return -100;
    }
    if row < 0 || col < 0 {
      -100
    } else {
      row * width + col
    }
  }
  let occupied = [
    wrap(idx, width, 0, -1),
    wrap(idx, width, 0, 1),
    wrap(idx, width, -1, -1),
    wrap(idx, width, -1, 0),
    wrap(idx, width, -1, 1),
    wrap(idx, width, 1, -1),
    wrap(idx, width, 1, 0),
    wrap(idx, width, 1, 1),
  ]
  .iter()
  .filter_map(|f| {
    if *f < 0 || *f == idx {
      None
    } else {
      Some(*f as usize)
    }
  })
  .filter_map(|f| state.get(f))
  .filter(|t| **t == Tile::Occupied)
  .count();

  match t {
    Tile::Occupied => {
      if occupied >= 4 {
        return (Tile::Empty, true);
      }
      (t.to_owned(), false)
    }
    Tile::Empty => {
      if occupied == 0 {
        return (Tile::Occupied, true);
      }
      (t.to_owned(), false)
    }
    _ => (t.to_owned(), false),
  }
}

fn update_state2(state: &Vec<Tile>, width: usize) -> Option<Vec<Tile>> {
  let (next_state, changed) = state
    .clone()
    .iter()
    .enumerate()
    .map(|(idx, t)| update_tile2(idx as i32, t, width as i32, state))
    .fold((Vec::new(), false), |(mut acc, ch), (t, u)| {
      acc.push(t.clone());
      (acc, ch || u)
    });
  match changed {
    false => None,
    true => Some(next_state),
  }
}

fn update_tile2(idx: i32, t: &Tile, width: i32, state: &Vec<Tile>) -> (Tile, bool) {
  if *t == Tile::Floor {
    return (t.to_owned(), false);
  }
  fn adv(idx: i32, width: i32, row_adj: i32, col_adj: i32) -> Option<i32> {
    let row = (idx / width) + row_adj;
    let col = (idx + col_adj) % width;
    let will_wrap = ((idx + col_adj) / width) != (idx / width);
    if will_wrap || row < 0 || col < 0 {
      return None;
    }
    Some(row * width + col)
  }

  fn find_first_non_floor(state: &Vec<Tile>, s: i32, w: i32, (ty, tx): (i32,i32)) -> Option<Tile> {
    let mut s = s;
    while let Some(idx) = adv(s, w, ty, tx) {
      let tile = match state.get(idx as usize) {
        None => return None,
        Some(t) => t,
      };
      if *tile != Tile::Floor {
        return Some(tile.to_owned());
      }
      s = idx;
    }
    None
  }

  let trag = [(0, -1),(0, 1),(-1, -1),(-1, 0),(-1, 1),(1, -1),(1, 0),(1, 1)];
  let occupied = trag
  .iter()
  .filter_map(|f| find_first_non_floor(state, idx, width, *f))
  .filter(|t| *t == Tile::Occupied)
  .count();

  match t {
    Tile::Occupied => {
      if occupied >= 5 { // <--- Bumped to 5 here
        return (Tile::Empty, true);
      }
      (t.to_owned(), false)
    }
    Tile::Empty => {
      if occupied == 0 {
        return (Tile::Occupied, true);
      }
      (t.to_owned(), false)
    }
    _ => (t.to_owned(), false),
  }
}


fn print_state(state: &Vec<Tile>, width: usize) {
  println!("{:-<10}", "-");
  state
    .chunks(width)
    .map(|chunk| {
      chunk
        .iter()
        .map(Tile::to_string)
        .fold("".to_string(), |acc, f| acc + &f)
    })
    .for_each(|chunk| println!("{}", chunk));
}
