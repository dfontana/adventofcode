use crate::day::{Day, DayArg};
use crate::util::read_input;
use regex::Regex;
use std::{collections::HashMap, error::Error};

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
    let mut ans = 0;
    let new_8 = "42 | 42 8";
    let new_11 = "42 31 | 42 11 31";
    let mut reps = 1;
    loop {
      let mut rules = self.rules.clone();
      rules.insert("8".to_owned(), replace_n(reps, new_8, "8"));
      rules.insert("11".to_owned(), replace_n(reps, new_11, "11"));
      let new_ans = solve(&rules, &self.messages);
      if ans == new_ans {
        break;
      }
      ans = new_ans;
      reps += 1;
    }
    Ok(ans.to_string())
  }
}

fn replace_n(times: u8, rule: &str, key: &str) -> String {
  let mut res = rule.to_owned();
  for _ in 0..times {
    res = res.replace(key, rule)
  }
  let rm_key = " ".to_owned() + &key;
  res = res.replacen(&rm_key, "", 1);
  res
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
