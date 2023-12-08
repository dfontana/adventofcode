use itertools::Itertools;
use rayon::prelude::*;
use rust_util::Day;
use std::{collections::HashMap, error::Error, fmt::Display, ops::Range};

#[derive(Debug)]
pub struct Solve {
  seeds: Vec<Id>,
  maps: HashMap<Category, Mapping>,
}

type Id = usize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Category {
  SEED,
  SOIL,
  FERTILIZER,
  WATER,
  LIGHT,
  TEMPERATURE,
  HUMIDITY,
  LOCATION,
}

#[derive(Debug)]
struct Mapping {
  src_type: Category,
  dest_type: Category,
  ranges: Vec<(Range<Id>, Range<Id>)>,
}

impl From<&str> for Category {
  fn from(s: &str) -> Self {
    match s {
      "seed" => Self::SEED,
      "soil" => Self::SOIL,
      "fertilizer" => Self::FERTILIZER,
      "water" => Self::WATER,
      "light" => Self::LIGHT,
      "temperature" => Self::TEMPERATURE,
      "humidity" => Self::HUMIDITY,
      "location" => Self::LOCATION,
      _ => unreachable!(),
    }
  }
}
impl From<&str> for Mapping {
  fn from(value: &str) -> Self {
    let mut lines = value.lines();
    let (src_type, dest_type) = lines
      .next()
      .and_then(|s| s.strip_suffix(" map:"))
      .and_then(|s| s.split_once("-to-"))
      .map(|(s, d)| (Category::from(s), Category::from(d)))
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
      maps: groups.map(Mapping::from).map(|m| (m.src_type, m)).collect(),
    })
  }
}

impl Mapping {
  fn maps_to_id(&self, id: &Id) -> (Category, Id) {
    self
      .ranges
      .iter()
      .find(|(src, _)| src.contains(id))
      .map(|(src, dest)| (self.dest_type, dest.start + (id - src.start)))
      .unwrap_or((self.dest_type, *id))
  }
}

impl Solve {
  fn find_min_loc<'a, I>(&self, rg: I) -> usize
  where
    I: Iterator<Item = usize>,
  {
    let mut min_loc: Id = usize::MAX;
    let starter = self.maps.get(&Category::SEED).unwrap();
    for seed in rg {
      let mut pair = Some(starter.maps_to_id(&seed));
      while let Some((src, id)) = pair {
        if src == Category::LOCATION {
          min_loc = min_loc.min(id);
          break;
        }
        pair = self.maps.get(&src).map(|map| map.maps_to_id(&id));
      }
    }
    min_loc
  }

  // TODO: This isn't working yet, something is wrong with how splitting is handled not sure what tho
  // Specifically that is the ss < ds or se > de cases, as part 1 doesn't exercise those two code paths
  // P1: 662197086
  // P2: 52510809
  fn find_min_loc_splits(&self, seeds: Vec<Range<Id>>) -> Option<usize> {
    [
      Category::SEED,
      Category::SOIL,
      Category::FERTILIZER,
      Category::WATER,
      Category::LIGHT,
      Category::TEMPERATURE,
      Category::HUMIDITY,
    ]
    .iter()
    .fold(seeds.clone(), |acc, cat| {
      let mapping = self.maps.get(cat).unwrap();
      acc
        .iter()
        .flat_map(|seed_rg: &Range<usize>| {
          mapping
            .ranges
            .iter()
            .find(|(src, _)| src.contains(&seed_rg.start) || src.contains(&(seed_rg.end - 1)))
            .map(|(src, dst)| {
              let (ss, se) = (seed_rg.start, seed_rg.end);
              let (ds, de) = (src.start, src.end);

              let mut splits = Vec::new();
              let mut os = seed_rg.start;
              if ss < ds {
                splits.push(Range {
                  start: seed_rg.start,
                  end: src.end,
                });
                os = src.start;
              }
              let mut oe = seed_rg.end;
              if se > de {
                splits.push(Range {
                  start: src.end,
                  end: seed_rg.end,
                });
                oe = src.end;
              }
              splits.push(Range {
                start: dst.start + (os - src.start),
                end: dst.start + (oe - src.start),
              });
              splits
            })
            .unwrap_or_else(|| vec![seed_rg.clone()])
        })
        .collect_vec()
    })
    .iter()
    .map(|rg| rg.start)
    .min()
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(format!(
      "{:?}",
      self.find_min_loc_splits(
        self
          .seeds
          .iter()
          .map(|s| Range {
            start: *s,
            end: s + 1
          })
          .collect_vec()
      )
    )))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let seed_ranges = self
      .seeds
      .chunks_exact(2)
      .map(|win| Range {
        start: win[0],
        end: win[0] + win[1],
      })
      .collect_vec();

    Ok(Box::new(format!(
      "{:?}",
      self.find_min_loc_splits(seed_ranges)
    )))
  }
}
