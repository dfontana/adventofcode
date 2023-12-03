use rust_util::Day;
use std::{collections::HashSet, error::Error, fmt::Display};

type Coord = (i32, i32);
pub struct Solve {
  numbers: Vec<Number>,
  symbols: HashSet<Coord>,
  gears: HashSet<Coord>,
}

#[derive(Debug)]
struct Number {
  value: usize,
  bbox: HashSet<Coord>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let mut x = 0;
    let mut y = 0;

    let mut numbers = Vec::new();
    let mut symbols = HashSet::new();
    let mut gears = HashSet::new();

    for line in value.lines() {
      let mut num = String::new();
      let mut bbox = HashSet::new();
      for c in line.chars() {
        if c.is_digit(10) {
          if num.is_empty() {
            bbox.insert((x - 1, y - 1));
            bbox.insert((x - 1, y));
            bbox.insert((x - 1, y + 1));
          }
          num.push(c);
          bbox.insert((x, y - 1));
          bbox.insert((x, y + 1));
          x += 1;
          continue;
        }

        if !num.is_empty() {
          bbox.insert((x, y - 1));
          bbox.insert((x, y));
          bbox.insert((x, y + 1));
          numbers.push(Number {
            value: num.parse::<usize>().unwrap(),
            bbox: bbox.clone(),
          });
          num.clear();
          bbox.clear();
        }

        if c != '.' {
          symbols.insert((x, y));
          if c == '*' {
            gears.insert((x, y));
          }
        }

        x += 1;
      }

      if !num.is_empty() {
        bbox.insert((x, y - 1));
        bbox.insert((x, y));
        bbox.insert((x, y + 1));
        numbers.push(Number {
          value: num.parse::<usize>().unwrap(),
          bbox: bbox.clone(),
        });
        num.clear();
        bbox.clear();
      }
      x = 0;
      y += 1;
    }

    Ok(Solve {
      numbers,
      symbols,
      gears,
    })
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .numbers
        .iter()
        .filter(|n| n.bbox.intersection(&self.symbols).next().is_some())
        .map(|n| n.value)
        .sum::<usize>(),
    ))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .gears
        .iter()
        .map(|g| {
          self
            .numbers
            .iter()
            .filter(|n| n.bbox.contains(g))
            .map(|n| n.value)
            .fold((0, 1), |(ct, acc), v| (ct + 1, acc * v))
        })
        .filter(|(ct, _)| *ct == 2)
        .map(|(_, v)| v)
        .sum::<usize>(),
    ))
  }
}
