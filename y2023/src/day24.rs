use itertools::Itertools;
use rust_util::Day;
use std::{error::Error, fmt::Display};

pub struct Solve {
  input: Vec<Vector>,
}

// (x, y, z)
#[derive(Debug)]
struct Vector((isize, isize, isize), (isize, isize, isize));

#[derive(Debug, PartialEq, Eq)]
enum Alignment {
  Same,
  Intersect((isize, isize)),
  Past((isize, isize)),
  None,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let input = value
      .lines()
      .filter_map(|l| l.split_once(" @ "))
      .map(|(pos, vel)| {
        let poss = pos
          .split(",")
          .map(|v| v.trim().parse::<isize>().unwrap())
          .collect_tuple()
          .unwrap();
        let vels = vel
          .split(",")
          .map(|v| v.trim().parse::<isize>().unwrap())
          .collect_tuple()
          .unwrap();
        Vector(poss, vels)
      })
      .collect();
    Ok(Solve { input })
  }
}

impl Vector {
  fn to_segment(&self) -> ((isize, isize), (isize, isize)) {
    (
      (self.0 .0, self.0 .1),
      (self.0 .0 + self.1 .0, self.0 .1 + self.1 .1),
    )
  }

  fn contains(&self, other: &Vector) -> bool {
    self.contains_pt(&(other.0 .0, other.0 .1))
  }

  fn contains_pt(&self, (x, y): &(isize, isize)) -> bool {
    let slope = self.1 .1 as f64 / self.1 .0 as f64;
    let b = self.0 .1 as f64 - slope * self.0 .0 as f64;
    *y as f64 == slope * (*x as f64) + b
  }

  fn in_future(&self, (x, y): &(isize, isize)) -> bool {
    let in_x_dir = (*x < self.0 .0 && self.1 .0 < 0)
      || (*x > self.0 .0 && self.1 .0 > 0)
      || (*x == self.0 .0 && self.1 .0 == 0);
    let in_y_dir = (*y < self.0 .1 && self.1 .1 < 0)
      || (*y > self.0 .1 && self.1 .1 > 0)
      || (*y == self.0 .1 && self.1 .1 == 0);
    in_y_dir && in_x_dir
  }

  fn xy_intersect(&self, other: &Vector) -> Alignment {
    let ma = self.1 .1 as f64 / self.1 .0 as f64;
    let mb = other.1 .1 as f64 / other.1 .0 as f64;
    if ma == mb {
      if self.contains(other) {
        return Alignment::Same;
      }
      return Alignment::None;
    }
    let ba = self.0 .1 as f64 - (ma * self.0 .0 as f64);
    let bb = other.0 .1 as f64 - (mb * other.0 .0 as f64);
    let xpos = (bb - ba) / (ma - mb);
    let p = (xpos as isize, (ma * xpos + ba) as isize);
    if self.in_future(&p) && other.in_future(&p) {
      Alignment::Intersect(p)
    } else {
      Alignment::Past(p)
    }
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let rg = 200000000000000..=400000000000000;
    let mut count = 0;
    for i in 0..self.input.len() {
      for j in i..self.input.len() {
        match self.input[i].xy_intersect(&self.input[j]) {
          Alignment::Same => {
            if self.input[i].contains_pt(&(*rg.start(), *rg.start())) {
              count += 1;
            }
          }
          Alignment::Intersect((x, y)) => {
            if rg.contains(&x) && rg.contains(&y) {
              count += 1;
            }
          }
          _ => {}
        }
      }
    }
    Ok(Box::new(count))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(1))
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use test_case::test_case;

  #[test_case(
    Vector((18, 19,22), (-1, -1, -2)),
    Vector((20, 25, 34), (-2, -2, -4)),
    Alignment::None;
    "parallel")]
  #[test_case(
    Vector((19, 13, 30), (-2, 1, -2)),
    Vector((18, 19, 22), (-1, -1, -2)),
    Alignment::Intersect((14,15));
    "intersects non-integer")]
  #[test_case(
    Vector((19, 13, 30), (-2, 1, -2)),
    Vector((20, 25, 34), (-2,-2,-4)),
    Alignment::Intersect((11, 16));
    "intersects non-integer 2")]
  #[test_case(
    Vector((18,19,22), (-1,-1,-2)),
    Vector((12,31,28), (-1,-2,-1)),
    Alignment::Intersect((-6, -5));
    "intersects integer")]
  #[test_case(
    Vector((20,25,34), (-2,-2,-4)),
    Vector((12,31,28), (-1,-2,-1)),
    Alignment::Intersect((-2, 3));
    "intersects integer 2")]
  #[test_case(
    Vector((20,25,34), (-2,-2,-4)),
    Vector((20,19,15), (1,-5,-3)),
    Alignment::Past((19, 24));
    "intersects in past")]
  #[test_case(
    Vector((10, 10, 10), (2, 3, 0)),
    Vector((20, 20, 20), (3, 2, 0)),
    Alignment::Past((14, 16));
    "perpendicular off axis")]
  #[test_case(
    Vector((10, 10, 10), (5, 3, 0)),
    Vector((10, 10, 10), (4, 2, 0)),
    Alignment::Past((10, 10));
    "Same start, diff velocities")]
  #[test_case(
    Vector((10, 10, 0), (5, 5, 0)),
    Vector((10, 10, 0), (2, 2, 0)),
    Alignment::Same;
    "same line diff velocity")]
  #[test_case(
    Vector((10, 10, 0), (5, 5, 0)),
    Vector((10, 10, 0), (5, 5, 0)),
    Alignment::Same;
    "same line same velocity")]
  fn test_xy_intersect(v1: Vector, v2: Vector, exp: Alignment) {
    assert_eq!(v1.xy_intersect(&v2), exp)
  }
}
