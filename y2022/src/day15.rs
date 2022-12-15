use regex::Regex;
use rust_util::Day;
use std::{
  collections::{HashMap, HashSet},
  error::Error,
  fmt::Display,
};

type Pnt = (i32, i32);
pub struct Solve {
  pairs: Vec<(Pnt, Pnt)>,
  row: i32,
}

#[derive(Eq, PartialEq)]
enum Spot {
  Beacon,
  NotBeacon,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let regex = Regex::new(
      "Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)",
    )
    .unwrap();
    Ok(Solve {
      pairs: value
        .lines()
        .map(|l| {
          let caps = regex.captures(l).unwrap();
          (
            (
              caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
              caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            ),
            (
              caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
              caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            ),
          )
        })
        .collect(),
      row: 2000000,
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    // X -> Spot, y is assumed = self.row
    let mut row_comp: HashMap<i32, Spot> = HashMap::new();

    for (sensor, beacon) in self.pairs.iter() {
      if sensor.1 == self.row {
        row_comp
          .entry(sensor.0)
          .and_modify(|v| *v = Spot::NotBeacon)
          .or_insert_with(|| Spot::NotBeacon);
      }
      region(*sensor, dist_to(sensor, beacon), |(x, y)| {
        if y == self.row {
          row_comp
            .entry(x)
            .and_modify(|v| *v = Spot::NotBeacon)
            .or_insert_with(|| Spot::NotBeacon);
        }
      });

      if beacon.1 == self.row {
        row_comp
          .entry(beacon.0)
          .and_modify(|v| *v = Spot::Beacon)
          .or_insert_with(|| Spot::Beacon);
      }
    }

    Ok(Box::new(format!(
      "{:?}",
      row_comp.values().filter(|v| **v == Spot::NotBeacon).count()
    )))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new("y"))
  }
}

fn dist_to((xa, ya): &Pnt, (xb, yb): &Pnt) -> i32 {
  (xa - xb).abs() + (ya - yb).abs()
}

fn region(anchor: Pnt, dist: i32, mut hit: impl FnMut(Pnt)) {
  // Bottom of triangle
  let mut width = dist * 2;
  for row in 0..dist + 1 {
    for i in 0..width + 1 {
      let pt = ((anchor.0 - dist + row + i), (anchor.1 + row));
      hit(pt);
    }
    width -= 2;
  }

  // Top of triangle
  let mut width = dist * 2;
  for row in 0..dist + 1 {
    for i in 0..width + 1 {
      let pt = ((anchor.0 - dist + row + i), (anchor.1 - row));
      hit(pt);
    }
    width -= 2;
  }
}
