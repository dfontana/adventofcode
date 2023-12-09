use std::{collections::VecDeque, marker::PhantomData, str::FromStr};

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

  fn take_until_whitespace(&mut self) -> String {
    let mut s = String::new();
    while let Some(_) = self.buffer.get(self.index).filter(|c| c.is_whitespace()) {
      self.index += 1;
    }
    while let Some(c) = self.buffer.get(self.index).filter(|c| !c.is_whitespace()) {
      s.push(*c);
      self.index += 1;
    }
    while let Some(_) = self.buffer.get(self.index).filter(|c| c.is_whitespace()) {
      self.index += 1;
    }
    s
  }

  fn take_until_newline(&mut self) -> Option<String> {
    let mut s = String::new();
    while let Some(c) = self.buffer.get(self.index).filter(|c| **c != '\n') {
      s.push(*c);
      self.index += 1;
    }
    Some(s).filter(String::is_empty)
  }

  fn take_until(&mut self, token: &str) -> String {
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

  fn consume(&mut self, token: &str) {
    self.take_until(token);
  }

  pub fn take_number<T: FromStr>(&mut self) -> Option<T> {
    self.take_until_whitespace().parse::<T>().ok()
  }

  pub fn numbers<T: FromStr>(&mut self) -> Vec<T> {
    let mut v = Vec::new();
    while let Some(n) = self.take_number::<T>() {
      v.push(n);
    }
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
  fn from_parse(value: Vec<String>) -> T;
}
impl FromParser<(String, String, String)> for (String, String, String) {
  fn from_parse(vec: Vec<String>) -> (String, String, String) {
    (vec[0].clone(), vec[1].clone(), vec[2].clone())
  }
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

  pub fn take_until_whitespace(&mut self) -> &mut Self {
    self.ops.push(BldOps::TakeUntilWhitespace);
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
          BldOps::TakeUntil(t) => Some(p.take_until(t)),
          BldOps::TakeUntilWhitespace => Some(p.take_until_whitespace()),
          BldOps::Consume(t) => {
            p.consume(t);
            None
          }
        })
        .collect(),
    )
  }
}

#[derive(Clone)]
enum BldOps {
  TakeUntil(String),
  TakeUntilWhitespace,
  Consume(String),
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
