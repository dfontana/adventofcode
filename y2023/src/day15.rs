use itertools::Itertools;
use rust_util::Day;
use std::{collections::VecDeque, error::Error, fmt::Display};

impl From<&str> for Operation {
  fn from(value: &str) -> Self {
    if let Some((lbl, len)) = value.split_once('=') {
      return Operation::Add(Focal {
        lbl: lbl.to_string(),
        len: len.parse::<u8>().unwrap(),
      });
    }
    value
      .strip_suffix('-')
      .map(|lbl| Operation::Remove {
        lbl: lbl.to_string(),
      })
      .unwrap()
  }
}

impl From<&Operation> for String {
  fn from(value: &Operation) -> Self {
    match value {
      Operation::Add(Focal { lbl, len }) => format!("{}={}", lbl, len),
      Operation::Remove { lbl } => format!("{}-", lbl),
    }
  }
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      input: value.trim().split(',').map(Operation::from).collect(),
    })
  }
}

pub struct Solve {
  input: Vec<Operation>,
}

#[derive(Clone, PartialEq, Eq)]
enum Operation {
  Add(Focal),
  Remove { lbl: String },
}

#[derive(Clone, PartialEq, Eq)]
struct Focal {
  lbl: String,
  len: u8,
}

struct HASHMAP {
  boxes: Vec<VecDeque<Focal>>,
}

impl Default for HASHMAP {
  fn default() -> Self {
    HASHMAP {
      boxes: vec![VecDeque::new(); 256],
    }
  }
}

impl HASHMAP {
  fn operate(&mut self, op: &Operation) {
    match op {
      Operation::Add(f) => {
        let idx = hash(&f.lbl);
        let pos = self.boxes[idx].iter().find_position(|bo| bo.lbl == f.lbl);
        match pos {
          Some((i, _)) => {
            self.boxes[idx][i] = f.clone();
          }
          None => {
            self.boxes[idx].push_back(f.clone());
          }
        };
      }
      Operation::Remove { lbl } => {
        let idx = hash(lbl);
        let pos = self.boxes[idx].iter().find_position(|bo| bo.lbl == *lbl);
        match pos {
          Some((i, _)) => {
            self.boxes[idx].remove(i);
          }
          None => {}
        }
      }
    }
  }

  fn focusing_power(&self) -> usize {
    self
      .boxes
      .iter()
      .enumerate()
      .flat_map(|(box_num, bx)| {
        bx.iter()
          .enumerate()
          .map(move |(slot_num, focal)| (1 + box_num) * (1 + slot_num) * focal.len as usize)
      })
      .sum()
  }
}

fn hash(s: &str) -> usize {
  s.chars()
    .map(|c| c as u32)
    .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

impl Solve {
  fn vet_code(&self) -> usize {
    self
      .input
      .iter()
      .map(|op| {
        let s: String = op.into();
        hash(&s)
      })
      .sum()
  }

  fn to_map(&self) -> HASHMAP {
    let mut map = HASHMAP::default();
    self.input.iter().for_each(|op| map.operate(op));
    map
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(self.vet_code()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(self.to_map().focusing_power()))
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_ascii() {
    assert_eq!(' ' as u32, 32);
    assert_eq!('H' as u32, 72);
    assert_eq!('A' as u32, 65);
    assert_eq!('S' as u32, 83);
  }

  #[test]
  fn test_hash() {
    assert_eq!(hash("HASH"), 52);
    assert_eq!(hash("rn=1"), 30);
    assert_eq!(hash("cm-"), 253);
    assert_eq!(hash("qp=3"), 97);
    assert_eq!(hash("cm=2"), 47);
    assert_eq!(hash("qp-"), 14);
    assert_eq!(hash("pc=4"), 180);
    assert_eq!(hash("ot=9"), 9);
    assert_eq!(hash("ab=5"), 197);
    assert_eq!(hash("pc-"), 48);
    assert_eq!(hash("pc=6"), 214);
    assert_eq!(hash("ot=7"), 231);
  }

  #[test]
  fn test_verification() {
    let inp = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string();
    assert_eq!(Solve::try_from(inp).map(|s| s.vet_code()).unwrap(), 1320);
  }
}
