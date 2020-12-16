use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::collections::HashMap;
use std::error::Error;

pub struct Solve {
  program: Vec<Op>,
}

#[derive(Debug, Clone, Copy)]
struct Mask {
  zero: u64,
  ones: u64,
  flow: u64,
}

#[derive(Debug, Clone)]
enum Op {
  Mask(Mask),
  Mem(u64, u64),
}

impl Mask {
  fn new() -> Self {
    Mask {
      zero: 0,
      ones: 0,
      flow: 0,
    }
  }
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    Ok(Solve {
      program: read_input(d)?
        .lines()
        .map(|inp| {
          let mut split = inp.splitn(2, " = ");
          let op = match (split.next().unwrap(), split.next().unwrap()) {
            ("mask", v) => Op::Mask(Mask {
              ones: u64::from_str_radix(&v.replace("X", "1"), 2).unwrap(),
              zero: u64::from_str_radix(&v.replace("X", "0"), 2).unwrap(),
              flow: u64::from_str_radix(&v.replace("1", "0").replace("X", "1"), 2).unwrap(),
            }),
            (op, v) => {
              let add = op[4..op.len() - 1].parse::<u64>().unwrap();
              let val = v.parse::<u64>().unwrap();
              Op::Mem(add, val)
            }
          };
          op
        })
        .collect(),
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    Ok(
      run_program(&self.program, |mask, adr, val, mem| {
        mem.insert(adr, val & mask.ones | mask.zero);
      })
      .to_string(),
    )
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok(
      run_program(&self.program, |mask, adr, val, mut mem| {
        update_memory(mask.flow, adr | mask.zero, val, &mut mem)
      })
      .to_string(),
    )
  }
}

fn run_program<F>(program: &Vec<Op>, mem_func: F) -> u64
where
  F: Fn(Mask, u64, u64, &mut HashMap<u64, u64>),
{
  program
    .iter()
    .fold((HashMap::new(), Mask::new()), |(mut mem, mut mask), op| {
      match op {
        Op::Mask(m) => mask = m.clone(),
        Op::Mem(adr, val) => mem_func(mask, *adr, *val, &mut mem),
      };
      (mem, mask)
    })
    .0
    .values()
    .sum()
}

fn update_memory(mask: u64, addr: u64, val: u64, memory: &mut HashMap<u64, u64>) {
  if mask == 0 {
    memory.insert(addr, val);
  } else {
    let x = mask & (!mask + 1);
    let mask = mask & !x;
    update_memory(mask, addr & !x, val, memory);
    update_memory(mask, addr | x, val, memory);
  }
}
