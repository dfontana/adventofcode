use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::collections::HashSet;
use std::error::Error;

pub struct Solve {
  formulas: Vec<Formula>,
}

#[derive(Debug)]
struct Bag {
  name: String,
  count: i32,
}

#[derive(Debug)]
struct Formula {
  result: Bag,
  parts: Vec<Bag>,
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    Ok(Solve {
      formulas: read_input(d)?
        .lines()
        .map(|l| {
          let mut lr = l.split("contain");
          Formula {
            result: Bag {
              count: 1,
              name: trim_bag_off(lr.next().unwrap().trim()),
            },
            parts: lr
              .next()
              .unwrap()
              .split(",")
              .filter_map(|e| {
                let clean = e.trim();
                let num_split = clean.find(" ").unwrap();
                let name = trim_bag_off(&clean[num_split..]);
                match (&clean[..num_split]).parse::<i32>() {
                  Ok(count) => Some(Bag { count, name }),
                  Err(_) => None,
                }
              })
              .collect(),
          }
        })
        .collect(),
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    // Given "shiny gold", how many colors eventually contain one?
    // Could probably represent this as a graph and be much more efficient
    //    - Start at shiny bag entry
    //    - Count parents
    //    - Traverse up each parent and count their parents.
    //    - continue until no more parents.
    let mut result: HashSet<String> = HashSet::new();
    let mut frontier: Vec<String> = vec!["shiny gold".to_string()];
    while !frontier.is_empty() {
      let exploring = match frontier.pop() {
        None => break,
        Some(name) => name,
      };
      for form in self.formulas.iter() {
        if form.parts.iter().any(|b| b.name == exploring) {
          result.insert(form.result.name.to_string());
          frontier.push(form.result.name.to_string());
        }
      }
    }
    Ok(result.len().to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok("Impl".to_string())
  }
}

fn trim_bag_off(name: &str) -> String {
  name
    .trim_end_matches('.')
    .trim_end_matches("bags")
    .trim_end_matches("bag")
    .trim()
    .to_string()
}
