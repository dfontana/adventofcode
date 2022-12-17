use regex::Regex;
use rust_util::Day;
use std::{error::Error, fmt::Display};

type Pnt = (isize, isize);
pub struct Solve {
  pairs: Vec<(Pnt, Pnt)>,
  row: isize,
  max: isize,
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
              caps.get(1).unwrap().as_str().parse().unwrap(),
              caps.get(2).unwrap().as_str().parse().unwrap(),
            ),
            (
              caps.get(3).unwrap().as_str().parse().unwrap(),
              caps.get(4).unwrap().as_str().parse().unwrap(),
            ),
          )
        })
        .collect(),
      row: 2000000,
      max: 4000000,
    })
  }
}

struct Range {
  pub start: isize,
  pub end: isize,
}

impl Range {
  fn intersecting(&self, other: &Range) -> bool {
    self.end >= other.start && other.end >= self.start
  }

  fn contains(&self, n: isize) -> bool {
    self.start <= n && n <= self.end
  }

  fn covering(&self) -> isize {
    self.end - self.start + 1
  }

  fn add(self, other: Range) -> Range {
    Range {
      start: isize::min(self.start, other.start),
      end: isize::max(self.end, other.end),
    }
  }
}

fn dist_to((xa, ya): &Pnt, (xb, yb): &Pnt) -> isize {
  (xa - xb).abs() + (ya - yb).abs()
}

fn get_row_coverage(pairs: &Vec<(Pnt, Pnt)>, row: isize) -> Vec<Range> {
  let mut ranges: Vec<Range> = Vec::new();

  for (sensor, beacon) in pairs {
    // Get how width the sensor is at this row
    let mut new_r = match dist_to(sensor, beacon) - (sensor.1 - row).abs() {
      x if x < 0 => continue,
      width => Range {
        start: sensor.0 - width,
        end: sensor.0 + width,
      },
    };

    // Squash down ranges that overlap with this one, so we
    // have less overall to consider
    let mut i = 0;
    while i < ranges.len() {
      if ranges[i].intersecting(&new_r) {
        new_r = new_r.add(ranges.remove(i));
      } else {
        i += 1;
      }
    }
    ranges.push(new_r);
  }

  ranges
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let ans = get_row_coverage(&self.pairs, self.row)
      .iter()
      .map(|r| r.covering())
      .sum::<isize>()
      - 1;
    Ok(Box::new(format!("{:?}", ans)))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let ans = (0..=self.max)
      .map(|row| (row, get_row_coverage(&self.pairs, row)))
      // A row with a gap in it (not 1 big range) has a spot for a beacon
      .find(|(_, ranges)| ranges.len() > 1)
      .and_then(|(y, ranges)| {
        (0..=self.max)
          // Place where no ranges touch x
          .find(|x| !ranges.iter().any(|r| r.contains(*x)))
          .map(|x| x * 4_000_000 + y)
      });
    Ok(Box::new(format!("{:?}", ans)))
  }
}
