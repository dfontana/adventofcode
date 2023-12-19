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

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut last = (0, 0);
    let mut perm = 0;
    let mut area = 0;
    for Inst { dir, amt, color: _ } in self.input.iter() {
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
    Ok(Box::new(area / 2 + perm / 2 + 1))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(1))
  }
}
