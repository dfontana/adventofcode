use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::error::Error;

#[derive(Clone, Debug, PartialEq)]
enum Token {
  Operator(Op),
  ExpStart,
  ExpEnd,
  Val(i32),
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
  value: i32,
}

impl Exp {
  pub fn new(items: &Vec<Exp>, op: &Op) -> Exp {
    Exp {
      operands: items.clone(),
      operator: op.clone(),
      value: 0,
    }
  }

  pub fn val(value: i32) -> Exp {
    Exp {
      operands: Vec::new(),
      operator: Op::Identity,
      value,
    }
  }
}

impl Default for Exp {
  fn default() -> Exp {
    Exp{operator: Op::Identity, operands: Vec::new(), value: 0}
  }
}

pub struct Solve {}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    let inp: Vec<i32> = read_input(d)?
      .lines()
      .map(|l| {
        let mut tokens = tokenize(l);
        // TODO tokenizing works, but parsing does not
        let expression = parse4(&mut tokens);
        let result = evaluate(&expression);
        println!("{:#?}\n\n{:?}", expression, result);
        result
      })
      .collect();
    Ok(Solve {})
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    Ok("Impl".to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok("Impl".to_string())
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
      v => Token::Val(v.trim().parse::<i32>().unwrap()),
    })
    .collect()
}

fn parse4(tokens: &[Token]) -> Exp {
  if tokens.is_empty() {
    return Exp::default();
  }
  if tokens[0] == Token::ExpStart {
    println!("Panic ==== {:?}\n", tokens);
    let end = tokens.iter().enumerate().find_map(|(idx, tk)| match tk {Token::ExpEnd => Some(idx), _ => None}).unwrap();

    println!("Sub >>>> {:?}\n", &tokens[1..end]);

    let sub = parse4(&tokens[1..end]);

    if let Some((first, rest)) = tokens[end+1..].split_first() {
      println!("First - {:?} ::: Rest - {:?}\n", first, rest);
      match first {
        Token::Operator(op) => {
          return Exp::new(&vec![sub, parse4(rest)], op)
        },
        _ => return sub,
      }
    }
    return sub;
  }

  let maybe = tokens.iter().enumerate().find_map(|(idx, tk)| match tk {Token::Operator(op) => Some((idx, op)), _ => None});
  let (idx, op) = match maybe {
    Some(v) => v,
    None => return match tokens[0] {
      Token::Val(v) => Exp::val(v),
      _ => Exp::default(),
    },
  };
  let (l, r) = tokens.split_at(idx);
  Exp::new(&vec![parse4(l), parse4(&r[1..])], op)
}

fn parse3(mut tokens: &[Token]) -> Exp {
  let mut operands = Vec::new();
  let mut operator = None;
  while let Some((first, rest)) = tokens.split_first() {
    tokens = rest.clone();
    match first {
      Token::ExpStart => operands.push(parse3(&tokens)),
      Token::Val(v) => operands.push(Exp::val(*v)),
      Token::Operator(op) => {
        match operator {
          None => (),
          Some(old_op) => {
            let mut exp_new = Exp::default();
            exp_new.operator = old_op;
            exp_new.operands = operands.clone();
            operands.clear();
            operands.push(exp_new);
          }
        }
        operator = Some(op.clone())
      }
      Token::ExpEnd => break,
    }
  }
  return Exp::new(&operands, &operator.unwrap());
}


fn evaluate(exp: &Exp) -> i32 {
  match exp.operator {
    Op::Identity => exp.value,
    Op::Add => exp.operands.iter().fold(0, |a, b| a + evaluate(b)),
    Op::Mul => exp.operands.iter().fold(1, |a, b| a * evaluate(b)),
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
