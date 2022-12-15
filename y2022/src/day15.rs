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
  max: i32,
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
      row: 10,
      max: 20,
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
    // Somewhere in (0, 4000000) (for example it's 20)
    // Ret: x*4000000 + y
    // There is only 1 undetected spot within those bounds a beacon can be
    //
    // Brute force we could go row by row and save all the 'Non' spots
    // into a hashset, removing any that get spotted in the next row
    // That would take like 563ms / row * 4,000,000 ~= 26 days
    //
    // We can trade more memory by operating on N rows at a time
    // in memory tho worth of non-spaces. let's say 10000 => 3.7mins
    let mut row_comp: HashSet<Pnt> = HashSet::new();

    let step = 5.clamp(0, self.max);
    let mut low = 0;
    let mut high = step;
    while high <= self.max {
      let bbox = ((0, low), (self.max, high));
      println!("Running: {:?}", bbox);

      for (sensor, beacon) in self.pairs.iter() {
        if in_bbox(bbox, *sensor) {
          row_comp.insert(*sensor);
        }
        region_bbox(bbox, *sensor, dist_to(sensor, beacon), |pt| {
          row_comp.insert(pt);
        });
        if in_bbox(bbox, *beacon) {
          row_comp.insert(*beacon);
        }
      }
      println!("Done scanning");

      let mut non_spot = None;
      'outer: for y in low..high + 1 {
        for x in 0..self.max + 1 {
          let pt = (x, y);
          if !row_comp.contains(&pt) {
            if non_spot.is_none() {
              non_spot = Some(pt);
            } else {
              break 'outer;
            }
          }
        }
      }

      println!("Done evaluating");

      if let Some((x, y)) = non_spot {
        // We found something that went undetected in range,
        // it is the answer. Finish early
        return Ok(Box::new((x * 4000000 + y).to_string()));
      }
      low = high + 1;
      high += step;
      row_comp.clear();
    }

    Err("Not found".into())
  }
}

fn dist_to((xa, ya): &Pnt, (xb, yb): &Pnt) -> i32 {
  (xa - xb).abs() + (ya - yb).abs()
}

fn in_bbox((p1, p2): (Pnt, Pnt), (x, y): Pnt) -> bool {
  x >= p1.0 && x <= p2.0 && y >= p1.1 && y <= p2.1
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

fn region_bbox(bbox: (Pnt, Pnt), anchor: Pnt, dist: i32, mut hit: impl FnMut(Pnt)) {
  // Bottom of triangle
  let mut width = dist * 2;
  let mut row = 0;
  let low_y = anchor.1.clamp(bbox.0 .1, bbox.1 .1);
  let high_y = (anchor.1 + dist).clamp(bbox.0 .1, bbox.1 .1) + 1;
  for y in low_y..high_y {
    let low_x = (anchor.0 - dist + row).clamp(bbox.0 .0, bbox.1 .0);
    let high_x = (anchor.0 - dist + row + width).clamp(bbox.0 .0, bbox.1 .0) + 1;
    for i in low_x..high_x {
      let pt = (i, y);
      if pt == (14, 11) {
        println!("Answer hit lower");
      }
      hit(pt);
    }
    row += 1;
    width -= 2;
  }

  // Top of triangle
  let mut width = dist * 2;
  let mut row = 0;
  let low_y = (anchor.1 - dist).clamp(bbox.0 .1, bbox.1 .1);
  let high_y = (anchor.1).clamp(bbox.0 .1, bbox.1 .1) + 1;
  for y in low_y..high_y {
    let low_x = (anchor.0 - dist + row).clamp(bbox.0 .0, bbox.1 .0);
    let high_x = (anchor.0 - dist + row + width).clamp(bbox.0 .0, bbox.1 .0) + 1;
    for i in low_x..high_x {
      let pt = (i, y);
      if pt == (14, 11) {
        println!("Answer hit lower");
      }
      hit(pt);
    }
    row += 1;
    width -= 2;
  }
}
