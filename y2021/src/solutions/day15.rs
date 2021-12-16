use rust_util::{AocDay, Day};
use std::{
  cmp::Ordering,
  collections::{BinaryHeap, HashMap},
  error::Error,
  fmt::Display,
};

const NEIGHBORS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

// x, y
type Coord = (usize, usize);

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
  node: Coord,
  cost: usize,
}
impl Ord for State {
  // For min heap
  fn cmp(&self, other: &Self) -> Ordering {
    other.cost.cmp(&self.cost)
  }
}

impl PartialOrd for State {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

pub struct Solve {
  tile: HashMap<Coord, usize>,
  width: usize,
  height: usize,
}

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    let input = rust_util::read_input(2021, d)?;
    let mut tile: HashMap<Coord, usize> = HashMap::new();
    let (mut width, mut height) = (0, 0);
    for (y, line) in input.lines().enumerate() {
      width = line.len();
      for (x, weight) in line.chars().enumerate() {
        tile.insert((x, y), weight.to_digit(10).unwrap() as usize);
      }
      height = y + 1;
    }

    Ok(Box::new(Solve {
      tile,
      width,
      height,
    }))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let (nodes, goal) = build_map(&self.tile, self.width, self.height, 1);
    match find_path(&nodes, &goal) {
      Some((_, cost)) => Ok(Box::new(cost)),
      None => Err("No solve found".into()),
    }
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let (nodes, goal) = build_map(&self.tile, self.width, self.height, 5);
    match find_path(&nodes, &goal) {
      Some((_, cost)) => Ok(Box::new(cost)),
      None => Err("No solve found".into()),
    }
  }
}

fn build_map(
  template: &HashMap<Coord, usize>,
  w: usize,
  h: usize,
  scale: usize,
) -> (HashMap<Coord, usize>, Coord) {
  let mut nodes: HashMap<Coord, usize> = HashMap::new();
  let mut goal = (0, 0);

  for xoff in 0..scale {
    for yoff in 0..scale {
      for x in 0..w {
        for y in 0..h {
          let mut c_p = (template.get(&(x, y)).unwrap() + xoff + yoff) % 9;
          if c_p == 0 {
            c_p = 9;
          }

          let x_p = (w * xoff) + x;
          let y_p = (h * yoff) + y;
          nodes.insert((x_p, y_p), c_p);
          goal = (x_p, y_p);
        }
      }
    }
  }
  (nodes, goal)
}

fn find_path(nodes: &HashMap<Coord, usize>, goal: &Coord) -> Option<(Vec<Coord>, usize)> {
  let mut dist: HashMap<Coord, (usize, Option<Coord>)> = HashMap::new();
  for (k, _) in nodes {
    dist.insert(*k, (usize::MAX, None));
  }

  let mut heap = BinaryHeap::new();
  dist.insert((0, 0), (0, None));
  heap.push(State {
    node: (0, 0),
    cost: 0,
  });

  while let Some(State { node, cost }) = heap.pop() {
    if node == *goal {
      let mut path: Vec<Coord> = Vec::with_capacity(dist.len() / 2);
      let mut current_dist = dist.get(goal).unwrap();
      path.push(*goal);
      while let Some(prev) = current_dist.1 {
        path.push(prev);
        current_dist = dist.get(&prev).unwrap();
      }
      path.reverse();
      return Some((path, cost));
    }

    if cost > dist.get(&node).unwrap().0 {
      continue;
    }

    for edge in find_neighbors(nodes, &node) {
      let next = State {
        node: edge,
        cost: cost + nodes.get(&edge).unwrap(),
      };
      if next.cost < dist.get(&next.node).unwrap().0 {
        dist.insert(next.node, (next.cost, Some(node)));
        heap.push(next);
      }
    }
  }
  None
}

fn find_neighbors(nodes: &HashMap<Coord, usize>, (bx, by): &Coord) -> Vec<Coord> {
  NEIGHBORS
    .iter()
    .filter_map(|(dx, dy)| {
      let xp = bx.overflowing_add(*dx as usize).0;
      let yp = by.overflowing_add(*dy as usize).0;
      nodes.get_key_value(&(xp, yp))
    })
    .map(|(k, _)| *k)
    .collect()
}
