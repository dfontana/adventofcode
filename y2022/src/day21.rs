use rust_util::Day;
use std::{collections::HashMap, error::Error, fmt::Display};

type Id = String;

pub struct Solve {
  tree: HashMap<Id, Operand>,
}

enum Operand {
  Num(i64),
  Op(Operation),
}

struct Operation {
  left: Id,
  right: Id,
  operator: Operator,
}

enum Operator {
  Add,
  Mul,
  Sub,
  Div,
}

impl From<&str> for Operand {
  fn from(value: &str) -> Self {
    if let Ok(v) = value.parse::<i64>() {
      return Operand::Num(v);
    }

    let left = value[0..=3].to_string();
    let operator = Operator::from(&value[5..6]);
    let right = value[7..=10].to_string();
    Operand::Op(Operation {
      left,
      right,
      operator,
    })
  }
}

impl From<&str> for Operator {
  fn from(v: &str) -> Self {
    match v {
      "+" => Operator::Add,
      "-" => Operator::Sub,
      "/" => Operator::Div,
      "*" => Operator::Mul,
      _ => unreachable!("Found: {:?}", v),
    }
  }
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      tree: value
        .lines()
        .filter_map(|v| v.split_once(": "))
        .map(|(id, op_str)| (id.to_string(), Operand::from(op_str)))
        .collect(),
    })
  }
}

// Evaluate the given id recursively
fn evaluate(tree: &HashMap<Id, Operand>, id: &Id) -> i64 {
  let Operation {
    operator,
    left,
    right,
  } = match tree.get(id) {
    Some(Operand::Num(v)) => return *v,
    Some(Operand::Op(op)) => op,
    None => unreachable!(),
  };
  match operator {
    Operator::Add => evaluate(tree, left) + evaluate(tree, right),
    Operator::Mul => evaluate(tree, left) * evaluate(tree, right),
    Operator::Sub => evaluate(tree, left) - evaluate(tree, right),
    Operator::Div => evaluate(tree, left) / evaluate(tree, right),
  }
}

// Like evaluate but instead solves the given id relative to the given value
// by reverseing the operation to simplify for the humn variable
fn solve(tree: &HashMap<Id, Operand>, id: &Id, value: i64) -> i64 {
  if id == "humn" {
    return value;
  }
  let Operation {
    operator,
    left,
    right,
  } = match tree.get(id) {
    Some(Operand::Num(v)) => return *v,
    Some(Operand::Op(op)) => op,
    None => unreachable!(),
  };
  match operator {
    Operator::Add => match has_humn(tree, left) {
      true => solve(tree, left, value - evaluate(tree, right)),
      false => solve(tree, right, value - evaluate(tree, left)),
    },
    Operator::Mul => match has_humn(tree, left) {
      true => solve(tree, left, value / evaluate(tree, right)),
      false => solve(tree, right, value / evaluate(tree, left)),
    },
    Operator::Sub => match has_humn(tree, left) {
      true => solve(tree, left, value + evaluate(tree, right)),
      false => solve(tree, right, evaluate(tree, left) - value),
    },
    Operator::Div => match has_humn(tree, left) {
      true => solve(tree, left, value * evaluate(tree, right)),
      false => solve(tree, right, evaluate(tree, left) / value),
    },
  }
}

// Determine if "humn" is in the given node's sub-tree
fn has_humn(tree: &HashMap<Id, Operand>, id: &Id) -> bool {
  if id == "humn" {
    return true;
  }
  match tree.get(id) {
    Some(Operand::Num(_)) => false,
    Some(Operand::Op(Operation { left, right, .. })) => {
      has_humn(tree, left) || has_humn(tree, right)
    }
    None => unreachable!(),
  }
}

impl Day for Solve {
  // 142707821472432
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      evaluate(&self.tree, &"root".to_string()).to_string(),
    ))
  }

  // 3587647562851
  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let value = match self.tree.get("root").unwrap() {
      Operand::Num(_) => return Err("Not found".into()),
      Operand::Op(Operation { left, right, .. }) => match has_humn(&self.tree, left) {
        true => solve(&self.tree, left, evaluate(&self.tree, right)),
        false => solve(&self.tree, right, evaluate(&self.tree, left)),
      },
    };
    Ok(Box::new(value.to_string()))
  }
}
