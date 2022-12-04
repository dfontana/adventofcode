use regex::Regex;
use rust_util::{ Day};
use std::{collections::HashMap, error::Error, fmt::Display};

type Rules = HashMap<String, String>;

pub struct Solve {
  rules: Rules,
  messages: Vec<String>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(inp: String) -> Result<Self, Self::Error> {
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
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(solve(&self.rules, &self.messages).to_string()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let (mut ans, mut nxt) = (0, 1);
    let mut reps = 1;
    while ans != nxt {
      ans = nxt;
      let mut rules = self.rules.clone();
      rules.insert("8".to_owned(), replace_n(reps, "42 | 42 8", "8"));
      rules.insert("11".to_owned(), replace_n(reps, "42 31 | 42 11 31", "11"));
      nxt = solve(&rules, &self.messages);
      reps += 1;
    }
    Ok(Box::new(ans.to_string()))
  }
}

fn replace_n(times: u8, rule: &str, key: &str) -> String {
  (0..times)
    .fold(rule.to_owned(), |acc, _| acc.replace(key, rule))
    .replacen(&(" ".to_owned() + &key), "", 1)
}

fn solve(rules: &Rules, messages: &Vec<String>) -> usize {
  let re = format!("^{}$", expand(&rules, "0".to_owned()));
  let re = Regex::new(&re).unwrap();
  messages.iter().filter(|msg| re.is_match(msg)).count()
}

fn expand(rules: &Rules, rule: String) -> String {
  let tokens = rules.get(&rule).unwrap();
  match tokens.chars().next().unwrap().is_alphabetic() {
    true => tokens.to_owned(),
    false => {
      let re = tokens
        .split(" | ")
        .map(|branch| {
          branch
            .split_whitespace()
            .map(|rule| expand(&rules, rule.to_owned()))
            .collect::<String>()
            + "|"
        })
        .collect::<String>();
      format!("({})", &re[..re.len() - 1])
    }
  }
}
