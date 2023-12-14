use rust_util::Day;
use std::{collections::HashMap, error::Error, fmt::Display};

#[derive(Debug)]
pub struct Solve {
  input: Vec<(Vec<SpaState>, Vec<usize>)>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum SpaState {
  OK,
  BR,
  UK,
}

impl From<char> for SpaState {
  fn from(value: char) -> Self {
    match value {
      '.' => SpaState::OK,
      '#' => SpaState::BR,
      '?' => SpaState::UK,
      _ => unreachable!(),
    }
  }
}

fn parse_spa(s: &str) -> (Vec<SpaState>, Vec<usize>) {
  let (spa, grp) = s.split_once(' ').unwrap();
  (
    spa.chars().map(SpaState::from).collect(),
    grp
      .split(',')
      .map(|c| c.parse::<usize>().unwrap())
      .collect(),
  )
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      input: value.lines().map(parse_spa).collect(),
    })
  }
}

fn combos(
  spas: Vec<SpaState>,
  grps: Vec<usize>,
  cache: &mut HashMap<(Vec<SpaState>, Vec<usize>), usize>,
) -> usize {
  let key = (spas.clone(), grps.clone());
  if let Some(v) = cache.get(&key) {
    return *v;
  }
  let mut nspa = spas.clone();
  let mut i = 0;
  while i < nspa.len() {
    if nspa[i] == SpaState::OK {
      i += 1;
    } else {
      break;
    }
  }
  nspa = nspa[i..].to_vec();

  if nspa.is_empty() {
    let ret = if grps.is_empty() { 1 } else { 0 };
    cache.insert(key, ret);
    return ret;
  }

  if grps.is_empty() {
    let ret = if !nspa.contains(&SpaState::BR) { 1 } else { 0 };
    cache.insert(key, ret);
    return ret;
  }

  if nspa[0] == SpaState::BR {
    if nspa.len() < grps[0] || nspa[..grps[0]].contains(&SpaState::OK) {
      cache.insert(key, 0);
      return 0;
    } else if nspa.len() == grps[0] {
      let ret = if grps.len() == 1 { 1 } else { 0 };
      cache.insert(key, ret);
      return ret;
    } else if nspa[grps[0]] == SpaState::BR {
      cache.insert(key, 0);
      return 0;
    } else {
      let ret = combos(nspa[grps[0] + 1..].to_vec(), grps[1..].to_vec(), cache);
      cache.insert(key, ret);
      return ret;
    }
  }

  let mut updated = nspa[1..].to_vec();
  updated.insert(0, SpaState::BR);
  let ret = combos(updated, grps.clone(), cache) + combos(nspa[1..].to_vec(), grps, cache);
  cache.insert(key, ret);
  return ret;
}

fn five_x((spas, grps): &(Vec<SpaState>, Vec<usize>)) -> (Vec<SpaState>, Vec<usize>) {
  let mut nspa = Vec::new();
  let mut ngrp = Vec::new();
  for i in 1..=5 {
    for s in spas.iter() {
      nspa.push(s.clone());
    }
    for g in grps.iter() {
      ngrp.push(*g);
    }
    if i != 5 {
      nspa.push(SpaState::UK);
    }
  }
  (nspa, ngrp)
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut cache = HashMap::new();
    Ok(Box::new(
      self
        .input
        .iter()
        .map(|(s, g)| combos(s.clone(), g.clone(), &mut cache))
        .sum::<usize>(),
    ))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut cache = HashMap::new();
    Ok(Box::new(
      self
        .input
        .iter()
        .map(five_x)
        .map(|(s, g)| combos(s.clone(), g.clone(), &mut cache))
        .sum::<usize>(),
    ))
  }
}
