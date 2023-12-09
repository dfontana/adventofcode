use itertools::Itertools;
use rust_util::Day;
use std::{collections::HashMap, error::Error, fmt::Display};

#[derive(Debug)]
pub struct Solve {
  seeds: Vec<usize>,
  maps: HashMap<Category, Mapping>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Range {
  start: usize,
  end: usize,
}

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
  ranges: Vec<(Range, Range)>,
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
    let src_type = lines
      .next()
      .and_then(|s| s.strip_suffix(" map:"))
      .and_then(|s| s.split_once("-to-"))
      .map(|(s, _)| Category::from(s))
      .unwrap();
    Mapping {
      src_type,
      ranges: lines
        .filter_map(|s| {
          s.split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect_tuple()
        })
        .map(|(dest, src, size)| (Range::of_len(src, size), Range::of_len(dest, size)))
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

impl Range {
  fn new(start: usize, end: usize) -> Self {
    if start >= end {
      panic!("Zero length range requested");
    }
    Range { start, end }
  }
  fn of_len(start: usize, len: usize) -> Self {
    Range::new(start, start + len)
  }

  fn split(&self, src: &Range, dst: &Range) -> Option<Vec<Range>> {
    if src.end <= self.start || self.end <= src.start {
      return None;
    }
    let mut splits = Vec::new();

    let mut offset = 0;
    let mut start = self.start;
    if start < src.start {
      splits.push(Range::new(start, src.start));
      start = src.start;
    } else if src.start <= start {
      offset = self.start - src.start;
    }

    let mut end = self.end;
    if src.end < end {
      splits.push(Range::new(src.end, end));
      end = src.end;
    }

    splits.push(Range::of_len(dst.start + offset, end - start));
    Some(splits)
  }
}

impl Solve {
  fn find_min_loc_splits(&self, seeds: Vec<Range>) -> Option<usize> {
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
    .fold(seeds.clone(), |mut acc, cat| {
      let mapping = self.maps.get(cat).unwrap();
      let mut new_acc = Vec::new();
      while let Some(seed_rg) = acc.pop() {
        let mut res = mapping
          .ranges
          .iter()
          .find_map(|(src, dst)| seed_rg.split(src, dst))
          .unwrap_or_else(|| vec![seed_rg.clone()]);
        new_acc.push(res.pop().expect("Must have item"));
        for r in res {
          acc.push(r);
        }
      }
      new_acc
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
          .map(|s| Range::of_len(*s, 1))
          .collect_vec()
      )
    )))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let seed_ranges = self
      .seeds
      .chunks_exact(2)
      .map(|win| Range::of_len(win[0], win[1]))
      .collect_vec();

    Ok(Box::new(format!(
      "{:?}",
      self.find_min_loc_splits(seed_ranges)
    )))
  }
}
