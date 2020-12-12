use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::{error::Error, str::FromStr};

#[derive(Debug)]
enum Action {
  N(i32),
  S(i32),
  E(i32),
  W(i32),
  L(i32),
  R(i32),
  F(i32),
}

#[derive(Clone)]
enum Direction {
  North,
  East,
  West,
  South,
}

pub struct Solve {
  actions: Vec<Action>,
}

impl FromStr for Action {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (act, amt) = (
      &s[..1],
      &s[1..]
        .parse::<i32>()
        .map_err(|_| "Can't parse amt".to_string())?,
    );
    match act {
      "N" => Ok(Action::N(*amt)),
      "S" => Ok(Action::S(*amt)),
      "E" => Ok(Action::E(*amt)),
      "W" => Ok(Action::W(*amt)),
      "L" => Ok(Action::L(*amt)),
      "R" => Ok(Action::R(*amt)),
      "F" => Ok(Action::F(*amt)),
      _ => Err("Unknown action type".to_string()),
    }
  }
}

impl Direction {
  fn turn(&self, act: &Action) -> Direction {
    let upper = match act {
      Action::R(amt) | Action::L(amt) => amt,
      _ => return self.clone(),
    };
    let mut crt = 0;
    let mut fin = self.clone();
    while crt < *upper {
      fin = match act {
        Action::L(_) => match fin {
          Direction::North => Direction::West,
          Direction::West => Direction::South,
          Direction::South => Direction::East,
          Direction::East => Direction::North,
        },
        Action::R(_) => match fin {
          Direction::North => Direction::East,
          Direction::East => Direction::South,
          Direction::South => Direction::West,
          Direction::West => Direction::North,
        },
        _ => return fin,
      };
      crt += 90;
    }
    fin
  }
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    Ok(Solve {
      actions: read_input(d)?
        .lines()
        .map(Action::from_str)
        .flatten()
        .collect(),
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    let mut dir = Direction::East;
    let (mut x, mut y) = (0, 0);
    for act in self.actions.iter() {
      match act {
        Action::L(_) | Action::R(_) => dir = dir.turn(act),
        Action::F(amt) => match dir {
          Direction::North => y += amt,
          Direction::East => x += amt,
          Direction::South => y -= amt,
          Direction::West => x -= amt,
        },
        Action::N(amt) => y += amt,
        Action::S(amt) => y -= amt,
        Action::E(amt) => x += amt,
        Action::W(amt) => x -= amt,
      }
    }
    Ok((x.abs() + y.abs()).to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok("Impl".to_string())
  }
}
