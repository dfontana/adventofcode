use rust_util::{read_input, AocDay, Day};
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Display;

// A graph might be better suited to this problem, but modeling
// that in rust is surprisingly non-trivial. TIL.
pub struct Solve {
  formulas: Vec<Formula>,
}

#[derive(Debug, Clone)]
struct Bag {
  name: String,
  count: i32,
}

#[derive(Debug, Clone)]
struct Formula {
  result: Bag,
  parts: Vec<Bag>,
}

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    Ok(Box::new(Solve {
      formulas: read_input(2020, d)?
        .lines()
        .map(|l| {
          let mut lr = l.split("contain");
          Formula {
            result: Bag {
              count: 1,
              name: trim_bag_off(lr.next().unwrap()),
            },
            parts: lr
              .next()
              .unwrap()
              .split(",")
              .filter_map(pull_name_count)
              .map(|(count, name)| Bag { name, count })
              .collect(),
          }
        })
        .collect(),
    }))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
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
    Ok(Box::new(result.len().to_string()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let shiny_form = match self.formulas.iter().find(|f| f.result.name == "shiny gold") {
      None => return Err("Couldn't find shiny gold formula".into()),
      Some(form) => form,
    };
    let mut result = 0;
    let mut frontier: Vec<String> = shiny_form
      .parts
      .iter()
      .flat_map(|b| std::iter::repeat(b.name.clone()).take(b.count as usize))
      .collect();
    while !frontier.is_empty() {
      let exploring = match frontier.pop() {
        None => break,
        Some(b) => b,
      };
      for form in self.formulas.iter() {
        if form.result.name == exploring {
          form
            .parts
            .iter()
            .flat_map(|b| std::iter::repeat(b.clone()).take(b.count as usize))
            .for_each(|b| frontier.push(b.name));
        }
      }
      result += 1;
    }
    Ok(Box::new(result.to_string()))
  }

  fn run(&self) -> Result<String, Box<dyn Error>> {
    Ok(format!("Part1: {}\nPart2: {}", self.p1()?, self.p2()?))
  }
}

fn trim_bag_off(name: &str) -> String {
  name
    .trim()
    .trim_end_matches('.')
    .trim_end_matches("bags")
    .trim_end_matches("bag")
    .trim()
    .to_string()
}

fn pull_name_count(s: &str) -> Option<(i32, String)> {
  let clean = s.trim();
  let num_split = clean.find(" ").unwrap();
  let name = trim_bag_off(&clean[num_split..]);
  match (&clean[..num_split]).parse::<i32>() {
    Ok(count) => Some((count, name)),
    Err(_) => None,
  }
}
