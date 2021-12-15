use itertools::{Itertools, MinMaxResult};
use rust_util::{AocDay, Day};
use std::{collections::HashMap, error::Error, fmt::Display};

type Element = char;
type Polymer = (Element, Element);
type PairRule = (Polymer, Element);
type Aggregate = (f64, f64, f64); // Multiplier, Value A, Value B
type Template = HashMap<Polymer, Aggregate>;

pub struct Solve {
  template: Template,
  rules: Vec<PairRule>,
}

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    let mut template: Template = HashMap::new();
    let mut rules: Vec<PairRule> = Vec::new();

    let input = rust_util::read_input(2021, d)?;
    let mut onto_transforms = false;
    for line in input.lines() {
      if line.is_empty() {
        onto_transforms = true;
        continue;
      }
      if onto_transforms {
        let mut trs = line.splitn(2, " -> ");
        let mut poly = trs.next().unwrap().chars();
        rules.push((
          (poly.nth(0).unwrap(), poly.nth(0).unwrap()),
          trs.next().and_then(|s| s.chars().nth(0)).unwrap(),
        ));
      } else {
        template =
          line
            .chars()
            .into_iter()
            .tuple_windows()
            .fold(HashMap::new(), |mut acc, (a, b)| {
              acc
                .entry((a, b))
                .and_modify(|v| {
                  v.0 += 1.0;
                  v.1 += 0.5;
                  v.2 += 0.5;
                })
                .or_insert((1.0, 0.5, 0.5));
              acc
            });
      }
    }
    Ok(Box::new(Solve { template, rules }))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let (min, max) = expand(&self.template, &self.rules, 10);
    Ok(Box::new(max - min))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let (min, max) = expand(&self.template, &self.rules, 40);
    Ok(Box::new(max - min))
  }
}

// There's a bug in here somewhere; if the ends of the starting template
// are the same, we over count that character by 1. If they differ, then 
// we under count it by 0.5. I'm not quite sure why...  
fn expand(base: &Template, rules: &Vec<PairRule>, steps: usize) -> (f64, f64) {
  let template = (0..steps).fold(base.clone(), |acc, _| apply(&acc, &rules));
  let summed = template
    .iter()
    .fold(HashMap::new(), |mut acc, ((a, b), (_, aw, bw))| {
      acc.entry(a).and_modify(|v| *v += *aw).or_insert(*aw);
      acc.entry(b).and_modify(|v| *v += *bw).or_insert(*bw);
      acc
    });
  let minmax = summed.iter().minmax_by_key(|f| f.1);
  let (min, max) = match minmax {
    MinMaxResult::MinMax(min, max) => (min, max),
    _ => unreachable!(),
  };
  println!("{:?}-{:?}", max, min);
  (*min.1, *max.1)
}

fn apply(base: &Template, rules: &Vec<PairRule>) -> Template {
  let mut adds: Template = HashMap::new();
  let mut removes: Vec<Polymer> = Vec::new();

  for (pair, insert) in rules {
    match base.get(pair) {
      None => continue,
      Some(agg) => {
        removes.push(*pair);
        let multi = agg.0 * 0.5;
        adds
          .entry((pair.0, *insert))
          .and_modify(|v| {
            v.0 += agg.0;
            v.1 += multi;
            v.2 += multi;
          })
          .or_insert((agg.0, multi, multi));
        adds
          .entry((*insert, pair.1))
          .and_modify(|v| {
            v.0 += agg.0;
            v.1 += multi;
            v.2 += multi;
          })
          .or_insert((agg.0, multi, multi));
      }
    }
  }
  let mut template = base.clone();
  removes.iter().for_each(|p| {
    template.remove(p);
  });
  adds.iter().for_each(|(k, agg)| {
    template
      .entry(*k)
      .and_modify(|v| {
        v.0 += agg.0;
        v.1 += agg.1;
        v.2 += agg.2;
      })
      .or_insert((agg.0, agg.1, agg.2));
  });
  template
}
