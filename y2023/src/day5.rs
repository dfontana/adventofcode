use rust_util::Day;
use std::{
  collections::{HashMap, HashSet, VecDeque},
  error::Error,
  fmt::Display,
  ops::Range,
};

type Type = String;
type Id = usize;

#[derive(Debug)]
pub struct Solve {
  seeds: HashSet<Id>,
  maps: HashMap<(Type, Type), Mapping>,
}
#[derive(Debug)]
struct Mapping {
  dest_type: Type,
  ranges: Vec<(Range<Id>, Range<Id>)>,
}
impl Mapping {
  fn maps_to(&self, id: &Id) -> Option<(Type, Id)> {
    todo!();
  }
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      seeds: HashSet::new(),
      maps: HashMap::new(),
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut locations: HashSet<Id> = HashSet::new();
    let mut frontier: VecDeque<(Type, Type, Id)> = VecDeque::new();

    // Init the BFS
    for seed in self.seeds.iter() {
      for (key, map) in self.maps.iter() {
        if key.0 == "seed" {
          // TODO: You are handling conversions wrong, maps_to only knows
          //    what the right hand side of the key is anyways so returning dest_type
          //    is useless. You have to rescan the map for dest_type in the LHS key
          //    to actually push on the frontier.
          if let Some((dest, id)) = map.maps_to(seed) {
            frontier.push_back(("seed".to_string(), dest, id));
          }
        }
      }
    }

    // Do the actual BFS
    while let Some((src, dest, dest_id)) = frontier.pop_front() {
      if let Some((ndest, nid)) = self
        .maps
        .get(&(src, dest.clone()))
        .and_then(|m| m.maps_to(&dest_id))
      {
        if ndest == "location" {
          locations.insert(nid);
        } else {
          frontier.push_back((dest, ndest, nid));
        }
      }
    }

    Ok(Box::new(format!("{:?}", locations.iter().min())))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new("todo"))
  }
}
