use itertools::Itertools;
use rust_util::Day;
use std::{collections::HashSet, error::Error, fmt::Display};

type Pnt = (usize, usize);
pub struct Solve {
  start: Pnt,
  realm: HashSet<Pnt>,
  bbox: (Pnt, Pnt),
  floor: usize,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let realm = value
      .lines()
      .map(|l| {
        // Parse the line of points that form a Path
        l.split(" -> ")
          .filter_map(|pt| pt.split_once(','))
          .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
          .collect::<Vec<Pnt>>()
      })
      .flat_map(|p| {
        // Find all the points that the paths occupy
        p.iter()
          .tuple_windows()
          .flat_map(|(pta, ptb)| {
            if pta.0 == ptb.0 {
              (pta.1.min(ptb.1)..pta.1.max(ptb.1) + 1)
                .map(|y| (pta.0, y))
                .collect::<Vec<Pnt>>()
            } else {
              (pta.0.min(ptb.0)..pta.0.max(ptb.0) + 1)
                .map(|x| (x, pta.1))
                .collect::<Vec<Pnt>>()
            }
          })
          .collect::<Vec<Pnt>>()
      })
      .collect::<HashSet<_>>();

    let mut largest_x: usize = 0;
    let mut largest_y: usize = 0;
    for (x, y) in realm.iter() {
      if *x > largest_x {
        largest_x = *x;
      }
      if *y > largest_y {
        largest_y = *y;
      }
    }

    Ok(Solve {
      start: (500, 0),
      realm,
      bbox: ((0, 0), (largest_x, largest_y)),
      floor: largest_y + 2,
    })
  }
}

fn in_bbox(bbox: &(Pnt, Pnt), sand: &Pnt) -> bool {
  sand.0 >= bbox.0 .0 && sand.1 >= bbox.0 .1 && sand.0 <= bbox.1 .0 && sand.1 <= bbox.1 .1
}

impl Solve {
  fn simulate(&self, off_edge: bool) -> usize {
    let mut occupied = self.realm.clone();

    let mut sandcnt = 0;
    let mut sim_done = false;
    while !sim_done {
      let mut sand = self.start;
      loop {
        let down = (sand.0, sand.1 + 1);
        let down_left = (sand.0 - 1, sand.1 + 1);
        let down_right = (sand.0 + 1, sand.1 + 1);
        if !occupied.contains(&down) {
          if off_edge && !in_bbox(&self.bbox, &down) {
            sim_done = true;
            break;
          } else if off_edge || down.1 < self.floor {
            sand.1 += 1;
            continue;
          }
        } else if !occupied.contains(&down_left) {
          if off_edge && !in_bbox(&self.bbox, &down_left) {
            sim_done = true;
            break;
          } else if off_edge || down_left.1 < self.floor {
            sand.1 += 1;
            sand.0 -= 1;
            continue;
          }
        } else if !occupied.contains(&down_right) {
          if off_edge && !in_bbox(&self.bbox, &down_right) {
            sim_done = true;
            break;
          } else if off_edge || down_right.1 < self.floor {
            sand.1 += 1;
            sand.0 += 1;
            continue;
          }
        }
        occupied.insert(sand);
        sandcnt += 1;
        if !off_edge && sand == self.start {
          sim_done = true;
        }
        break;
      }
    }

    sandcnt
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(self.simulate(true).to_string()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(self.simulate(false).to_string()))
  }
}
