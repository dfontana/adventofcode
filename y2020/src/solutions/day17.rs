use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::{collections::HashMap, error::Error, str::FromStr};

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
}

impl Coord {
  pub fn new(x: i32, y: i32, z: i32) -> Coord {
    Coord { x, y, z }
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

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    Ok(Solve {
      state: read_input(d)?
        .lines()
        .enumerate()
        .map(|(y, l)| {
          l.chars()
            .enumerate()
            .map(move |(x, c)| (x as i32, y as i32, Cell::from_str(&c.to_string())))
        })
        .flatten()
        .fold(HashMap::new(), |mut acc, (x, y, may_cell)| {
          acc.insert(Coord::new(x, y, 0), may_cell.unwrap());
          acc
        }),
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    let mut state = self.state.clone();
    print_state(&state);
    for _ in 0..6 {
      state = update_state(&state);
      print_state(&state);
    }
    Ok(
      state
        .values()
        .filter(|c| **c == Cell::ACTIVE)
        .count()
        .to_string(),
    )
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok("Impl".to_string())
  }
}

fn update_state(state: &HashMap<Coord, Cell>) -> HashMap<Coord, Cell> {
  state
    .keys()
    .map(|coor| {
      let mut nbs: Vec<Coord> = expand_3d(coor);
      nbs.push(coor.clone());
      nbs
        .iter()
        .map(|nc| map_cell(state, nc))
        .collect::<Vec<(Coord, Cell)>>()
    })
    .flatten()
    .fold(HashMap::new(), |mut acc, (coor, cell)| {
      acc.insert(coor, cell);
      acc
    })
}

fn map_cell(state: &HashMap<Coord, Cell>, coor: &Coord) -> (Coord, Cell) {
  let nbs: Vec<Coord> = expand_3d(coor);
  let cell = state.get(&coor).unwrap_or(&Cell::INACTIVE);
  let count = nbs
    .iter()
    .filter(|c| state.get(*c).filter(|ce| **ce == Cell::ACTIVE).is_some())
    .count();
  // if *coor == Coord::new(0,2,0) {
  //   println!("{:?}", nbs.iter().map(|n| ((n.x, n.y, n.z), state.get(n).unwrap_or(&Cell::INACTIVE))).collect::<Vec<((i32,i32,i32), &Cell)>>());
  // }
  let ans = match count {
    2 | 3 if *cell == Cell::ACTIVE => (coor.clone(), Cell::ACTIVE),
    3 if *cell == Cell::INACTIVE => (coor.clone(), Cell::ACTIVE),
    _ => (coor.clone(), Cell::INACTIVE),
  };
  ans
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

fn print_state(state: &HashMap<Coord, Cell>) {
  // println!("STEP");
  // for x in -7..7i32 {
  //   print!("{}", x.abs());
  // }
  // println!();
  // for z in -1..2 {
  //   for y in -7..7 {
  //     for x in -7..7 {
  //       let c = match state.get(&Coord::new(x,y,z)) {
  //         Some(c) => {
  //           match c {
  //             Cell::ACTIVE => "#",
  //             Cell::INACTIVE => "."
  //           }
  //         },
  //         _ => "."
  //       };
  //       print!("{}", c);
  //     }
  //     print!("{}\n", y);
  //   }
  //   print!("\n\n");
  // }
}
