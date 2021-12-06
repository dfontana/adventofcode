use rust_util::{AocDay, Day};
use std::{collections::HashMap, error::Error, fmt::Display};

pub struct Solve {
  segments: Vec<Segment>,
}

type Point = (i32, i32);
type Segment = (Point, Point);

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    Ok(Box::new(Solve {
      segments: rust_util::read_input(2021, d)?
        .lines()
        .map(|l| {
          l.split_once(" -> ")
            .map(|sp| (point_from_str(sp.0), point_from_str(sp.1)))
            .unwrap()
        })
        .collect(),
    }))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(count_overlaps(
      self
        .segments
        .iter()
        .filter(|(p1, p2)| p1.0 == p2.0 || p1.1 == p2.1),
    )))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(count_overlaps(self.segments.iter())))
  }
}

fn point_from_str(v: &str) -> Point {
  v.split_once(",")
    .map(|sp| (sp.0.parse::<i32>().unwrap(), sp.1.parse::<i32>().unwrap()))
    .unwrap()
}

fn count_overlaps<'a, Iter>(segments: Iter) -> usize
where
  Iter: Iterator<Item = &'a Segment>,
{
  // Fairly naive solve, since I iterate point by point.
  // There might be a solve that could check if lines intesect
  // and for how long each intersect is, summed. Would require
  // checking co-linearity
  let mut arena: HashMap<Point, i32> = HashMap::new();
  segments.for_each(|(p1, p2)| {
    let x_dir = (-1 * (p1.0 - p2.0)).clamp(-1, 1);
    let y_dir = (-1 * (p1.1 - p2.1)).clamp(-1, 1);
    let (mut ptr_x, mut ptr_y) = (0, 0);
    let (mut x_rng, mut y_rng) = ((p2.0 - p1.0).abs(), (p2.1 - p1.1).abs());
    while x_rng >= 0 || y_rng >= 0 {
      arena
        .entry((p1.0 + ptr_x, p1.1 + ptr_y))
        .and_modify(|cnt| *cnt += 1)
        .or_insert(1);
      (ptr_y += y_dir, ptr_x += x_dir);
      (y_rng -= 1, x_rng -= 1);
    }
  });
  arena.values().filter(|v| **v > 1).count()
}
