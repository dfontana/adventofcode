use itertools::Itertools;
use rust_util::Day;
use std::{error::Error, fmt::Display};

pub struct Solve {
  input: Vec<String>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      input: value.lines().map(|s| s.to_owned()).collect_vec(),
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .input
        .iter()
        .filter_map(|s| {
          let nums = s.chars().filter(|x| x.is_digit(10)).collect::<Vec<char>>();

          let tupe = match nums.len() {
            1 => (nums[0], nums[0]),
            x if x > 1 => (nums[0], nums[nums.len() - 1]),
            _ => return None,
          };
          return Some(tupe);
        })
        .map(|(a, b)| format!("{}{}", a, b).parse::<u32>().unwrap())
        .sum::<u32>(),
    ))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .input
        .iter()
        .map(|s| {
          return parse_nums(s);
        })
        .map(|(a, b)| format!("{}{}", a, b).parse::<u32>().unwrap())
        .sum::<u32>(),
    ))
  }
}

fn parse_nums(s: &String) -> (String, String) {
  let repl = vec![
    ("1", "1"),
    ("2", "2"),
    ("3", "3"),
    ("4", "4"),
    ("5", "5"),
    ("6", "6"),
    ("7", "7"),
    ("8", "8"),
    ("9", "9"),
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
  ];
  // Replace first instance, and then replace last instance
  if let Some((p, r, _)) = repl
    .iter()
    .filter_map(|(p, r)| s.find(p).map(|i| (p, r, i)))
    .min_by_key(|(_, _, i)| *i)
  {
    let news = s.replacen(p, r, 1);

    if let Some((_, r2, _)) = repl
      .iter()
      .filter_map(|(p2, r2)| news.rfind(p2).map(|i2| (p2, r2, i2)))
      .max_by_key(|(_, _, i2)| *i2)
    {
      return (r.to_string(), r2.to_string());
    }
  }
  unreachable!()
}
