use crate::day::{Day, DayArg};
use crate::util::read_input;
use regex::Regex;
use std::{collections::HashMap, error::Error, str::Lines};

type Rules = HashMap<String, String>;

pub struct Solve {
  rules: Rules,
  messages: Vec<String>,
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    let inp = read_input(d)?;
    let mut groups = inp.split("\n\n");
    Ok(Solve {
      rules: groups
        .next()
        .unwrap()
        .lines()
        .fold(HashMap::new(), |mut acc, rule| {
          let lhs = rule.split(':').next().unwrap().trim().to_owned();
          let rhs = rule.split(':').nth(1).unwrap().trim().replace('"', "");
          acc.insert(lhs, rhs);
          acc
        }),
      messages: groups.next().unwrap().lines().map(str::to_owned).collect(),
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    Ok(solve(&self.rules, &self.messages).to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok("Impl".to_string())
  }
}

fn solve(rules: &Rules, messages: &Vec<String>) -> usize {
  let re = format!("^{}$", expand(&rules, "0".to_owned()));
  let re = Regex::new(&re).unwrap();
  messages.iter().filter(|msg| re.is_match(msg)).count()
}

fn expand(rules: &Rules, rule: String) -> String {
  let tokens = rules
    .get(&rule)
    .unwrap_or_else(|| panic!("no such rule {}", rule));
  if !tokens.chars().next().unwrap().is_alphabetic() {
    let re = tokens
      .split(" | ")
      .map(|branch| {
        format!(
          "{}|",
          branch
            .split(' ')
            .map(|rule| expand(&rules, rule.to_owned()))
            .collect::<String>()
        )
      })
      .collect::<String>();
    format!("({})", &re[..re.len() - 1])
  } else {
    tokens.to_owned()
  }
}
