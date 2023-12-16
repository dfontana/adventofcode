use itertools::Itertools;
use rust_util::Day;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct Solve {
  round: Vec<(usize, usize)>,
  board: Vec<Vec<Tile>>,
  y_max: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tile {
  Round,
  Blank,
  Square,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let mut round = Vec::new();
    let board = value
      .lines()
      .enumerate()
      .map(|(y, l)| {
        l.chars()
          .enumerate()
          .map(|(x, c)| match c {
            'O' => {
              round.push((x, y));
              Tile::Round
            }
            '.' => Tile::Blank,
            '#' => Tile::Square,
            _ => unreachable!(),
          })
          .collect_vec()
      })
      .collect_vec();
    Ok(Solve {
      round,
      y_max: board.len(),
      board,
    })
  }
}

fn shift_north(v: &mut Vec<Vec<Tile>>, bldrs: &mut Vec<(usize, usize)>) {
  for (x, dy) in bldrs.iter_mut().sorted() {
    loop {
      if *dy == 0 {
        break;
      }

      if v[*dy - 1][*x] == Tile::Round || v[*dy - 1][*x] == Tile::Square {
        break;
      }

      v[*dy][*x] = Tile::Blank;
      v[*dy - 1][*x] = Tile::Round;
      *dy -= 1;
    }
  }
}

fn debug_brd(v: &Vec<Vec<Tile>>) {
  for r in v {
    for t in r {
      print!(
        "{}",
        match t {
          Tile::Round => 'O',
          Tile::Blank => '.',
          Tile::Square => '#',
        }
      );
    }
    println!();
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut tiles = self.board.clone();
    let mut round = self.round.clone();
    shift_north(&mut tiles, &mut round);
    Ok(Box::new(format!(
      "{:?}",
      round
        .iter()
        .map(|(_, y)| { self.y_max - *y })
        .reduce(|acc, y| acc + y)
    )))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(1))
  }
}
