use rust_util::{ Day};
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Action {
  N(i32),
  S(i32),
  E(i32),
  W(i32),
  L(i32),
  R(i32),
  F(i32),
}

#[derive(Clone, Debug, PartialEq)]
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

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      actions: value 
        .lines()
        .map(Action::from_str)
        .flatten()
        .collect(),
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut dir = Direction::East;
    let (mut x, mut y) = (0, 0);
    for act in self.actions.iter() {
      match act {
        Action::L(amt) | Action::R(amt) => {
          let mut t = [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
          ];
          match act {
            Action::R(_) => t.reverse(),
            _ => (),
          };
          let idx: usize = t.iter().position(|d| *d == dir).unwrap();
          dir = t[(idx + (amt / 90) as usize) % 4].clone();
        }
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
    Ok(Box::new((x.abs() + y.abs()).to_string()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let (mut ship_x, mut ship_y) = (0, 0);
    let (mut wp_x, mut wp_y) = (10, 1);
    for act in self.actions.iter() {
      match act {
        Action::L(amt) => {
          let (x, y) = rotate(-*amt, wp_x, wp_y);
          wp_x = x;
          wp_y = y;
        }
        Action::R(amt) => {
          let (x, y) = rotate(*amt, wp_x, wp_y);
          wp_x = x;
          wp_y = y;
        }
        Action::F(amt) => {
          ship_x += wp_x * amt;
          ship_y += wp_y * amt;
        }
        Action::N(amt) => wp_y += amt,
        Action::S(amt) => wp_y -= amt,
        Action::E(amt) => wp_x += amt,
        Action::W(amt) => wp_x -= amt,
      }
    }
    Ok(Box::new((ship_x.abs() + ship_y.abs()).to_string()))
  }
}

fn rotate(deg: i32, x: i32, y: i32) -> (i32, i32) {
  let f = (deg as f32) * (std::f32::consts::PI / 180.0);
  let a_x = (((x as f32) * f32::cos(f)) + ((y as f32) * f32::sin(f))).round() as i32;
  let a_y = (((y as f32) * f32::cos(f)) - ((x as f32) * f32::sin(f))).round() as i32;
  (a_x, a_y)
}
