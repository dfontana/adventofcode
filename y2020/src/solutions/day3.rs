use crate::day::{Day, DayArg};
use crate::util::read_input;

use std::collections::HashMap;
use std::error::Error;

const TREE: char = '#';

pub struct Day3 {
  trees: HashMap<Point, bool>,
  height: usize,
  width: usize,
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Point {
  x: usize,
  y: usize,
}

impl Day for Day3 {
  fn new() -> Result<Day3, Box<dyn Error>> {
    let input = read_input(DayArg::D(3))?;
    let mut height = 0;
    let mut width = 0;
    let mut trees = HashMap::new();
    for (y, line) in input.lines().enumerate() {
      height += 1;
      if width == 0 {
        width = line.len();
      }
      for (x, ch) in line.chars().enumerate() {
        match ch {
          TREE => {
            trees.insert(Point { x, y }, true);
          }
          _ => (),
        }
      }
    }
    Ok(Day3 {
      trees,
      height,
      width,
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    Ok(format!("Trees hit: {}", trees_hit(&self, 3, 1)))
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok(format!(
      "Total {}",
      trees_hit(&self, 1, 1)
        * trees_hit(&self, 3, 1)
        * trees_hit(&self, 5, 1)
        * trees_hit(&self, 7, 1)
        * trees_hit(&self, 1, 2)
    ))
  }
}

fn trees_hit(day: &Day3, left: usize, down: usize) -> i32 {
  let mut i = 0;
  let mut x = 0;
  let mut y = 0;
  let mut trees_hit = 0;
  while day.height > i {
    x += left;
    y += down;
    if x > day.width - 1 {
      x -= day.width;
    }
    let test = Point { x, y };
    if day.trees.contains_key(&test) {
      trees_hit += 1;
    }
    i += 1;
  }
  return trees_hit;
}
