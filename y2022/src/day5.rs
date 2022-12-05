use regex::Regex;
use rust_util::Day;
use std::{collections::HashMap, error::Error, fmt::Display};

type CargoId = char;
type StackId = usize;
type Move = (usize, StackId, StackId);

pub struct Solve {
  moves: Vec<Move>,
  cargo: HashMap<StackId, Vec<CargoId>>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let Some((cargo, moves)) = value.split_once("\n\n") else {
            return Err("Could not split input".into());
        };

    let move_exp =
      Regex::new("move ([0-9]+) from ([0-9]+) to ([0-9]+)").expect("Regex did not parse");
    let parsed_moves = moves
      .lines()
      .filter_map(|l| move_exp.captures(l))
      .map(|c| {
        (
          c.get(1).unwrap().as_str().parse::<usize>().unwrap(),
          c.get(2).unwrap().as_str().parse::<usize>().unwrap(),
          c.get(3).unwrap().as_str().parse::<usize>().unwrap(),
        )
      })
      .collect::<Vec<Move>>();

    let mut parsed_cargo: HashMap<StackId, Vec<CargoId>> = HashMap::new();

    // Flip the map so it's easier to parse stacks in the right order
    let mut cargo_lines = cargo.lines().rev();

    // Init the keys
    cargo_lines
      .next()
      .unwrap()
      .chars()
      .filter(|c| *c != ' ')
      .filter_map(|c| c.to_digit(10))
      .for_each(|d| {
        parsed_cargo.entry(d as usize).or_default();
      });

    for line in cargo_lines {
      let mut idx = 0;
      let mut in_spaces = 0;
      for char in line.chars() {
        if char == ' ' {
          in_spaces += 1;
          if in_spaces % 4 == 0 {
            // We crossed open air on a stack
            idx += 1;
          }
          continue;
        }
        // We have left open air
        in_spaces = 0;

        // Now let's parse out a box
        if char == '[' || char == ']' {
          continue;
        }
        idx += 1;
        parsed_cargo.entry(idx).and_modify(|st| st.push(char));
      }
    }

    Ok(Solve {
      moves: parsed_moves,
      cargo: parsed_cargo,
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let fully_contained: usize = 0;
    Ok(Box::new(fully_contained))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let partly_contained: usize = 0;
    Ok(Box::new(partly_contained))
  }
}
