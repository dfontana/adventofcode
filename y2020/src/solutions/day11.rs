use std::time::Duration;
use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::{error::Error, fmt::Display, str::FromStr, thread};

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
    print_state(&state, self.width);
    while let Some(next) = update_state(&state, self.width) {
      thread::sleep(Duration::from_millis(200));
      print_state(&next, self.width);
      state = next;
      break;
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
    Ok("Impl".to_string())
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
  print!("\nLooking at {}\n", idx);
  fn wrap(idx: i32, width: i32, row: i32, col: i32) -> i32 {
    ((idx/width) * width) + row + (idx%width) + col
  }
  let occupied = [
    idx - 1),
    idx + 1,
    idx - width - 1,
    idx - width,
    idx - width + 1,
    idx + width - 1,
    idx + width,
    idx + width + 1,
  ]
  .iter()
  .filter_map(|f| {
    print!("A{} ", f);
    if *f < 0 { None } else { Some(*f as usize) }
  })
  .filter_map(|f| {
    print!("C{} - {:?};", f, state.get(f));
    state.get(f)
  })
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

fn print_state(state: &Vec<Tile>, width: usize) {
  // print!("\x1B[2J\x1B[1;1H");
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
