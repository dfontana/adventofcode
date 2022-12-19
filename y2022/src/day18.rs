use itertools::Itertools;
use rust_util::Day;
use std::{
  collections::{HashMap, HashSet},
  error::Error,
  fmt::Display,
};

type Coord = u8;
type Coords = (Coord, Coord, Coord);
pub struct Solve {
  blocks: Vec<Coords>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      blocks: value
        .lines()
        .map(|l| {
          l.split(",")
            .map(|c| c.parse::<Coord>().unwrap())
            .collect_tuple::<(_, _, _)>()
            .unwrap()
        })
        .collect(),
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut ylocal: HashMap<Coord, HashSet<(Coord, Coord)>> = HashMap::new();

    for (x, y, z) in &self.blocks {
      ylocal
        .entry(*y)
        .and_modify(|set| {set.insert((*x, *z));})
        .or_insert_with(|| {
          let mut set = HashSet::new();
          set.insert((*x, *z));
          set
        });
    }

    // println!("{:?}", ylocal);

    let mut surface_area = 0;
    for (x, y, z) in &self.blocks {
      // println!("{},{},{}", x, y, z);
      let mut block_area = 0;
      // Above:  x, y+1, z
      match ylocal.get(
        &(y-1)).map(|s| s.contains(&(*x, *z))) {
        Some(false) | None => block_area += 1,
        Some(true) => (),//println!("\tHit: {},{},{}",x,y-1,z),
      };
      // Below:  x, y-1, z
      match ylocal.get(&(y+1)).map(|s| s.contains(&(*x, *z))) {
        Some(false) | None => block_area += 1,
        Some(true) => (),//println!("\tHit: {},{},{}", x,y+1,z),
      };
      // Left:   x-1, y, z
      match ylocal.get(&y).map(|s| s.contains(&(*x-1, *z))) {
        Some(false) | None => block_area += 1,
        Some(true) => (),//println!("\tHit: {},{},{}", x-1,y,z),
      };
      // Right:  x+1, y, z
      match ylocal.get(&y).map(|s| s.contains(&(*x+1, *z))) {
        Some(false) | None => block_area += 1,
        Some(true) => (),//println!("\tHit: {},{},{}", x+1,y,z),
      };
      // Behind: x, y, z-1
      match ylocal.get(&y).map(|s| s.contains(&(*x, *z-1))) {
        Some(false) | None => block_area += 1,
        Some(true) => (),//println!("\tHit: {},{},{}", y,x,z-1),
      };
      // Front:  x, y, z+1
      match ylocal.get(&y).map(|s| s.contains(&(*x, *z+1))) {
        Some(false) | None => block_area += 1,
        Some(true) => (),//println!("\tHit: {},{},{}", y,x,z+1),
      };
      surface_area += block_area;
      // println!("\t-> {}", block_area);
    }

    Ok(Box::new(surface_area.to_string()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new("y"))
  }
}
