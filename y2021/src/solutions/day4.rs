use itertools::Itertools;
use rust_util::{AocDay, Day};
use std::{
  collections::{HashMap, HashSet},
  error::Error,
  fmt::Display,
};

const BINGOS: [u32; 10] = [
  0b_00000_00000_00000_00000_11111,
  0b_00000_00000_00000_11111_00000,
  0b_00000_00000_11111_00000_00000,
  0b_00000_11111_00000_00000_00000,
  0b_11111_00000_00000_00000_00000,
  0b_00001_00001_00001_00001_00001,
  0b_00010_00010_00010_00010_00010,
  0b_00100_00100_00100_00100_00100,
  0b_01000_01000_01000_01000_01000,
  0b_10000_10000_10000_10000_10000,
];

pub struct Solve {
  calls: Vec<u32>,
  boards: Vec<Board>,
}

#[derive(Clone, Debug)]
struct Board {
  marked: u32,
  board: HashMap<u32, u32>,
  id: usize,
}

impl Board {
  fn from(line: &str, id: usize) -> Board {
    Board {
      board: line
        .split(' ')
        .filter(|f| !f.is_empty())
        .map(|f| f.parse::<u32>().unwrap())
        .enumerate()
        .fold(HashMap::new(), |mut acc, (pos, v)| {
          acc.insert(v, pos as u32);
          acc
        }),
      id,
      marked: 0b00000_00000_00000_00000_00000,
    }
  }

  fn call(&mut self, c: &u32) {
    if let Some(pos) = self.board.get(c) {
      self.marked = self.marked ^ (1 << pos);
    }
  }

  fn maybe_bingo(&self) -> Option<&Board> {
    BINGOS
      .iter()
      .find_map(|bingo| match self.marked & bingo == *bingo {
        true => Some(self),
        false => None,
      })
  }

  fn score(&self, c: &u32) -> u32 {
    self
      .board
      .iter()
      .filter(|(_, pos)| (1 << *pos) & !self.marked > 0)
      .map(|(v, _)| v)
      .sum::<u32>()
      * c
  }
}

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    let input = rust_util::read_input(2021, d)?;
    let mut lines = input.lines();
    let calls = lines
      .next()
      .unwrap()
      .split(",")
      .map(|f| f.parse::<u32>().unwrap())
      .collect();
    let mut boards = Vec::new();
    let mut id = 0;
    for mut ls in &lines.filter(|f| !f.is_empty()).chunks(5) {
      boards.push(Board::from(&ls.join(" "), id));
      id += 1;
    }
    Ok(Box::new(Solve { calls, boards }))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    self
      .calls
      .iter()
      .fold((self.boards.clone(), None), |(mut boards, score), call| {
        if let Some(_) = score {
          return (boards, score);
        }
        boards.iter_mut().for_each(|b| b.call(call));
        let new_score = boards
          .iter()
          .find_map(|board| board.maybe_bingo().map(|b| b.score(call)));
        (boards, new_score.map(|s| Box::new(s) as Box<dyn Display>))
      })
      .1
      .ok_or::<Box<dyn Error>>("No Solve Found".into())
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut boards_in_play = self.boards.iter().map(|b| b.id).collect::<HashSet<usize>>();
    let mut boards = self.boards.clone();
    for call in &self.calls {
      for board in boards.iter_mut() {
        if !boards_in_play.contains(&board.id) {
          continue;
        }
        board.call(call);
        boards_in_play.insert(board.id);
        if let Some(board) = board.maybe_bingo() {
          if boards_in_play.len() == 1 {
            return Ok(Box::new(board.score(call)));
          }
          boards_in_play.remove(&board.id);
        }
      }
    }
    Err("No solve found".into())
  }
}
