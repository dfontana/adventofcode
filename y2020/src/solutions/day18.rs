use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::error::Error;

#[derive(Clone, Debug, PartialEq)]
enum Token {
  Operator(Op),
  ExpStart,
  ExpEnd,
  Val(i64),
}

#[derive(Clone, Debug, PartialEq)]
enum Op {
  Mul,
  Add,
  Identity,
}

#[derive(Clone, Debug)]
struct Exp {
  operator: Op,
  operands: Vec<Exp>,
  value: i64,
}

impl Exp {
  pub fn new(items: &Vec<Exp>, op: &Op) -> Exp {
    Exp {
      operands: items.clone(),
      operator: op.clone(),
      value: 0,
    }
  }

  pub fn val(value: i64) -> Exp {
    Exp {
      operands: Vec::new(),
      operator: Op::Identity,
      value,
    }
  }
}

impl Default for Exp {
  fn default() -> Exp {
    Exp {
      operator: Op::Identity,
      operands: Vec::new(),
      value: 0,
    }
  }
}

pub struct Solve {
  tokens: Vec<Vec<Token>>,
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    Ok(Solve {
      tokens: read_input(d)?.lines().map(tokenize).collect(),
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    let ans: i64 = self
      .tokens
      .iter()
      .map(|tks| {
        tks
          .iter()
          .rev()
          .map(|tk| match tk {
            Token::ExpStart => Token::ExpEnd,
            Token::ExpEnd => Token::ExpStart,
            _ => tk.clone(),
          })
          .collect::<Vec<Token>>()
      })
      .map(|tks| eval_expr_easy(&tks, 0).1)
      .sum();
    Ok(ans.to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    let ans: i64 = self
      .tokens
      .iter()
      .map(|tks| parse(&tks, 0).1)
      .map(|exp| evaluate(&exp))
      .sum();
    Ok(ans.to_string())
  }
}

fn tokenize(inp: &str) -> Vec<Token> {
  split_with_delimiter(&['(', ')', '+', '*'], inp)
    .iter()
    .filter(|s| **s != " " && **s != "")
    .map(|token| match *token {
      "+" => Token::Operator(Op::Add),
      "*" => Token::Operator(Op::Mul),
      "(" => Token::ExpStart,
      ")" => Token::ExpEnd,
      v => Token::Val(v.trim().parse::<i64>().unwrap()),
    })
    .collect()
}

fn parse(tokens: &[Token], idx: usize) -> (usize, Exp) {
  let (mut i, mut left) = parse_add(tokens, idx);
  while i < tokens.len() && tokens[i] == Token::Operator(Op::Mul) {
    let next_group = parse_add(tokens, i + 1);
    i = next_group.0;
    left = Exp::new(&vec![left, next_group.1], &Op::Mul);
  }
  (i, left)
}

fn parse_add(tokens: &[Token], idx: usize) -> (usize, Exp) {
  let (mut i, mut left) = parse_val(tokens, idx);
  while i < tokens.len() && tokens[i] == Token::Operator(Op::Add) {
    let next_group = parse_val(tokens, i + 1);
    i = next_group.0;
    left = Exp::new(&vec![left, next_group.1], &Op::Add);
  }
  (i, left)
}

fn parse_val(tokens: &[Token], idx: usize) -> (usize, Exp) {
  match tokens[idx] {
    Token::Val(v) => (idx + 1, Exp::val(v)),
    Token::ExpStart => {
      let (i, val) = parse(tokens, idx + 1);
      match tokens[i] {
        Token::ExpEnd => (i + 1, val),
        _ => panic!("Parser Error - Non-end token Hit"),
      }
    }
    _ => panic!("Parser Error - Operator Or End Hit"),
  }
}

fn evaluate(exp: &Exp) -> i64 {
  match exp.operator {
    Op::Identity => exp.value,
    Op::Add => exp.operands.iter().fold(0, |a, b| a + evaluate(b)),
    Op::Mul => exp.operands.iter().fold(1, |a, b| a * evaluate(b)),
  }
}

fn eval_expr_easy(tokens: &[Token], i: usize) -> (usize, i64) {
  let (i, lhs) = match tokens[i] {
    Token::Val(n) => (i + 1, n),
    Token::ExpStart => eval_expr_easy(tokens, i + 1),
    _ => panic!(format!("Malformed; got {:?} at left hand side", tokens[i])),
  };
  if i == tokens.len() {
    return (i, lhs);
  }
  match tokens[i] {
    Token::Operator(Op::Add) => {
      let (i, rhs) = eval_expr_easy(tokens, i + 1);
      (i, lhs + rhs)
    }
    Token::Operator(Op::Mul) => {
      let (i, rhs) = eval_expr_easy(tokens, i + 1);
      (i, lhs * rhs)
    }
    Token::ExpEnd => (i + 1, lhs),
    _ => panic!(format!(
      "Malformed; got {:?} after left hand side",
      tokens[i]
    )),
  }
}

fn split_with_delimiter<'a>(pattern: &[char], inp: &'a str) -> Vec<&'a str> {
  let mut result = Vec::new();
  let mut last = 0;
  for (index, matched) in inp.match_indices(&pattern[..]) {
    if last != index {
      result.push(&inp[last..index]);
    }
    result.push(matched);
    last = index + matched.len();
  }
  if last < inp.len() {
    result.push(&inp[last..]);
  }
  result
}
