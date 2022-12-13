use rust_util::Day;
use std::{
  collections::{HashMap, HashSet, VecDeque},
  error::Error,
  fmt::Display,
};

type Pnt = (usize, usize);
pub struct Solve {
  value: String,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve { value })
  }
}

fn parse(value: String, rev: bool) -> (Vec<Vec<usize>>, Pnt, Pnt) {
  let mut map: Vec<Vec<usize>> = Vec::new();
  let mut start = None;
  let mut goal = None;
  for (y, row) in value.lines().enumerate() {
    let mut nrow = Vec::new();
    for (x, char) in row.chars().enumerate() {
      nrow.push(match (char, rev) {
        ('S', false) => {
          start = Some((x, y));
          0
        }
        ('S', true) => {
          goal = Some((x, y));
          25
        }
        ('E', false) => {
          goal = Some((x, y));
          25
        }
        ('E', true) => {
          start = Some((x, y));
          0
        }
        (_, false) => char as usize - 'a' as usize,
        (_, true) => 'z' as usize - char as usize,
      });
    }
    map.push(nrow);
  }
  (
    map,
    start.expect("Start not found"),
    goal.expect("Goal not found"),
  )
}

fn search(
  map: Vec<Vec<usize>>,
  start: Pnt,
  goal_test: impl Fn(Pnt, usize) -> Option<Pnt>,
) -> usize {
  let dirs = [(1, 0, true), (0, 1, true), (1, 0, false), (0, 1, false)];

  let mut dist_to: HashMap<Pnt, usize> = std::iter::once((start, 0)).collect();
  let mut explored: HashSet<Pnt> = HashSet::new();
  let mut frontier: VecDeque<Pnt> = VecDeque::from(vec![start]);

  while let Some(current_pos @ (cx, cy)) = frontier.pop_front() {
    if !explored.insert(current_pos) {
      continue;
    }

    let current_h = map[cy][cx];
    if let Some(solve) = goal_test(current_pos, current_h) {
      return dist_to[&solve];
    }

    for (dx, dy, subtract) in dirs {
      let next_point = match subtract {
        true => (cx.checked_sub(dx), cy.checked_sub(dy)),
        false => (cx.checked_add(dx), cy.checked_add(dy)),
      };
      let (Some(nx), Some(ny)) = next_point else {
            continue;
      };
      let Some(next_h) = map.get(ny).and_then(|row| row.get(nx)) else {
            continue;
      };
      if *next_h > current_h + 1 {
        continue;
      }
      let current_dist = dist_to[&current_pos];
      let next_dist = dist_to.entry((nx, ny)).or_insert(usize::MAX);
      if current_dist < *next_dist {
        dist_to.insert((nx, ny), current_dist + 1);
        frontier.push_back((nx, ny));
      }
    }
  }
  0
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let (map, start, goal) = parse(self.value.to_owned(), false);
    let dist = search(map, start, |current_pos, _| {
      if current_pos == goal {
        Some(goal)
      } else {
        None
      }
    });
    Ok(Box::new(format!("{:?}", dist)))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let (map, start, _) = parse(self.value.to_owned(), true);
    let dist = search(map, start, |current_pos, current_h| {
      if current_h == 25 {
        Some(current_pos)
      } else {
        None
      }
    });
    Ok(Box::new(format!("{:?}", dist)))
  }
}
