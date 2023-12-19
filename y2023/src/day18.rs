use itertools::Itertools;
use rust_util::{
  grid::{Dir, Grid},
  Day,
};
use std::{error::Error, fmt::Display};

pub struct Solve {
  input: Vec<Inst>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Inst {
  dir: Dir,
  amt: isize,
  color: String,
}

impl From<(&str, &str, &str)> for Inst {
  fn from((d, n, cde): (&str, &str, &str)) -> Self {
    Inst {
      dir: match d {
        "R" => Dir::E,
        "D" => Dir::S,
        "L" => Dir::W,
        "U" => Dir::N,
        _ => unreachable!(),
      },
      amt: n.parse().unwrap(),
      color: cde
        .strip_prefix("(#")
        .and_then(|s| s.strip_suffix(")"))
        .unwrap()
        .to_string(),
    }
  }
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      input: value
        .lines()
        .map(|s| s.splitn(3, ' '))
        .map(|mut s| (s.next().unwrap(), s.next().unwrap(), s.next().unwrap()))
        .map(Inst::from)
        .collect_vec(),
    })
  }
}

fn shoelace_picks(inst: Vec<(Dir, isize)>) -> isize {
  let mut last = (0, 0);
  let mut perm = 0;
  let mut area = 0;
  for (dir, amt) in inst.iter() {
    let next = match dir {
      Dir::N => (last.0 - amt, last.1),
      Dir::E => (last.0, last.1 + amt),
      Dir::S => (last.0 + amt, last.1),
      Dir::W => (last.0, last.1 - amt),
      _ => unreachable!(),
    };
    perm += amt;
    area += last.1 * next.0 - next.1 * last.0;
    last = next;
  }
  area / 2 + perm / 2 + 1
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(shoelace_picks(
      self
        .input
        .iter()
        .map(|Inst { dir, amt, .. }| (dir.clone(), *amt))
        .collect(),
    )))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(shoelace_picks(
      self
        .input
        .iter()
        .map(|Inst { color, .. }| {
          let amt = isize::from_str_radix(&color[..5], 16).unwrap();
          let dir = match &color[5..] {
            "0" => Dir::E,
            "1" => Dir::S,
            "2" => Dir::W,
            "3" => Dir::N,
            _ => unreachable!(),
          };
          (dir, amt)
        })
        .collect(),
    )))
  }
}
