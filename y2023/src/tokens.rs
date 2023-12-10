#![allow(dead_code)]
use std::{collections::VecDeque, marker::PhantomData, str::FromStr, fmt::Debug};

use tracing::debug;

impl FromParser<(String, usize)> for (String, usize) {
  fn from_parse(vec: Vec<ParseToken>) -> (String, usize) {
    if vec.len() != 2 {
      panic!("Parsed more than 2 tokens, can't make tuple2");
    }
    (vec[0].as_string(), vec[1].as_usize())
  }
}

impl FromParser<(String, String, String)> for (String, String, String) {
  fn from_parse(vec: Vec<ParseToken>) -> (String, String, String) {
    if vec.len() != 3 {
      panic!("Parsed more than 3 tokens, can't make tuple3");
    }
    (vec[0].as_string(), vec[1].as_string(), vec[2].as_string())
  }
}

impl FromParser<Vec<i64>> for Vec<i64> {
  fn from_parse(value: Vec<ParseToken>) -> Vec<i64> {
    value
      .iter()
      .flat_map(|v| match v {
        ParseToken::I64s(i) => i.to_owned(),
        ParseToken::I64(i) => vec![*i],
        _ => panic!("Attempted to collect non-i64 value"),
      })
      .collect()
  }
}

impl FromParser<Vec<usize>> for Vec<usize> {
  fn from_parse(value: Vec<ParseToken>) -> Vec<usize> {
    value
      .iter()
      .flat_map(|v| match v {
        ParseToken::Usizes(i) => i.to_owned(),
        ParseToken::Usize(i) => vec![*i],
        _ => panic!("Attempted to collect non-usize value"),
      })
      .collect()
  }
}

impl FromParser<(Vec<usize>, Vec<usize>)> for (Vec<usize>, Vec<usize>) {
  fn from_parse(value: Vec<ParseToken>) -> (Vec<usize>, Vec<usize>) {
    if value.len() != 2 {
      panic!("Parsed more than 2 tokens, can't make tuple2");
    }
    (value[0].as_usizes(), value[1].as_usizes())
  }
}

pub struct Parser {
  buffer: Vec<char>,
  index: usize,
}

impl Parser {
  pub fn new(buffer: &str) -> Self {
    Parser {
      buffer: buffer.chars().collect(),
      index: 0,
    }
  }

  pub fn lazy() -> LineParseBuilder {
    LineParseBuilder::new()
  }

  pub fn take_until_whitespace(&mut self) -> String {
    let mut s = String::new();
    while let Some(c) = self.buffer.get(self.index).filter(|c| !c.is_whitespace()) {
      s.push(*c);
      self.index += 1;
    }
    s
  }

  pub fn take_until_not_digit(&mut self) -> String {
    let mut s = String::new();
    while let Some(c) = self.buffer.get(self.index).filter(|c| c.is_digit(10) || **c == '-') {
      s.push(*c);
      self.index += 1;
    }
    s
  }
  
  pub fn take_until_newline(&mut self) -> Option<String> {
    let mut s = String::new();
    while let Some(c) = self.buffer.get(self.index).filter(|c| **c != '\n') {
      s.push(*c);
      self.index += 1;
    }
    self.index += 1;
    Some(s).filter(|s| !s.is_empty())
  }

  pub fn take_until(&mut self, token: &str) -> String {
    let mut to_find: VecDeque<char> = token.chars().collect();
    let mut buf = String::new();
    let mut s = String::new();
    while let Some(c) = self.buffer.get(self.index) {
      match to_find.pop_front() {
        None => return s,
        Some(f) if f == *c => {
          buf.push(f);
          self.index += 1;
          continue;
        }
        Some(_) => {
          to_find = token.chars().collect();
          s.push_str(&buf);
          buf = String::new();
          s.push(*c);
          self.index += 1;
        }
      }
    }
    s
  }

  pub fn consume(&mut self, token: &str) {
    self.take_until(token);
    debug!("Consumed: {:?} -> {:?}", token, &self.buffer[self.index..]);
  }

  pub fn consume_whitespace(&mut self) {
    while let Some(_) = self.buffer.get(self.index).filter(|c| c.is_whitespace()) {
      self.index += 1;
    }
    debug!("Consumed Whitespace -> {:?}", &self.buffer[self.index..]);
  }

  pub fn take_number<T: FromStr>(&mut self) -> Option<T> {
    self.consume_whitespace();
    self.take_until_not_digit().parse::<T>().ok()
  }

  pub fn numbers<T: FromStr + Debug>(&mut self) -> Vec<T> {
    let mut v = Vec::new();
    while let Some(n) = self.take_number::<T>() {
    debug!("Took num: {:?} -> {:?}", n, &self.buffer[self.index..]);
      v.push(n);
    }
    debug!("All Nums Consumed: {:?}",&self.buffer[self.index..]);
    v
  }

  pub fn from_chars<T: for<'a> From<&'a char>>(&mut self) -> Vec<T> {
    let mut v = Vec::new();
    while let Some(c) = self.buffer.get(self.index) {
      v.push(T::from(c));
      self.index += 1;
    }
    v
  }

  pub fn lines<T>(self, ops: &mut LineParseBuilder) -> LineParser<T>
  where
    T: FromParser<T>,
  {
    LineParser {
      parser: self,
      ops,
      _marker: PhantomData::<T>::default(),
    }
  }
}

pub trait FromParser<T> {
  fn from_parse(value: Vec<ParseToken>) -> T;
}

