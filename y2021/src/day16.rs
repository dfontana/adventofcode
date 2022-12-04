use rust_util::{Day};
use std::{error::Error, fmt::Display};

pub struct Solve {
  tape: String,
}

// This one got me...
// Adapted from https://github.com/jeffomatic/adventofcode/tree/main/2021-rust/day16b
// The iterators were a clever solve, rather than trying to manipulate bit vectors

struct Packet {
  version: usize,
  data: Data,
}

enum Data {
  Literal(u64),
  Operator(Operator),
}

enum OpType {
  Sum,
  Product,
  Min,
  Max,
  GreaterThan,
  LessThan,
  EqualTo,
}

struct Operator {
  optype: OpType,
  subpackets: Vec<Packet>,
}

fn parse_n(bits: &mut dyn Iterator<Item = char>, n: usize) -> usize {
  usize::from_str_radix(&bits.take(n).collect::<String>(), 2).unwrap()
}

fn parse_literal(bits: &mut dyn Iterator<Item = char>) -> u64 {
  let mut lit_bits = Vec::new();
  loop {
    let last = bits.next().unwrap() == '0';
    lit_bits.extend(bits.take(4));
    if last {
      let bitstring = lit_bits.iter().collect::<String>();
      return u64::from_str_radix(&bitstring, 2).unwrap();
    }
  }
}

fn parse_operator_subpackets(bits: &mut dyn Iterator<Item = char>) -> Vec<Packet> {
  let mut res = Vec::new();
  let length_type_id = bits.next().unwrap();
  match length_type_id {
    '0' => {
      let nbits = parse_n(bits, 15);
      let mut sub_bits = bits.take(nbits).peekable();
      while sub_bits.peek().is_some() {
        res.push(parse_packet(&mut sub_bits));
      }
    }
    '1' => {
      for _ in 0..parse_n(bits, 11) {
        res.push(parse_packet(bits));
      }
    }
    _ => unreachable!(),
  }
  res
}

fn parse_packet(bits: &mut dyn Iterator<Item = char>) -> Packet {
  let version = parse_n(bits, 3);
  let type_id = parse_n(bits, 3);
  let data = match type_id {
    4 => Data::Literal(parse_literal(bits)),
    _ => Data::Operator(Operator {
      optype: match type_id {
        0 => OpType::Sum,
        1 => OpType::Product,
        2 => OpType::Min,
        3 => OpType::Max,
        5 => OpType::GreaterThan,
        6 => OpType::LessThan,
        7 => OpType::EqualTo,
        _ => unimplemented!(),
      },
      subpackets: parse_operator_subpackets(bits),
    }),
  };
  Packet { version, data }
}

fn eval(p: &Packet) -> u64 {
  match &p.data {
    Data::Literal(val) => *val,
    Data::Operator(op) => {
      let mut sub_vals = op.subpackets.iter().map(|sub| eval(sub));
      match op.optype {
        OpType::Sum => sub_vals.sum(),
        OpType::Product => sub_vals.product(),
        OpType::Min => sub_vals.min().unwrap(),
        OpType::Max => sub_vals.max().unwrap(),
        OpType::GreaterThan => {
          if sub_vals.next().unwrap() > sub_vals.next().unwrap() {
            1
          } else {
            0
          }
        }
        OpType::LessThan => {
          if sub_vals.next().unwrap() < sub_vals.next().unwrap() {
            1
          } else {
            0
          }
        }
        OpType::EqualTo => {
          if sub_vals.next().unwrap() == sub_vals.next().unwrap() {
            1
          } else {
            0
          }
        }
      }
    }
  }
}

fn sum_versions(p: &Packet) -> u64 {
  match &p.data {
    Data::Literal(_) => p.version as u64,
    Data::Operator(op) => {
      p.version as u64
        + op
          .subpackets
          .iter()
          .map(|sub| sum_versions(sub))
          .sum::<u64>()
    }
  }
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(input: String) -> Result<Self, Self::Error> {
    let tape = input
      .trim()
      .chars()
      .flat_map(|char| {
        let val = char.to_digit(16).unwrap();
        vec![
          val & 0b1000u32 != 0,
          val & 0b0100u32 != 0,
          val & 0b0010u32 != 0,
          val & 0b0001u32 != 0,
        ]
      })
      .map(|v| if v { '1' } else { '0' })
      .collect::<String>();
    Ok(Solve { tape })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let tape = self.tape.clone();
    let mut bits = tape.chars();
    Ok(Box::new(sum_versions(&parse_packet(&mut bits))))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let tape = self.tape.clone();
    let mut bits = tape.chars();
    Ok(Box::new(eval(&parse_packet(&mut bits))))
  }
}
