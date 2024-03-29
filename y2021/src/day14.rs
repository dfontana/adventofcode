use itertools::{Itertools, MinMaxResult};
use rust_util::{Day};
use std::{collections::HashMap, error::Error, fmt::Display};

type Element = char;
type Polymer = (Element, Element);
type PairRule = (Polymer, Element);
type Aggregate = (f64, f64, f64); // Multiplier, Value A, Value B
type Template = HashMap<Polymer, Aggregate>;

pub struct Solve {
  template: Template,
  ends: (Element, Element),
  rules: Vec<PairRule>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(input: String) -> Result<Self, Self::Error> {
    let mut template: Template = HashMap::new();
    let mut rules: Vec<PairRule> = Vec::new();
    let mut ends: (Element, Element) = (' ', ' ');
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
        ends = (
          line.chars().nth(0).unwrap(),
          line.chars().nth(line.len() - 1).unwrap(),
        );
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
    Ok(Solve {
      template,
      ends,
      rules,
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let (min, max) = expand(&self.template, &self.rules, &self.ends, 10);
    Ok(Box::new(max - min))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let (min, max) = expand(&self.template, &self.rules, &self.ends, 40);
    Ok(Box::new(max - min))
  }
}

fn expand(
  base: &Template,
  rules: &Vec<PairRule>,
  ends: &(Element, Element),
  steps: usize,
) -> (f64, f64) {
  let template = (0..steps).fold(base.clone(), |acc, _| apply(&acc, &rules));
  let mut summed = template
    .iter()
    .fold(HashMap::new(), |mut acc, ((a, b), (_, aw, bw))| {
      acc.entry(a).and_modify(|v| *v += *aw).or_insert(*aw);
      acc.entry(b).and_modify(|v| *v += *bw).or_insert(*bw);
      acc
    });
  // Rectify we only treated the ends as worth 0.5 instead of 1.
  summed.entry(&ends.0).and_modify(|v| *v += 0.5);
  summed.entry(&ends.1).and_modify(|v| *v += 0.5);
  let minmax = summed.iter().minmax_by_key(|f| f.1);
  let (min, max) = match minmax {
    MinMaxResult::MinMax(min, max) => (min, max),
    _ => unreachable!(),
  };
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
