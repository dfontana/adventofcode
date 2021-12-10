use rust_util::{AocDay, Day};
use std::{collections::HashSet, error::Error, fmt::Display};

pub struct Solve {
  input: String,
}

// Part 1: 265527
// Part 2: 3969823589
// Elapsed: 306.58µs
impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    let input = rust_util::read_input(2021, d)?;
    Ok(Box::new(Solve { input }))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let opens: HashSet<char> = HashSet::from_iter(vec!['{', '[', '(', '<']);
    let mut score = 0;
    for line in self.input.lines() {
      // TODO maybe optimize this further; its already fairly quick for the input
      // TODO you don't need a stack, just a pointer to the top
      let mut stack = Vec::new();
      for ch in line.chars() {
        if opens.contains(&ch) {
          // TODO (which increments here)
          stack.push(ch);
        } else {
          // TODO (and returns val/decrements here)
          if let Some(top) = stack.pop() {
            if invert(&top) == ch {
              // valid
              continue;
            } else {
              // TODO for re-useability, return Some(IDX) of these corrupted chars
              // & map/score them in a second transform
              // corrupted
              score += match ch {
                '}' => 1197,
                ']' => 57,
                ')' => 3,
                '>' => 25137,
                _ => unreachable!(),
              };
              break;
            }
          }
          // incomplete otherwise
          break;
        }
      }
    }
    Ok(Box::new(score))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let opens: HashSet<char> = HashSet::from_iter(vec!['{', '[', '(', '<']);

    let mut scores: Vec<u64> = Vec::new();

    for line in self.input.lines() {
      let mut stack = Vec::new();

      // Build up the stack of opens that need closes
      for ch in line.chars() {
        if opens.contains(&ch) {
          stack.push(ch);
        } else {
          // It's a close char. Pop the top
          if let Some(top) = stack.pop() {
            if invert(&top) == ch {
              // Since it matches, continue to next char
              continue;
            } else {
              // If it's incorrect, this line is invalid. We're done.
              stack = Vec::new();
              break;
            }
          }
        }
      }

      // Now pop off the stack & fill in the gaps
      let mut score = 0;
      while let Some(top) = stack.pop() {
        score *= 5;
        match invert(&top) {
          '>' => score += 4,
          ')' => score += 1,
          ']' => score += 2,
          '}' => score += 3,
          _ => unreachable!(),
        }
      }
      if score > 0 {
        scores.push(score);
      }
    }
    scores.sort();
    Ok(Box::new(scores[scores.len() / 2]))
  }
}

fn invert(ch: &char) -> char {
  match ch {
    '{' => '}',
    '[' => ']',
    '(' => ')',
    '<' => '>',
    '>' => '<',
    ')' => '(',
    ']' => '[',
    '}' => '{',
    _ => unreachable!(),
  }
}
