use rust_util::{ Day};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

pub struct Solve {
  jolts: Vec<u64>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let mut jolts: Vec<u64> = value 
      .lines()
      .map(|i| i.parse::<u64>())
      .flatten()
      .collect();
    jolts.insert(0, 0);
    jolts.sort();
    jolts.push(jolts[jolts.len() - 1] + 3);
    Ok(Solve { jolts })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut c1 = 0;
    let mut c3 = 0;
    let mut prev = 0;
    for i in self.jolts.iter() {
      match i - prev {
        1 => c1 += 1,
        3 => c3 += 1,
        _ => (),
      }
      prev = *i;
    }
    Ok(Box::new((c1 * c3).to_string()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    fn search(jolts: &[u64], memo: &mut HashMap<u64, u64>) -> u64 {
      match jolts.split_first() {
        None => 0,          // Unreachable
        Some((_, [])) => 1, // Base Case
        Some((v, queue)) => queue
          .iter()
          .take_while(|a| *a - v <= 3)
          .enumerate()
          .map(|(idx, val)| match memo.get(val) {
            Some(v) => *v,
            None => {
              // Look at each sub list in the queue from val onwards
              let child_sum = search(&queue[idx..], memo);
              *memo.entry(*val).or_insert(child_sum)
            }
          })
          .sum(),
      }
    }
    Ok(Box::new(
      search(&self.jolts, &mut HashMap::new()).to_string(),
    ))
  }
}
