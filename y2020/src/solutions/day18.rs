use rust_util::{read_input, AocDay, Day};
use std::{error::Error, fmt::Display};

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
}

enum State {
  Val(i64),
  Exp(Op, Vec<State>),
}

struct Lexer {
  tokens: Vec<Token>,
}

pub struct Solve {
  input: String,
}

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    Ok(Box::new(Solve {
      input: read_input(2020, d)?,
    }))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let ans: i64 = self
      .input
      .lines()
      .map(Lexer::tokenize)
      .map(|mut lexer| {
        parse(&mut lexer, 0, &|op| match op {
          Op::Add => (1, 2),
          Op::Mul => (1, 2),
        })
      })
      .map(|exp| evaluate(&exp))
      .sum();
    Ok(Box::new(ans.to_string()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let ans: i64 = self
      .input
      .lines()
      .map(Lexer::tokenize)
      .map(|mut lexer| {
        parse(&mut lexer, 0, &|op| match op {
          Op::Add => (3, 4),
          Op::Mul => (1, 2),
        })
      })
      .map(|exp| evaluate(&exp))
      .sum();
    Ok(Box::new(ans.to_string()))
  }
}

impl Lexer {
  fn tokenize(inp: &str) -> Lexer {
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

    let mut tokens: Vec<Token> = split_with_delimiter(&['(', ')', '+', '*'], inp)
      .iter()
      .filter(|s| **s != " " && **s != "")
      .map(|token| match *token {
        "+" => Token::Operator(Op::Add),
        "*" => Token::Operator(Op::Mul),
        "(" => Token::ExpStart,
        ")" => Token::ExpEnd,
        v => Token::Val(v.trim().parse::<i64>().unwrap()),
      })
      .collect();
    tokens.reverse();
    Lexer { tokens }
  }
  fn next(&mut self) -> Option<Token> {
    self.tokens.pop()
  }
  fn peek(&mut self) -> Option<Token> {
    self.tokens.last().cloned()
  }
}

fn parse<F>(lexer: &mut Lexer, min_bp: u8, bp_fn: &F) -> State
where
  F: Fn(&Op) -> (u8, u8),
{
  let mut lhs = match lexer.next() {
    Some(Token::Val(it)) => State::Val(it),
    Some(Token::ExpStart) => {
      let lhs = parse(lexer, 0, bp_fn);
      assert_eq!(lexer.next(), Some(Token::ExpEnd));
      lhs
    }
    Some(t) => panic!("bad token: {:?}", t),
    None => panic!("Empty Expression given"),
  };

  loop {
    let op = match lexer.peek() {
      None => break,
      Some(Token::Operator(op)) => op,
      Some(Token::ExpEnd) => break,
      Some(t) => panic!("bad token: {:?}", t),
    };
    let (l_bp, r_bp) = bp_fn(&op);
    if l_bp < min_bp {
      break;
    }
    lexer.next();
    let rhs = parse(lexer, r_bp, bp_fn);
    lhs = State::Exp(op.clone(), vec![lhs, rhs]);
  }

  lhs
}

fn evaluate(exp: &State) -> i64 {
  match exp {
    State::Val(v) => *v,
    State::Exp(Op::Add, operands) => operands.iter().fold(0, |a, b| a + evaluate(b)),
    State::Exp(Op::Mul, operands) => operands.iter().fold(1, |a, b| a * evaluate(b)),
  }
}
