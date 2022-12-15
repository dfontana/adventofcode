use rust_util::Day;
use std::{
  collections::{HashMap, HashSet},
  error::Error,
  fmt::Display,
};

type Pnt = (i32, i32);
pub struct Solve {
  pairs: Vec<(Pnt, Pnt)>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve { pairs: Vec::new() })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut not_beacons: HashMap<i32, HashSet<Pnt>> = HashMap::new();
    let mut beacons = HashSet::new();
    for (sensor, beacon) in self.pairs.iter() { 
      not_beacons
        .entry(sensor.1)
        .and_modify(|v| {
          v.insert(*sensor);
        })
        .or_insert_with(|| {
          let mut set = HashSet::new();
          set.insert(*sensor);
          set
        });
      beacons.insert(beacon);
      // Walk out in all 4 directions the m-distance from sensor,
      // skipping over beacons
      let dist = dist_to(sensor, beacon);
    }

    Ok(Box::new(format!(
      "{:?}",
      not_beacons.get(&2000000).map(HashSet::len)
    )))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new("y"))
  }
}

fn dist_to(pntA: &Pnt, pntB: &Pnt) -> i32 {
  1
}
