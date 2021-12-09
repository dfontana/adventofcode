use rust_util::{AocDay, Day};
use std::{collections::HashMap, error::Error, fmt::Display};

type Digit = u32;
const LET_A: u32 = 'a' as u32;

pub struct Solve {
  items: Vec<(Vec<Digit>, Vec<Digit>)>,
}

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    let input = rust_util::read_input(2021, d)?;
    let items = input
      .lines()
      .map(|l| {
        let mut pair = l.splitn(2, " | ").map(|v| {
          v.split(" ")
            .map(|v| {
              v.chars()
                .fold(0b0000000, |mask, c| mask | 1 << (c as u32 - LET_A))
            })
            .collect::<Vec<Digit>>()
        });
        (pair.next().unwrap(), pair.next().unwrap())
      })
      .collect();
    Ok(Box::new(Solve { items }))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .items
        .iter()
        .flat_map(|(_, out)| out.iter())
        .filter(|out| {
          let count = count_ones(out);
          (count >= 2 && count <= 4) || count == 7
        })
        .count(),
    ))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .items
        .iter()
        .map(|(inp, out)| {
          let codex = decode(&inp);
          out
            .iter()
            .map(|digit| solve(&codex, digit))
            .collect::<String>()
            .parse::<u32>()
            .unwrap()
        })
        .sum::<u32>(),
    ))
  }
}

fn count_ones(mask: &Digit) -> usize {
  (0..7).fold(0, |acc, shift| {
    if mask & (1 << shift) > 0 {
      acc + 1
    } else {
      acc
    }
  })
}

fn decode(digits: &Vec<Digit>) -> HashMap<u32, Digit> {
  let mut num_0: Digit = 0;
  let mut num_1: &Digit = &0;
  let mut num_2: Digit = 0;
  let mut num_3: Digit = 0;
  let mut num_4: &Digit = &0;
  let mut num_5: Digit = 0;
  let mut num_6: Digit = 0;
  let mut num_7: &Digit = &0;
  let mut num_8: &Digit = &0;
  let mut num_9: Digit = 0;

  // Stash the known digits, and set aside the ones we need to discover
  // the others (eg the six long digits)
  let mut fivers: Vec<(&Digit, Digit)> = Vec::new();
  let mut sixers: Vec<(&Digit, Digit)> = Vec::new();
  for digit in digits {
    match count_ones(&digit) {
      2 => num_1 = &digit,
      3 => num_7 = &digit,
      4 => num_4 = &digit,
      7 => num_8 = &digit,
      5 => fivers.push((&digit, *digit)),
      6 => sixers.push((&digit, *digit)),
      _ => (),
    }
  }

  // The top segment is the difference of (1, 7)
  let top_md = num_7 ^ num_1;

  // The middle segment is the common item of (4,2,3,5)
  let middle = &fivers.iter().fold(*num_4, |acc, (_, mask)| acc & mask);

  // The bottom segment is (2,3,5) - (Top, Middle, 1)
  for (digit, mask) in fivers.iter_mut() {
    *mask = *mask & !num_1 & !top_md & !middle;
    if num_3 == 0 && count_ones(mask) == 1 {
      num_3 = **digit;
    }
  }

  // Now intersect the fivers with 4; 5 will match 1 times. 2 will match 0 times.
  for (digit, mask) in fivers.iter_mut() {
    if **digit == num_3 {
      continue;
    }
    *mask &= num_4;
    match count_ones(mask) {
      1 => num_5 = **digit,
      0 => num_2 = **digit,
      _ => (),
    }
  }

  // 0 has a middle, while 1 intersects 6 once & 9 twice.
  for (digit, mask) in sixers.iter_mut() {
    if num_0 == 0 && (*mask & middle == 0) {
      num_0 = **digit;
      continue;
    }
    *mask &= num_1;
    match count_ones(mask) {
      1 => num_6 = **digit,
      2 => num_9 = **digit,
      _ => (),
    }
  }

  let mut codex: HashMap<u32, Digit> = HashMap::new();
  codex.insert(0, num_0);
  codex.insert(1, *num_1);
  codex.insert(2, num_2);
  codex.insert(3, num_3);
  codex.insert(4, *num_4);
  codex.insert(5, num_5);
  codex.insert(6, num_6);
  codex.insert(7, *num_7);
  codex.insert(8, *num_8);
  codex.insert(9, num_9);
  codex
}

fn solve(codex: &HashMap<u32, Digit>, digit: &Digit) -> String {
  codex
    .iter()
    .find(|(_, set)| set == &digit)
    .map(|(v, _)| v.to_string())
    .expect("Could not decode digit")
}
