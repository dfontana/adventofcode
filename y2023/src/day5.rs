use itertools::Itertools;
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
  seeds: Vec<Id>,
  maps: HashMap<(Type, Type), Mapping>,
}
impl Solve {
  fn maps_from(&self, typ: &Type) -> Vec<(Type, Type)> {
    self
      .maps
      .iter()
      .filter(|(k, _)| *k.0 == *typ)
      .map(|(k, _)| k.to_owned())
      .collect_vec()
  }
}

#[derive(Debug)]
struct Mapping {
  src_type: Type,
  dest_type: Type,
  ranges: Vec<(Range<Id>, Range<Id>)>,
}
impl Mapping {
  fn maps_to_id(&self, id: &Id) -> (Type, Id) {
    self
      .ranges
      .iter()
      .find(|(src, _)| src.contains(id))
      .map(|(src, dest)| (self.dest_type.clone(), dest.start + (id - src.start)))
      .unwrap_or((self.dest_type.clone(), *id))
  }
}

impl From<&str> for Mapping {
  fn from(value: &str) -> Self {
    let mut lines = value.lines();
    let (src_type, dest_type) = lines
      .next()
      .and_then(|s| s.strip_suffix(" map:"))
      .and_then(|s| s.split_once("-to-"))
      .map(|(s, d)| (s.to_string(), d.to_string()))
      .unwrap();
    Mapping {
      src_type,
      dest_type,
      ranges: lines
        .filter_map(|s| {
          s.split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect_tuple()
        })
        .map(|(dest, src, size)| {
          (
            Range {
              start: src,
              end: src + size,
            },
            Range {
              start: dest,
              end: dest + size,
            },
          )
        })
        .collect(),
    }
  }
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let mut groups = value.split("\n\n");
    Ok(Solve {
      seeds: groups
        .next()
        .and_then(|s| s.strip_prefix("seeds: "))
        .map(|s| s.split_whitespace().map(|n| n.parse::<usize>().unwrap()))
        .map(|v| v.collect())
        .unwrap(),
      maps: groups
        .map(Mapping::from)
        .map(|m| ((m.src_type.clone(), m.dest_type.clone()), m))
        .collect(),
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut locations: HashSet<Id> = HashSet::new();
    let mut frontier: VecDeque<(Type, Id)> = VecDeque::new();

    // Init the BFS
    for key in self.maps_from(&"seed".to_string()).iter() {
      let map = self.maps.get(&key).unwrap();
      for seed in self.seeds.iter() {
        frontier.push_back(map.maps_to_id(seed));
      }
    }

    // Do the actual BFS
    while let Some((src, id)) = frontier.pop_front() {
      for (dest, d_id) in self
        .maps_from(&src)
        .iter()
        .filter_map(|key| self.maps.get(key).map(|m| m.maps_to_id(&id)))
      {
        if dest == "location" {
          locations.insert(d_id);
        } else {
          frontier.push_back((dest, d_id));
        }
      }
    }

    Ok(Box::new(format!("{:?}", locations.iter().min())))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut locations: HashSet<Id> = HashSet::new();
    let mut frontier: VecDeque<(Type, Id)> = VecDeque::new();

    // Init the BFS
    let seed_ranges = self
      .seeds
      .windows(2)
      .map(|win| Range {
        start: win[0],
        end: win[1],
      })
      .collect_vec();

    println!("{:?}", seed_ranges);

    for rg in seed_ranges.iter() {
      for key in self.maps_from(&"seed".to_string()).iter() {
        let map = self.maps.get(&key).unwrap();
        for seed in rg.start..rg.end {
          frontier.push_back(map.maps_to_id(&seed));
        }
      }
    }

    println!("Seeds done: {:?}", frontier.len());

    // Do the actual BFS
    while let Some((src, id)) = frontier.pop_front() {
      for (dest, d_id) in self
        .maps_from(&src)
        .iter()
        .filter_map(|key| self.maps.get(key).map(|m| m.maps_to_id(&id)))
      {
        if dest == "location" {
          locations.insert(d_id);
        } else {
          frontier.push_back((dest, d_id));
        }
      }
    }

    Ok(Box::new(format!("{:?}", locations.iter().min())))
  }
}
