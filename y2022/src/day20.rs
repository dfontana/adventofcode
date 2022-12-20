use itertools::Itertools;
use rust_util::Day;
use std::{error::Error, fmt::Display};

pub struct Solve {
  file: Vec<(usize, i32)>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      file: parse_input(value),
    })
  }
}

fn parse_input(input: String) -> Vec<(usize, i32)> {
  input
    .lines()
    .enumerate()
    .filter_map(|(idx, v)| v.parse::<i32>().ok().map(|k| (idx, k)))
    .collect()
}

fn rotate(id: &usize, buffer: &mut Vec<(usize, i32)>) {
  // find where the id is
  let (idx, (_, amt)) = buffer
    .iter()
    .find_position(|(id_o, _)| id_o == id)
    .expect("Must exist");

  let new_idx = {
    if amt > &0 {
      (idx as i32 + amt) as usize % buffer.len()
    } else {
       //= 1949
        // given 0 + -3051 on 5000 buff len
      let mut new_idx = idx as i32 + amt;
      if new_idx < 0 {
        let magnitude = new_idx.abs();
        let reduced = magnitude % buffer.len() as i32;
        new_idx = buffer.len() as i32 - reduced;
        if (idx as i32) < new_idx {
            // If we remove an element to the left of where this one is going,
            // the new idx shifts left one. So we need to shift this left one.
            // Otherwise the shift is "downstream" and doesn't affect this outcome
            new_idx -= 1;
        }
        println!("{} + {} => {} => {} => {}", idx, amt, magnitude, reduced, new_idx);
        // new_idx = (((idx as i32 + amt).abs() / buffer.len() as i32) + 1) * buffer.len() as i32 + (idx as i32 + amt);
        //(idx + buffer.len()) as i32 + amt;
      }
      new_idx as usize
    }
  };
  // println!("{:?}",buffer);
  let ele = buffer.remove(idx);
  
  buffer.insert(new_idx, ele);
  
}
// WRONG: 11212 ; 
impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut buffer = self.file.clone();
    for (id, _) in &self.file {
        rotate(id, &mut buffer);
    }
    let (offset, _) = buffer.iter().find_position(|(_, v)| v == &0).unwrap();
    let (_, first) = buffer[(offset + 1000) % buffer.len()];
    let (_, second) = buffer[(offset + 2000) % buffer.len()];
    let (_, third) = buffer[(offset + 3000) % buffer.len()];
    Ok(Box::new((first+second+third).to_string()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new("y"))
  }
}
