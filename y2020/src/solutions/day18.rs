use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::error::Error;

#[derive(Clone, Debug)]
enum Token {
  Operator(Op),
  ExpStart,
  ExpEnd,
  Val(i32),
}

#[derive(Clone, Debug)]
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

pub struct Solve {}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    let inp: Vec<i32> = read_input(d)?
      .lines()
      .map(|l| {
        let tokens = tokenize(l);
        // TODO tokenizing works, but parsing does not
        let expression = parse2(&tokens, &mut Vec::new(), Op::Identity);
        let result = evaluate(&expression);
        println!("{:?}\n\n{:?}\n\n{:?}", tokens, expression, result);
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

// fn parse(tokens: &Vec<Token>) -> Exp {
//   let mut items = Vec::new();
//   let mut operator = None;
//   for token in tokens {
//     match token {
//       Token::ExpStart => items.push(parse(str[idx..])),
//       Token::ExpEnd => break,
//       Token::Val(v) => items.push(Exp::val(*v)),
//       Token::Operator(op) => {
//         let new_op = match operator {
//           Some(o) => {
//             let sub_exp = Exp::new(&items, o);
//             items.clear();
//             items.push(sub_exp);
//             op
//           },
//           None => op
//         };
//         operator = Some(new_op.clone());
//       },
//     }
//   }
//   match operator {
//     Some(o) => Exp::new(&items, o),
//     None => unreachable!()
//   }
// }

fn parse2(tokens: &[Token], items: &mut Vec<Exp>, operator: Op) -> Exp {
  if let Some((first, rest)) = tokens.split_first() {
    match first {
      Token::ExpStart => items.push(parse2(rest, &mut Vec::new(), Op::Identity)),
      Token::Val(v) => items.push(Exp::val(*v)),
      Token::Operator(op) => {
        parse2(rest, &mut vec![Exp::new(&items, &operator)], op.clone());
      }
      Token::ExpEnd => return Exp::new(items, &operator),
    }
  }
  return Exp::new(items, &operator);
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
