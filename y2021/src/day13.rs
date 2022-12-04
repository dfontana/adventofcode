use itertools::Itertools;
use rust_util::{Day};
use std::{collections::HashMap, error::Error, fmt::Display};

type Coord = (usize, usize);
type Paper = HashMap<Coord, u8>;

#[derive(Debug)]
enum Fold {
  X(usize),
  Y(usize),
}

pub struct Solve {
  paper: Paper,
  folds: Vec<Fold>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(input: String) -> Result<Self, Self::Error> {
    let mut paper = HashMap::new();
    let mut folds = Vec::new();

    let mut foldstart = false;
    for line in input.lines() {
      if line.is_empty() {
        foldstart = true;
        continue;
      }

      if !foldstart {
        let mut coords = line.splitn(2, ",");
        let key = (
          coords.next().unwrap().parse::<usize>()?,
          coords.next().unwrap().parse::<usize>()?,
        );
        paper.insert(key, 1);
      } else {
        let mut fold = line.trim_start_matches("fold along ").splitn(2, "=");
        folds.push(match fold.next().unwrap() {
          "x" => Fold::X(fold.next().unwrap().parse::<usize>()?),
          "y" => Fold::Y(fold.next().unwrap().parse::<usize>()?),
          _ => unreachable!(),
        });
      }
    }
    Ok(Solve { paper, folds })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(fold_paper(&self.paper, &self.folds[0..1]).len()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let paper = fold_paper(&self.paper, &self.folds[..]);
    let max_x = paper.keys().map(|(x, _)| x).max().unwrap_or(&0);
    let output = paper
      .keys()
      .fold(HashMap::new(), |mut acc: HashMap<usize, String>, (x, y)| {
        acc
          .entry(*y)
          .and_modify(|v| v.replace_range(x..&(x + 1), "#"))
          .or_insert_with(|| {
            let mut ret = (0..*max_x + 1).map(|_| " ").collect::<String>();
            ret.replace_range(x..&(x + 1), "#");
            ret
          });
        acc
      })
      .iter()
      .sorted()
      .map(|(_, v)| v)
      .join("\n");
    Ok(Box::new(format!("\n{}", output)))
  }
}

fn fold_paper(inp: &Paper, folds: &[Fold]) -> Paper {
  let mut paper = inp.clone();
  for fold in folds {
    let mut new_paper = paper.clone();
    for coord in paper.keys() {
      match fold {
        Fold::Y(line) => {
          if &coord.1 < line {
            continue;
          }
          let new_y = line - (coord.1 - line);
          new_paper.insert((coord.0, new_y), 1);
          new_paper.remove(coord);
        }
        Fold::X(line) => {
          if &coord.0 < line {
            continue;
          }
          let new_x = line - (coord.0 - line);
          new_paper.insert((new_x, coord.1), 1);
          new_paper.remove(coord);
        }
      }
    }
    paper = new_paper;
  }
  paper
}