pub enum ParseToken {
  String(String),
  I64(i64),
  I64s(Vec<i64>),
  Usize(usize),
  Usizes(Vec<usize>),
}
impl ParseToken {
  pub fn as_string(&self) -> String {
    match self {
      ParseToken::String(s) => s.to_owned(),
      _ => panic!("Not a String type"),
    }
  }
  pub fn as_usize(&self) -> usize {
    match self {
      ParseToken::Usize(s) => s.to_owned(),
      _ => panic!("Not a usize type"),
    }
  }
  pub fn as_usizes(&self) -> Vec<usize> {
    match self {
      ParseToken::Usizes(s) => s.to_owned(),
      _ => panic!("Not a Vec<usize> type"),
    }
  }
  pub fn as_i64(&self) -> i64 {
    match self {
      ParseToken::I64(s) => s.to_owned(),
      _ => panic!("Not a i64 type"),
    }
  }
  pub fn as_i64s(&self) -> Vec<i64> {
    match self {
      ParseToken::I64s(s) => s.to_owned(),
      _ => panic!("Not a Vec<i64> type"),
    }
  }
}

#[derive(Clone)]
enum BldOps {
  TakeUntil(String),
  ConsumeWhitespace,
  TakeNonWhitespace,
  TakeI64,
  TakeI64s,
  TakeUSize,
  TakeUSizes,
  Consume(String),
}

pub struct LineParseBuilder {
  ops: Vec<BldOps>,
}

pub struct LineParser<'a, T>
where
  T: FromParser<T>,
{
  parser: Parser,
  ops: &'a mut LineParseBuilder,
  _marker: PhantomData<T>,
}

impl LineParseBuilder {
  pub fn new() -> Self {
    LineParseBuilder { ops: Vec::new() }
  }

  pub fn take_until(&mut self, token: &str) -> &mut Self {
    self.ops.push(BldOps::TakeUntil(token.to_string()));
    self
  }

  pub fn consume_whitespace(&mut self) -> &mut Self {
    self.ops.push(BldOps::ConsumeWhitespace);
    self
  }

  pub fn take_non_whitespace(&mut self) -> &mut Self {
    self.ops.push(BldOps::TakeNonWhitespace);
    self
  }

  pub fn take_i64(&mut self) -> &mut Self {
    self.ops.push(BldOps::TakeI64);
    self
  }

  pub fn take_i64s(&mut self) -> &mut Self {
    self.ops.push(BldOps::TakeI64s);
    self
  }

  pub fn take_usize(&mut self) -> &mut Self {
    self.ops.push(BldOps::TakeUSize);
    self
  }

  pub fn take_usizes(&mut self) -> &mut Self {
    self.ops.push(BldOps::TakeUSizes);
    self
  }

  pub fn consume(&mut self, token: &str) -> &mut Self {
    self.ops.push(BldOps::Consume(token.to_string()));
    self
  }

  pub fn apply<T>(&self, s: &str) -> T
  where
    T: FromParser<T>,
  {
    let mut p = Parser::new(&s);
    T::from_parse(
      self
        .ops
        .iter()
        .filter_map(|op| match op {
          BldOps::TakeUntil(t) => Some(ParseToken::String(p.take_until(t))),
          BldOps::TakeNonWhitespace => Some(ParseToken::String(p.take_until_whitespace())),
          BldOps::TakeI64 => Some(ParseToken::I64(p.take_number().unwrap())),
          BldOps::TakeI64s => Some(ParseToken::I64s(p.numbers())),
          BldOps::TakeUSize => Some(ParseToken::Usize(p.take_number().unwrap())),
          BldOps::TakeUSizes => Some(ParseToken::Usizes(p.numbers())),
          BldOps::ConsumeWhitespace => {
            p.consume_whitespace();
            None
          },
          BldOps::Consume(t) => {
            p.consume(t);
            None
          }
        })
        .collect(),
    )
  }
}

impl<'a, T> Iterator for LineParser<'a, T>
where
  T: FromParser<T>,
{
  type Item = T;
  fn next(&mut self) -> Option<Self::Item> {
    self.parser.take_until_newline().map(|l| self.ops.apply(&l))
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[derive(Debug, PartialEq, Eq)]
  enum TestFromChars {
    Left,
    Right,
  }

  impl From<&char> for TestFromChars {
    fn from(c: &char) -> Self {
      match c {
        'L' => TestFromChars::Left,
        'R' => TestFromChars::Right,
        _ => unreachable!(),
      }
    }
  }

  #[test]
  fn from_chars() {
    let mut p = Parser::new("LRRL");
    assert_eq!(
      p.from_chars::<TestFromChars>(),
      vec![
        TestFromChars::Left,
        TestFromChars::Right,
        TestFromChars::Right,
        TestFromChars::Left
      ]
    );
  }

  #[test]
  fn numbers() {
    let mut p = Parser::new("   1 20   4  601\n10\n\n3");
    assert_eq!(p.numbers::<i64>(), vec![1, 20, 4, 601, 10, 3]);
  }

  #[test]
  fn take_number() {
    let mut p = Parser::new("   1 20   4  601\n10\n\n3");
    assert_eq!(p.take_number::<i64>(), Some(1));
    assert_eq!(p.take_number::<i64>(), Some(20));
    assert_eq!(p.take_number::<i64>(), Some(4));
    assert_eq!(p.take_number::<i64>(), Some(601));
    assert_eq!(p.take_number::<i64>(), Some(10));
    assert_eq!(p.take_number::<i64>(), Some(3));
    assert_eq!(p.take_number::<i64>(), None);
  }
}
