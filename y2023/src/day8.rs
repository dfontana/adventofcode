use itertools::Itertools;
use rust_util::Day;
use std::{collections::HashMap, error::Error, fmt::Display};

#[derive(Debug)]
pub struct Solve {
  ins: CircleBuf,
  maze: HashMap<String, (String, String)>,
}

#[derive(Clone, Debug)]
enum Dir {
  Left,
  Right,
}

#[derive(Clone, Debug)]
struct CircleBuf(Vec<Dir>, usize, usize);

impl CircleBuf {
  fn next(&mut self) -> &Dir {
    let n = &self.0[self.2];
    self.2 = (self.2 + 1) % self.1;
    n
  }
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let (inst, maze) = value.split_once("\n\n").unwrap();
    Ok(Solve {
      ins: CircleBuf(
        inst
          .chars()
          .map(|c| match c {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => unreachable!(),
          })
          .collect_vec(),
        inst.len(),
        0,
      ),
      maze: maze
        .lines()
        .filter_map(|l| l.split_once(" = "))
        .filter_map(|(key, tup)| {
          tup
            .trim_matches(|c| c == '(' || c == ')')
            .split_once(", ")
            .map(|(l, r)| (key.to_owned(), (l.to_owned(), r.to_owned())))
        })
        .collect(),
    })
  }
}

impl Solve {
  fn steps_to_end(&self, pos: &str, is_goal: impl Fn(&String) -> bool) -> usize {
    let mut ins = self.ins.clone();
    let mut step = 0;
    let mut ptr = &pos.to_string();
    while !is_goal(&ptr) {
      let (l, r) = self.maze.get(ptr).unwrap();
      ptr = match ins.next() {
        Dir::Left => l,
        Dir::Right => r,
      };
      step += 1;
    }
    step
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(self.steps_to_end("AAA", |s| s == "ZZZ")))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .maze
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|pos| self.steps_to_end(&pos, |s| s.ends_with('Z')))
        .reduce(|acc, e| lcm(acc, e))
        .unwrap(),
    ))
  }
}

fn gcd(a: usize, b: usize) -> usize {
  if a == 0 || b == 0 {
    a + b
  } else {
    let max = a.max(b);
    let min = a.min(b);
    gcd(max % min, min)
  }
}
fn lcm(a: usize, b: usize) -> usize {
  if a == 0 || b == 0 {
    0
  } else {
    (a * b) / gcd(a, b)
  }
}
