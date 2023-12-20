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
    let mut finals = Vec::new();
    let mut frontier = vec![("in".to_string(), Ranges::default())];
    while let Some((ptr, ranges)) = frontier.pop() {
      let rules = self.workflows.get(&ptr).unwrap();

      let mut rng = ranges.clone();
      rng.append_path(ptr.clone());
      for rule in rules.0.iter() {
        match rule {
          Rule::Outcome(Outcome::Accepted) => finals.push(rng.clone()),
          Rule::Outcome(Outcome::Pointer(nxt)) => frontier.push((nxt.clone(), rng.clone())),
          Rule::Outcome(Outcome::Rejected) => { /* Trash it*/ }
          Rule::Condition(field, cnd, bound, out) => {
            let mut this_rng = rng.clone();
            match cnd {
              Condition::LT => {
                this_rng.update_high(field, *bound - 1);
                rng.update_low(field, *bound);
              }
              Condition::GT => {
                this_rng.update_low(field, *bound + 1);
                rng.update_high(field, *bound);
              }
            }
            if *out == Outcome::Accepted {
              finals.push(this_rng.clone());
            } else if let Outcome::Pointer(ptr) = out {
              frontier.push((ptr.to_string(), this_rng.clone()));
            }
          }
        }
      }
    }
    Ok(Box::new(finals.iter().map(|rg| rg.combos()).sum::<usize>()))
  }
}

#[derive(Clone, Debug)]
struct Ranges {
  x: (usize, usize),
  m: (usize, usize),
  a: (usize, usize),
  s: (usize, usize),
  path: Vec<String>,
}

impl Default for Ranges {
  fn default() -> Self {
    Ranges {
      x: (1, 4000),
      m: (1, 4000),
      a: (1, 4000),
      s: (1, 4000),
      path: vec![],
    }
  }
}

impl Ranges {
  fn update_high(&mut self, field: &Field, bound: usize) {
    match field {
      Field::X => self.x.1 = self.x.1.min(bound),
      Field::M => self.m.1 = self.m.1.min(bound),
      Field::A => self.a.1 = self.a.1.min(bound),
      Field::S => self.s.1 = self.s.1.min(bound),
    };
  }

  fn update_low(&mut self, field: &Field, bound: usize) {
    match field {
      Field::X => self.x.0 = self.x.0.max(bound),
      Field::M => self.m.0 = self.m.0.max(bound),
      Field::A => self.a.0 = self.a.0.max(bound),
      Field::S => self.s.0 = self.s.0.max(bound),
    };
  }

  fn combos(&self) -> usize {
    (self.x.1 - self.x.0 + 1)
      * (self.m.1 - self.m.0 + 1)
      * (self.a.1 - self.a.0 + 1)
      * (self.s.1 - self.s.0 + 1)
  }

  fn append_path(&mut self, ptr: String) {
    self.path.push(ptr);
  }
}
