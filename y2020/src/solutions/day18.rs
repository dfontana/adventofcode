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
    Ok("Impl".to_string())
  }
}

fn eval_expr_easy(tokens: &[Token], i: usize) -> (usize, i64) {
  let (i, lhs) = match tokens[i] {
    Token::Val(n) => (i + 1, n as i64),
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

fn parse5(tokens: &[Token], idx: usize) -> (usize, Exp) {
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
      let (i, val) = parse5(tokens, idx + 1);
      match tokens[i] {
        Token::ExpEnd => (idx + 1, val),
        _ => panic!("Parser Error"),
      }
    }
    _ => panic!("Parser Error"),
  }
}

// fn parse4(tokens: &[Token]) -> Exp {
//   if tokens.is_empty() {
//     return Exp::default();
//   }
//   if tokens[0] == Token::ExpStart {
//     println!("Panic ==== {:?}\n", tokens);
//     let (idx, paren) = tokens[1..]
//       .iter()
//       .enumerate()
//       .find_map(|(idx, tk)| match tk {Token::ExpEnd | Token::ExpStart => Some((idx, tk)), _ => None})
//       .unwrap();

//     let sub = match paren {
//       Token::ExpEnd => {
//         println!("Sub >>>> {:?}\n", &tokens[1..idx+1]);
//         parse4(&tokens[1..idx+1])
//       },
//       Token::ExpStart => {
//         println!("Sub >>>> {:?}\n", &tokens[1..]);
//         return parse4(&tokens[1..])
//       },
//       _ => unreachable!(),
//     };

//     println!("Sub <<<<\n");

//     if let Some((first, rest)) = tokens[idx+2..].split_first() {
//       println!("First - {:?} ::: Rest - {:?}\n", first, rest);
//       match first {
//         Token::Operator(op) => {
//           return Exp::new(&vec![sub, parse4(rest)], op)
//         },
//         _ => return sub,
//       }
//     }
//     return sub;
//   }

//   let maybe = tokens.iter().enumerate().find_map(|(idx, tk)| match tk {Token::Operator(op) => Some((idx, op)), _ => None});
//   let (idx, op) = match maybe {
//     Some(v) => v,
//     None => return match tokens[0] {
//       Token::Val(v) => Exp::val(v),
//       _ => Exp::default(),
//     },
//   };

//   let (l, r) = tokens.split_at(idx);
//   if !&r[1..].is_empty(){
//     println!("EVAL **** {:?} **** {:?}\n", l, &r[1..]);
//     return Exp::new(&vec![parse4(l), parse4(&r[1..])], op)
//   }
//   println!("VALUE $$$$ {:?}\n", l);
//   return parse4(l);
// }

/*
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
1 + (2 * 3) + (4 * (5 + 6))
*/

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
