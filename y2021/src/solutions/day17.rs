use rust_util::{AocDay, Day};
use std::{error::Error, fmt::Display, ops::RangeInclusive};

pub struct Solve {
  max: (i32, (i32, i32)),
  hits: usize,
}

impl Day for Solve {
  fn new(_d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    // This one is just easier to enter rather than parse lol
    let target = Target {
      xrange: to_range(209, 238),
      yrange: to_range(-86, -59),
    };

    let mut max = (0, (0, 0));
    let mut hits = 0;

    // Feels like there should be a way to model the individual parabolas,
    // evaluate the x/y intercepts within the target range,
    // and inform the domain of intercepts better... but this also works
    for x in 0..1000 {
      for y in -1000..1000 {
        match simulate(&(x, y), &target) {
          Some(h) => {
            hits += 1;
            if h > max.0 {
              max = (h, (x, y));
            }
          }
          None => (),
        }
      }
    }

    Ok(Box::new(Solve { max, hits }))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(format!("{} {:?}", self.max.0, self.max.1)))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(self.hits))
  }
}

type Point = (i32, i32);
type Forces = (i32, i32);
#[derive(Debug)]
struct Target {
  xrange: RangeInclusive<i32>,
  yrange: RangeInclusive<i32>,
}

enum HitCheck {
  Hit,
  Miss,
  NotYet,
}

fn simulate(start_forces: &Forces, target: &Target) -> Option<i32> {
  let mut forces = start_forces.clone();
  let mut point = (0, 0);
  let mut max_y = 0;
  loop {
    point = step(&point, &forces);
    match is_hit(target, &point) {
      HitCheck::Hit => return Some(max_y),
      HitCheck::Miss => return None,
      HitCheck::NotYet => {
        if point.1 > max_y {
          max_y = point.1;
        }
      }
    }
    forces = degrade_forces(&forces);
  }
}

fn step((x1, y1): &Point, (vx, vy): &Forces) -> Point {
  (x1 + vx, y1 + vy)
}

fn degrade_forces((vx, vy): &Forces) -> Forces {
  let dx = if *vx > 0 {
    vx - 1
  } else if *vx < 0 {
    vx + 1
  } else {
    *vx
  };
  (dx, vy - 1)
}

fn to_range(start: i32, end: i32) -> RangeInclusive<i32> {
  if start > end {
    RangeInclusive::new(end, start)
  } else {
    RangeInclusive::new(start, end)
  }
}

fn is_hit(target: &Target, point: &Point) -> HitCheck {
  let lower_y = if target.yrange.start() < &0
    && target.yrange.end() < &0
    && target.yrange.start() < target.yrange.end()
  {
    target.yrange.start()
  } else {
    target.yrange.end()
  };
  if target.xrange.contains(&point.0) && target.yrange.contains(&point.1) {
    HitCheck::Hit
  } else if target.xrange.end() < &point.0 || lower_y > &point.1 {
    HitCheck::Miss
  } else {
    HitCheck::NotYet
  }
}
