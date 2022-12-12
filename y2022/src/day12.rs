use rust_util::Day;
use std::{collections::VecDeque, error::Error, fmt::Display};

type Pnt = (usize, usize);
pub struct Solve {
  map: Vec<Vec<char>>,
  height: usize,
  width: usize,
  start: Pnt,
  goal: Pnt,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let map: Vec<Vec<char>> = value.lines().map(|l| l.chars().collect()).collect();
    let mut start = None;
    let mut goal = None;
    for (y, row) in map.iter().enumerate() {
      for (x, char) in row.iter().enumerate() {
        match char {
          'S' => {
            start = Some((x, y));
          }
          'E' => {
            goal = Some((x, y));
          }
          _ => (),
        }
      }
    }
    Ok(Solve {
      height: map.len(),
      width: map[0].len(),
      start: start.expect("Start point not found"),
      goal: goal.expect("Goal point not found"),
      map,
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let dirs = [(1, 0, true), (0,1, true), (1,0, false), (0,1, false)];

    let mut frontier: VecDeque<Vec<Pnt>> = VecDeque::from(vec![vec![self.start]]);
    let mut solve = None;
    while let Some(next) = frontier.pop_front() {
      let current_pos = next.last().expect("Cannot be empty");
      if current_pos == &self.goal {
        solve = Some(next);
        break;
      }
      let (cx, cy) = current_pos;
      for (dx, dy, subtract) in dirs {
        let next_point = match subtract {
            true => (cx.checked_sub(dx), cy.checked_sub(dy)),
            false => (cx.checked_add(dx), cy.checked_add(dy)),
        };
        // Can only push next point if it's
        // A) inside bounds
        // B) the char at that point is <= char @ current_pos + 1
        // Otherwise we push nothing

        let mut new_path = next.clone();
        new_path.push(next_point);
        frontier.push_back(new_path);
      }
    }

    Ok(Box::new(format!("{:?}", solve.map(|v| v.len()))))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new("y"))
  }
}
