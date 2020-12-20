use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::{collections::HashMap, error::Error};

#[derive(Debug)]
enum Rule {
  Char(String),
  Groups(Vec<Vec<i32>>),
}

pub struct Solve {
  rules: HashMap<i32, Rule>,
  messages: Vec<String>,
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    let inp = read_input(d)?;
    let mut groups = inp.splitn(2, "\n\n");
    Ok(Solve {
      rules: groups
        .next()
        .unwrap()
        .lines()
        .map(parse_rule)
        .fold(HashMap::new(), |mut acc, (id, r)| {
          acc.insert(id, r);
          acc
        }),
      messages: groups.next().unwrap().lines().map(str::to_owned).collect(),
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    let ans = self.messages
      .iter()
      .filter(|s| {
        if let Some(v) = matches_rule(&0, &self.rules, &s) {
          v.len() == 0
        } else {
          false
        }
      })
      .count();
    Ok(ans.to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok("Impl".to_string())
  }
}

fn matches_rule(id: &i32, rules: &HashMap<i32, Rule>, input: &String) -> Option<String> {
  if input.len() == 0 {
    return None;
  }
  match rules.get(id).unwrap() {
    Rule::Char(value) => {
      if input[0..1] == *value {
        return Some(input[1..].to_owned());
      } else {
        return None;
      }
    }
    Rule::Groups(choices) => {
      for sequence in choices {
        let mut current = input.to_owned();
        let mut valid = true;
        for id2 in sequence {
          match matches_rule(id2, rules, &current) {
            Some(v) => current = v,
            None => {
              valid = false;
              break;
            }
          }
        }
        if valid {
          return Some(current.to_owned());
        }
      }
      return None;
    }
  }
}

fn parse_rule(l: &str) -> (i32, Rule) {
  let mut i = 0;
  let mut in_id = true;
  let mut in_char = false;
  let mut in_group = false;
  let mut id = String::new();
  let mut chr = String::new();
  let mut grp_item = String::new();
  let mut groups = Vec::new();
  let mut group = Vec::new();
  loop {
    let ch = match l.get(i..i + 1) {
      None => break,
      Some(c) => c,
    };

    // Start in Id
    if in_id {
      if ch == ":" {
        in_id = false;
        i += 1;
        continue;
      }
      id += ch;
    }

    // Then comes either sub-rule or char
    if in_char {
      if ch == "\"" {
        in_char = false;
        i += 1;
        continue;
      }
      chr += ch;
    }

    if in_group {
      if ch == "|" {
        groups.push(group.clone());
        group.clear();
        in_group = false;
        i += 1;
        continue;
      }
      if ch == " " {
        group.push(grp_item.parse::<i32>().unwrap());
        grp_item.clear();
        i += 1;
        continue;
      }
    }

    // State transitions
    match ch {
      "\"" if !in_id && !in_char => in_char = true,
      c if !in_id && c.chars().nth(0).filter(|c| c.is_numeric()).is_some() => {
        in_group = true;
        grp_item += c;
      }
      _ => (),
    }
    i += 1;
  }

  // Terminate / Clean up
  if in_group {
    group.push(grp_item.parse::<i32>().unwrap());
    groups.push(group);
  }
  match chr.is_empty() {
    true => (id.parse::<i32>().unwrap(), Rule::Char(chr)),
    false => (id.parse::<i32>().unwrap(), Rule::Groups(groups))
  }
}
