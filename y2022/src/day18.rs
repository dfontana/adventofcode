use itertools::Itertools;
use rust_util::Day;
use std::{collections::HashSet, error::Error, fmt::Display};

type Coord = i8;
type Bounds = (Coords, Coords);
pub struct Solve {
  blocks: HashSet<Coords>,
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
struct Coords {
  x: Coord,
  y: Coord,
  z: Coord,
}

const OFFSETS: [(Coord, Coord, Coord); 6] = [
  (0, 0, 1),
  (0, 0, -1),
  (0, 1, 0),
  (0, -1, 0),
  (1, 0, 0),
  (-1, 0, 0),
];
impl Coords {
  fn neighbors(&self) -> Vec<Coords> {
    let mut neigh = Vec::new();
    for (dx, dy, dz) in OFFSETS {
      neigh.push(Coords {
        x: self.x + dx,
        y: self.y + dy,
        z: self.z + dz,
      });
    }
    neigh
  }

  fn in_bounds(&self, (min, max): &Bounds) -> bool {
    self.x >= min.x - 1
      && self.x <= max.x + 1
      && self.y >= min.y - 1
      && self.y <= max.y + 1
      && self.z >= min.z - 1
      && self.z <= max.z + 1
  }
}

struct ExposedBox {
  air: HashSet<Coords>,
}

fn bounds(blocks: &HashSet<Coords>) -> Bounds {
  let mut min = Coords::default();
  let mut max = Coords::default();
  for Coords { x, y, z } in blocks {
    min.x = *x.min(&min.x);
    max.x = *x.max(&max.x);
    min.y = *y.min(&min.y);
    max.y = *y.max(&max.y);
    min.z = *z.min(&min.z);
    max.z = *z.max(&max.z);
  }
  (min, max)
}

impl ExposedBox {
  fn from(blocks: &HashSet<Coords>) -> Self {
    let bounds = bounds(blocks);
    let mut air = HashSet::new();

    let start = Coords::default();
    let mut stack = Vec::new();
    let mut seen = HashSet::new();

    stack.push(start);
    seen.insert(start);

    while let Some(coord) = stack.pop() {
      for neighbor in coord.neighbors() {
        if blocks.contains(&neighbor) || !neighbor.in_bounds(&bounds) {
          continue;
        }
        if seen.insert(neighbor) {
          stack.push(neighbor);
          air.insert(neighbor);
        }
      }
    }

    ExposedBox { air }
  }

  fn contains(&self, coords: &Coords) -> bool {
    self.air.contains(coords)
  }
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      blocks: value
        .lines()
        .map(|l| {
          let (x, y, z) = l
            .split(',')
            .map(|c| c.parse::<Coord>().unwrap())
            .collect_tuple::<(_, _, _)>()
            .unwrap();
          Coords { x, y, z }
        })
        .collect(),
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .blocks
        .iter()
        .flat_map(Coords::neighbors)
        .filter(|c| !self.blocks.contains(c))
        .count()
        .to_string(),
    ))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let air = ExposedBox::from(&self.blocks);
    Ok(Box::new(
      self
        .blocks
        .iter()
        .flat_map(Coords::neighbors)
        .filter(|c| air.contains(c))
        .count()
        .to_string(),
    ))
  }
}
