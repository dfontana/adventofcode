use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

pub struct Solve {
  tape: Vec<Instruction>,
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
  NOP(i32),
  ACC(i32),
  JMP(i32),
}

#[derive(Debug)]
enum Terminate {
  NORMAL(i32),
  LOOP(i32),
  ERROR(String),
}

impl FromStr for Instruction {
  type Err = String;
  fn from_str(input: &str) -> Result<Instruction, Self::Err> {
    let mut iter = input.splitn(2, " ");
    let ins = iter
      .next()
      .ok_or("Missing Instruction in line")?
      .to_ascii_lowercase();
    let amt = iter
      .next()
      .and_then(|f| f.parse::<i32>().ok())
      .ok_or("Missing Argument in line")?;
    match ins.to_ascii_uppercase().as_str() {
      "NOP" => Ok(Instruction::NOP(amt)),
      "ACC" => Ok(Instruction::ACC(amt)),
      "JMP" => Ok(Instruction::JMP(amt)),
      _ => Err(format!("Unknown instruction hit: {}", ins)),
    }
  }
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    Ok(Solve {
      tape: read_input(d)?
        .lines()
        .map(Instruction::from_str)
        .flatten()
        .collect(),
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    match run_tape(&self.tape) {
      Terminate::LOOP(amt) => Ok(amt.to_string()),
      _ => Err("No loop found".into()),
    }
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    self
      .tape
      .repeat(self.tape.len())
      .chunks_mut(self.tape.len())
      .enumerate()
      .flat_map(|(idx, chunk)| {
        let mut alt = chunk.to_owned();
        match chunk[idx] {
          Instruction::NOP(amt) => alt[idx] = Instruction::JMP(amt),
          Instruction::JMP(amt) => alt[idx] = Instruction::NOP(amt),
          _ => return vec![],
        }
        vec![(idx, chunk[idx], chunk.to_owned()), (idx, alt[idx], alt)]
      })
      .map(|(idx, ins, t)| (idx, ins, run_tape(&t.to_vec())))
      .filter_map(|(idx, ins, res)| match res {
        Terminate::NORMAL(amt) => Some((idx, ins, amt)),
        _ => None,
      })
      .next()
      .map(|(idx, ins, acc)| format!("Acc {} - INS {} to {:?}", acc, idx, ins))
      .ok_or("Did not find normal termination".into())
  }
}

fn run_tape(tape: &Vec<Instruction>) -> Terminate {
  let mut seen_idx: HashSet<i32> = HashSet::new();
  let mut next_idx: i32 = 0;
  let mut acc = 0;
  while !seen_idx.contains(&next_idx) {
    seen_idx.insert(next_idx);
    if next_idx == tape.len() as i32 {
      return Terminate::NORMAL(acc);
    }
    let ins = match tape.get(next_idx as usize) {
      None => return Terminate::ERROR("Tape Overflow".to_string()),
      Some(ins) => ins,
    };
    match ins {
      Instruction::NOP(_) => next_idx += 1,
      Instruction::ACC(amt) => {
        acc += amt;
        next_idx += 1;
      }
      Instruction::JMP(amt) => next_idx += amt,
    }
  }
  Terminate::LOOP(acc)
}
