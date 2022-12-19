use itertools::Itertools;
use rust_util::Day;
use std::{
  collections::{HashMap, HashSet},
  error::Error,
  fmt::Display,
};

type Coord = u8;
pub struct Solve {
  blocks: HashSet<Coords>,
}
#[derive(Eq, Hash, PartialEq)]
struct Coords {
  x: Coord,
  y: Coord,
  z: Coord,
}

impl Coords {
  fn neighbors(&self) -> Vec<Coords> {
    // Just add the offsets to this & return
    vec![]
  }
}

struct ExposedBox {
    air: HashSet<Coords>,
}

impl ExposedBox {
  fn from(blocks: &HashSet<Coords>) -> Self {
    // (You should be able to just iterate, build, then subtract from the final set)
    ExposedBox {air: HashSet::new()}
  }

  /// Determines if the given coordinate is exposed
  fn contains(&self, coords: &Coords) -> bool {
      self.air.contains(coords)
  }
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      blocks: value
        .lines()
        .map(|l| {
          let (x, y, z) = l
            .split(',')
            .map(|c| c.parse::<Coord>().unwrap())
            .collect_tuple::<(_, _, _)>()
            .unwrap();
          Coords { x, y, z }
        })
        .collect(),
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .blocks
        .iter()
        .flat_map(Coords::neighbors)
        .filter(|c| !self.blocks.contains(c))
        .count()
        .to_string(),
    ))
    // let mut ylocal: HashMap<Coord, HashSet<(Coord, Coord)>> = HashMap::new();

    // for coord in &self.blocks {
    //   ylocal
    //     .entry(*y)
    //     .and_modify(|set| {set.insert((*x, *z));})
    //     .or_insert_with(|| {
    //       let mut set = HashSet::new();
    //       set.insert((*x, *z));
    //       set
    //     });
    // }

    // // println!("{:?}", ylocal);

    // let mut surface_area = 0;
    // for (x, y, z) in &self.blocks {
    //   // println!("{},{},{}", x, y, z);
    //   let mut block_area = 0;
    //   // Above:  x, y+1, z
    //   match ylocal.get(
    //     &(y-1)).map(|s| s.contains(&(*x, *z))) {
    //     Some(false) | None => block_area += 1,
    //     Some(true) => (),//println!("\tHit: {},{},{}",x,y-1,z),
    //   };
    //   // Below:  x, y-1, z
    //   match ylocal.get(&(y+1)).map(|s| s.contains(&(*x, *z))) {
    //     Some(false) | None => block_area += 1,
    //     Some(true) => (),//println!("\tHit: {},{},{}", x,y+1,z),
    //   };
    //   // Left:   x-1, y, z
    //   match ylocal.get(&y).map(|s| s.contains(&(*x-1, *z))) {
    //     Some(false) | None => block_area += 1,
    //     Some(true) => (),//println!("\tHit: {},{},{}", x-1,y,z),
    //   };
    //   // Right:  x+1, y, z
    //   match ylocal.get(&y).map(|s| s.contains(&(*x+1, *z))) {
    //     Some(false) | None => block_area += 1,
    //     Some(true) => (),//println!("\tHit: {},{},{}", x+1,y,z),
    //   };
    //   // Behind: x, y, z-1
    //   match ylocal.get(&y).map(|s| s.contains(&(*x, *z-1))) {
    //     Some(false) | None => block_area += 1,
    //     Some(true) => (),//println!("\tHit: {},{},{}", y,x,z-1),
    //   };
    //   // Front:  x, y, z+1
    //   match ylocal.get(&y).map(|s| s.contains(&(*x, *z+1))) {
    //     Some(false) | None => block_area += 1,
    //     Some(true) => (),//println!("\tHit: {},{},{}", y,x,z+1),
    //   };
    //   surface_area += block_area;
    //   // println!("\t-> {}", block_area);
    // }

    // // Get each coords neighbors and filter out those that are
    // // a in the original coord list (non-neighbors)
    // Ok(Box::new(surface_area.to_string()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    // Bounding box the min/max of all 3 dimensions, creating a cube the thing
    // fits within. Then flood fill to find all the air pockets.
    // Finally, for each cube, get it's neighbors but filter
    // them to only those that are in the flood fill list.
      let bbox = BBox::from(&self.blocks);
    Ok(Box::new(
        self.blocks
        .iter()
        .map(Coords::neighbors)
        .filter(|c| bbox.contains(c))
        .count()
        .to_string()
    ))
  }
}
