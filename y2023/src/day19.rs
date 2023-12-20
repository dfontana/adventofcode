use rust_util::Day;
use std::{collections::HashMap, error::Error, fmt::Display};

pub struct Solve {
  parts: Vec<Part>,
  workflows: HashMap<String, Workflow>,
}

struct Workflow(Vec<Rule>);

#[derive(Clone, Debug)]
enum Rule {
  Condition(Field, Condition, usize, Outcome),
  Outcome(Outcome),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Outcome {
  Accepted,
  Rejected,
  Pointer(String),
}

#[derive(Clone, Debug)]
enum Condition {
  LT,
  GT,
}

#[derive(Clone, Debug)]
enum Field {
  X,
  M,
  A,
  S,
}

#[derive(Default, Debug, Clone)]
struct Part {
  x: usize,
  m: usize,
  a: usize,
  s: usize,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let (workflows, parts) = value.split_once("\n\n").unwrap();

    Ok(Solve {
      workflows: workflows
        .lines()
        .filter_map(|s| s.split_once('{'))
        .map(|(id, flw)| (id.to_string(), Workflow::from(flw)))
        .collect(),
      parts: parts.lines().map(Part::from).collect(),
    })
  }
}

impl From<&str> for Outcome {
  fn from(value: &str) -> Self {
    match value {
      "A" => Outcome::Accepted,
      "R" => Outcome::Rejected,
      ptr => Outcome::Pointer(ptr.to_string()),
    }
  }
}

impl From<&str> for Rule {
  fn from(value: &str) -> Self {
    if value.contains(':') {
      let (lhs, rhs) = value.split_once(':').unwrap();
      let ((f, a), cnd) = match lhs.contains('<') {
        true => (lhs.split_once('<').unwrap(), Condition::LT),
        false => (lhs.split_once('>').unwrap(), Condition::GT),
      };
      return Rule::Condition(
        Field::from(f),
        cnd,
        a.parse::<usize>().unwrap(),
        Outcome::from(rhs),
      );
    }
    Rule::Outcome(Outcome::from(value))
  }
}

impl From<&str> for Workflow {
  fn from(value: &str) -> Self {
    Workflow(
      value
        .trim_end_matches('}')
        .split(',')
        .map(Rule::from)
        .collect(),
    )
  }
}

impl From<&str> for Part {
  fn from(value: &str) -> Self {
    let mut res = Part::default();
    value
      .trim_start_matches('{')
      .trim_end_matches('}')
      .split(',')
      .filter_map(|s| s.split_once('='))
      .for_each(|(f, amt)| {
        let pamt = amt.parse::<usize>().unwrap();
        match f {
          "x" => res.x = pamt,
          "m" => res.m = pamt,
          "a" => res.a = pamt,
          "s" => res.s = pamt,
          _ => unreachable!(),
        };
      });
    res
  }
}

impl From<&str> for Field {
  fn from(value: &str) -> Self {
    match value.to_uppercase().as_str() {
      "X" => Field::X,
      "M" => Field::M,
      "A" => Field::A,
      "S" => Field::S,
      _ => unreachable!(),
    }
  }
}

impl Part {
  fn get(&self, field: &Field) -> usize {
    match field {
      Field::X => self.x,
      Field::M => self.m,
      Field::A => self.a,
      Field::S => self.s,
    }
  }

  fn sum(&self) -> usize {
    self.x + self.m + self.a + self.s
  }
}

impl Rule {
  fn apply(&self, part: &Part) -> Option<Outcome> {
    match self {
      Rule::Condition(f, cnd, amt, o) => match (cnd, part.get(f)) {
        (Condition::LT, v) if v < *amt => Some(o.clone()),
        (Condition::GT, v) if v > *amt => Some(o.clone()),
        _ => None,
      },
      Rule::Outcome(o) => Some(o.clone()),
    }
  }
}

impl Workflow {
  fn apply(&self, part: &Part) -> Option<Outcome> {
    self.0.iter().find_map(|r| r.apply(part))
  }
}

impl Solve {
  fn evaluate(&self, part: &Part) -> Outcome {
    let mut wrkflw = self.workflows.get("in").unwrap();
    while let Some(outcome) = wrkflw.apply(part) {
      wrkflw = match outcome {
        Outcome::Accepted | Outcome::Rejected => return outcome,
        Outcome::Pointer(p) => self.workflows.get(&p).unwrap(),
      };
    }
    unreachable!("Workflow was not terminal")
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .parts
        .iter()
        .filter(|p| self.evaluate(p) == Outcome::Accepted)
        .map(Part::sum)
        .sum::<usize>(),
    ))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(1))
  }
}
