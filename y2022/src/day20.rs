use itertools::Itertools;
use rust_util::Day;
use std::{collections::VecDeque, error::Error, fmt::Display};

type Bit = i64;
pub struct Solve {
  file: VecDeque<(usize, Bit)>,
  key: Bit,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      file: value
        .lines()
        .filter_map(|v| v.parse::<Bit>().ok())
        .enumerate()
        .collect(),
      key: 811589153,
    })
  }
}

fn rotate(id: &usize, buffer: &mut VecDeque<(usize, Bit)>) {
  let (idx, _) = buffer
    .iter()
    .find_position(|(id_o, _)| id_o == id)
    .expect("Must exist");

  buffer.rotate_left(idx); // Bring it to the front
  let (id, amt) = buffer.pop_front().unwrap();
  let new_idx = amt.rem_euclid(buffer.len() as Bit) as usize;
  buffer.rotate_left(new_idx); // Bring the front to the new idx
  buffer.push_front((id, amt));
}

fn grove_position(buffer: &VecDeque<(usize, Bit)>) -> Bit {
  let (offset, _) = buffer.iter().find_position(|(_, v)| v == &0).unwrap();
  let (_, first) = buffer[(offset + 1000) % buffer.len()];
  let (_, second) = buffer[(offset + 2000) % buffer.len()];
  let (_, third) = buffer[(offset + 3000) % buffer.len()];
  first + second + third
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut buffer = self.file.clone();
    for id in 0..buffer.len() {
      rotate(&id, &mut buffer);
    }
    Ok(Box::new(grove_position(&buffer).to_string()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut buffer = self
      .file
      .iter()
      .map(|(idx, v)| (*idx, *v * self.key))
      .collect::<VecDeque<_>>();
    for _ in 0..10 {
      for id in 0..buffer.len() {
        rotate(&id, &mut buffer);
      }
    }
    Ok(Box::new(grove_position(&buffer).to_string()))
  }
}
