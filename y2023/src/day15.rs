use itertools::Itertools;
use rust_util::Day;
use std::{collections::HashSet, error::Error, fmt::Display};

pub struct Solve {
  input: Vec<String>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      input: value.trim().split(',').map(|s| s.to_string()).collect(),
    })
  }
}

fn hash(s: &str) -> usize {
  s.chars()
    .map(|c| c as u32)
    .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

impl Solve {
  fn vet_code(&self) -> usize {
    self.input.iter().map(|s| hash(s)).sum()
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut dis = HashSet::new();
    for v in self.input.iter() {
      v.chars().for_each(|c| {
        dis.insert(c);
      });
      println!("{:?}", v);
    }
    println!("{:?}", dis);
    Ok(Box::new(self.vet_code()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(1))
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
