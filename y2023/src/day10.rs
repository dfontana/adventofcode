use itertools::Itertools;
use rust_util::Day;
use std::{collections::HashMap, error::Error, fmt::Display};

#[derive(Debug)]
pub struct Solve {
  start_loc: (usize, usize),
  nodes: HashMap<(usize, usize), Pipe>,
  graph: HashMap<(usize, usize), Adj>,
}

#[derive(Debug)]
struct Adj([Option<(usize, usize)>; 4]);
impl Adj {
  fn new() -> Self {
    Adj([None, None, None, None])
  }
  fn update(&mut self, dir: &Dir, loc: (usize, usize)) {
    match dir {
      Dir::North => self.0[0] = Some(loc),
      Dir::East => self.0[1] = Some(loc),
      Dir::South => self.0[2] = Some(loc),
      Dir::West => self.0[3] = Some(loc),
    };
  }

  fn get(&self) -> ((usize, usize), (usize, usize)) {
    self.0.iter().filter_map(|p| *p).collect_tuple().unwrap()
  }

  fn get_from(&self, from: &(usize, usize)) -> (usize, usize) {
    self
      .0
      .iter()
      .filter_map(|p| *p)
      .filter(|p| p != from)
      .next()
      .unwrap()
  }
}

enum Dir {
  North,
  East,
  South,
  West,
}

#[derive(Debug, PartialEq, Eq)]
enum Pipe {
  VERT, // North-South
  HORZ, // East-West
  NE90, // North-East
  SE90, // South-East
  NW90, // North-West
  SW90, // South-West
  Blank,
  Start,
}

impl From<char> for Pipe {
  fn from(value: char) -> Self {
    match value {
      '|' => Pipe::VERT,
      '-' => Pipe::HORZ,
      'L' => Pipe::NE90,
      'F' => Pipe::SE90,
      'J' => Pipe::NW90,
      '7' => Pipe::SW90,
      '.' => Pipe::Blank,
      'S' => Pipe::Start,
      _ => unreachable!(),
    }
  }
}

impl Pipe {
  fn can_connect(&self, other: &Pipe, dir: &Dir) -> bool {
    match (self, other, dir) {
      (Pipe::VERT, Pipe::Start | Pipe::VERT, Dir::North | Dir::South) => true,
      (Pipe::VERT, Pipe::NE90 | Pipe::NW90, Dir::South) => true,
      (Pipe::VERT, Pipe::SE90 | Pipe::SW90, Dir::North) => true,
      (Pipe::HORZ, Pipe::Start | Pipe::HORZ, Dir::West | Dir::East) => true,
      (Pipe::HORZ, Pipe::NE90 | Pipe::SE90, Dir::West) => true,
      (Pipe::HORZ, Pipe::NW90 | Pipe::SW90, Dir::East) => true,
      (Pipe::NE90, Pipe::VERT | Pipe::SE90, Dir::North) => true,
      (Pipe::NE90, Pipe::HORZ | Pipe::NW90, Dir::East) => true,
      (Pipe::NE90, Pipe::Start | Pipe::SW90, Dir::North | Dir::East) => true,
      (Pipe::SE90, Pipe::VERT | Pipe::NE90, Dir::South) => true,
      (Pipe::SE90, Pipe::HORZ | Pipe::SW90, Dir::East) => true,
      (Pipe::SE90, Pipe::Start | Pipe::NW90, Dir::East | Dir::South) => true,
      (Pipe::NW90, Pipe::VERT | Pipe::SW90, Dir::North) => true,
      (Pipe::NW90, Pipe::HORZ | Pipe::NE90, Dir::West) => true,
      (Pipe::NW90, Pipe::Start | Pipe::SE90, Dir::North | Dir::West) => true,
      (Pipe::SW90, Pipe::VERT | Pipe::NW90, Dir::South) => true,
      (Pipe::SW90, Pipe::HORZ | Pipe::SE90, Dir::West) => true,
      (Pipe::SW90, Pipe::Start | Pipe::NE90, Dir::South | Dir::West) => true,
      (Pipe::Start, Pipe::VERT | Pipe::SE90 | Pipe::SW90, Dir::North) => true,
      (Pipe::Start, Pipe::VERT | Pipe::NE90 | Pipe::NW90, Dir::South) => true,
      (Pipe::Start, Pipe::HORZ | Pipe::NW90 | Pipe::SW90, Dir::East) => true,
      (Pipe::Start, Pipe::HORZ | Pipe::NE90 | Pipe::SE90, Dir::West) => true,
      _ => false,
    }
  }
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let nodes: HashMap<_, _> = value
      .lines()
      .enumerate()
      .flat_map(|(y, l)| {
        l.chars()
          .enumerate()
          .map(|(x, c)| ((x + 1, y + 1), Pipe::from(c)))
          .collect_vec()
      })
      .filter(|(_, p)| *p != Pipe::Blank)
      .collect();

    let start_loc = nodes
      .iter()
      .find_map(|(loc, p)| match p {
        Pipe::Start => Some(loc.clone()),
        _ => None,
      })
      .unwrap();

    let mut graph: HashMap<(usize, usize), Adj> = HashMap::new();
    for ((x, y), p) in nodes.iter() {
      for (maybe_dloc, dir) in vec![
        (Some((x + 1, *y)), Dir::East),
        (y.checked_sub(1).map(|y| (*x, y)), Dir::North),
        (Some((*x, y + 1)), Dir::South),
        (x.checked_sub(1).map(|x| (x, *y)), Dir::West),
      ] {
        if let Some(dloc) = maybe_dloc {
          if let Some(_) = nodes.get(&dloc).filter(|pi| p.can_connect(pi, &dir)) {
            graph
              .entry((*x, *y))
              .and_modify(|adj| adj.update(&dir, dloc))
              .or_insert_with(|| {
                let mut adj = Adj::new();
                adj.update(&dir, dloc);
                adj
              });
          }
        }
      }
    }

    Ok(Solve {
      start_loc,
      nodes,
      graph,
    })
  }
}

impl Solve {
  fn steps_to_meet(&self) -> usize {
    let mut step = 1;

    let (mut p_loc_a, mut p_loc_b) = (self.start_loc, self.start_loc);
    let (mut loc_a, mut loc_b) = self
      .graph
      .get(&self.start_loc)
      .map(|adj| adj.get())
      .unwrap();

    loop {
      step += 1;

      let n_loc_a = self.graph.get(&loc_a).map(|adj| adj.get_from(&p_loc_a));
      p_loc_a = loc_a;
      loc_a = n_loc_a.unwrap();

      let n_loc_b = self.graph.get(&loc_b).map(|adj| adj.get_from(&p_loc_b));
      p_loc_b = loc_b;
      loc_b = n_loc_b.unwrap();

      if loc_a == loc_b {
        break;
      }
    }

    step
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(self.steps_to_meet()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(1))
  }
}
