use rust_util::Day;
use std::{error::Error, fmt::Display};

pub struct Solve {
  instrs: Vec<Instr>,
}

enum Instr {
  AddX(i32),
  NoOp,
}

impl Instr {
  fn cycles(&self) -> usize {
    match self {
      Instr::AddX(_) => 2,
      Instr::NoOp => 1,
    }
  }
}

impl From<&str> for Instr {
  fn from(value: &str) -> Instr {
    let mut split = value.split(' ');
    match (split.next(), split.next()) {
      (Some("addx"), Some(v)) => Instr::AddX(v.parse::<i32>().unwrap()),
      (Some("noop"), None) => Instr::NoOp,
      _ => unreachable!("Not a valid instruction: {}", value),
    }
  }
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      instrs: value.lines().map(Instr::from).collect(),
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    // System state
    let mut cycle = 0;
    let mut register_x = 1;
    let mut signal_strength = 0;

    // Instruction State
    let mut instr_ptr = self.instrs.iter();
    let mut current_instr = instr_ptr.next();
    let mut instr_cycle_cnt = 0;

    loop {
      let Some(instr) = current_instr else {
        break;
      };
      instr_cycle_cnt += 1;
      cycle += 1;

      // Read
      if cycle == 20 || (cycle - 20) % 40 == 0 {
        signal_strength += register_x * cycle;
      }

      // Check if instruction is done
      if instr_cycle_cnt < instr.cycles() {
        continue;
      }
      instr_cycle_cnt = 0;

      // Perform writes
      match instr {
        Instr::AddX(v) => {
          register_x += v; 
        }
        Instr::NoOp => (),
      }
      current_instr = instr_ptr.next();
    }

    Ok(Box::new(signal_strength))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new("y"))
  }
}
