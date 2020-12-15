use crate::day::{Day, DayArg};
use crate::util::read_input;
use std::error::Error;
use std::{collections::HashMap, str::FromStr};

pub struct Solve {
  program: Vec<Op>,
}

#[derive(Debug, Clone)]
struct Mask {
  zero: u64,
  ones: u64,
}

#[derive(Debug)]
enum Op {
  Mask(Mask),
  Mem(u64, u64),
}

impl FromStr for Op {
  type Err = String;
  fn from_str(inp: &str) -> std::result::Result<Self, Self::Err> {
    let mut split = inp.splitn(2, " = ");
    let op = split.next().unwrap();
    match op {
      "mask" => Ok(Op::Mask(
        Mask::from_str(split.next().unwrap()).map_err(|_| "Cant pull mask")?,
      )),
      _ => {
        let add = op[4..op.len() - 1]
          .parse::<u64>()
          .map_err(|_| "Cant pull address")?;
        let val = split
          .next()
          .unwrap()
          .parse::<u64>()
          .map_err(|_| "Cant pull value")?;
        Ok(Op::Mem(add, val))
      }
    }
  }
}

impl FromStr for Mask {
  type Err = String;
  fn from_str(inp: &str) -> std::result::Result<Self, Self::Err> {
    Ok(Mask {
      ones: u64::from_str_radix(&inp.replace("X", "1"), 2).unwrap(),
      zero: u64::from_str_radix(&inp.replace("X", "0"), 2).unwrap(),
    })
  }
}

impl Default for Mask {
  fn default() -> Self {
    Self {
      ones: 2_u64.pow(36) - 1,
      zero: 0,
    }
  }
}

impl Day for Solve {
  fn new(d: DayArg) -> Result<Solve, Box<dyn Error>> {
    Ok(Solve {
      program: read_input(d)?.lines().map(Op::from_str).flatten().collect(),
    })
  }

  fn p1(&self) -> Result<String, Box<dyn Error>> {
    let ans: u64 = self
      .program
      .iter()
      .fold(
        (HashMap::new(), Mask::default()),
        |(mut mem, mut mask), op| {
          match op {
            Op::Mask(m) => mask = m.clone(),
            Op::Mem(adr, val) => {
              mem.insert(*adr, val & mask.ones | mask.zero);
            }
          };
          (mem, mask)
        },
      )
      .0
      .values()
      .sum();
    Ok(ans.to_string())
  }

  fn p2(&self) -> Result<String, Box<dyn Error>> {
    Ok("Impl".to_string())
  }
}
