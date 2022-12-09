use rust_util::Day;
use std::{error::Error, fmt::Display};

pub struct Solve {
  visible: usize,
  high_score: i32,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let field: Vec<Vec<u32>> = value
      .lines()
      .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
      .collect();

    let (mut visible, mut high_score) = (0, 0);
    for i in 1..field.len() - 1 {
      for k in 1..field[0].len() - 1 {
        let t = field[i][k];

        let (ld, lv) = score_dir(|v| field[i][v] >= t, (0..k).rev());
        let (rd, rv) = score_dir(|v| field[i][v] >= t, (k + 1)..field[0].len());
        let (ud, uv) = score_dir(|v| field[v][k] >= t, (0..i).rev());
        let (dd, dv) = score_dir(|v| field[v][k] >= t, (i + 1)..field.len());

        if lv || rv || uv || dv {
          visible += 1;
        }
        let score = ld * rd * ud * dd;
        if score > high_score {
          high_score = score;
        }
      }
    }

    Ok(Solve {
      visible: visible + (2 * field.len()) + (2 * (field[0].len() - 2)),
      high_score,
    })
  }
}

fn score_dir(test: impl Fn(usize) -> bool, rg: impl Iterator<Item = usize>) -> (i32, bool) {
  let mut dist = 0;
  for i in rg {
    dist += 1;
    if test(i) {
      return (dist, false);
    }
  }
  (dist, true)
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(self.visible))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(self.high_score))
  }
}
