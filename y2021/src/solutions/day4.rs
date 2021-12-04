use rust_util::{AocDay, Day};
use std::{collections::HashMap, error::Error, fmt::Display};

pub struct Solve {
  calls: Vec<u32>,
  boards: Vec<Board>,
}

#[derive(Clone, Debug)]
struct Board {
  board: Vec<Tile>,
  id: usize,
}

impl Board {
  fn call(&self, c: u32) -> Board {
    Board {
      id: self.id,
      board: self
        .board
        .iter()
        .map(|tile| match tile {
          Tile::Unmarked(v) if *v == c => Tile::Marked(*v),
          _ => tile.clone(),
        })
        .collect(),
    }
  }

  fn maybe_bingo(&self) -> Option<&Board> {
    // This could probably be faster if boards were a matrix of bit vectors
    // That could be compared (0b11111) to determine row-bingo
    // Or &'d together (row1 & row2 & row 3 & row4 & row5 > 0) to determine col-bingo
    for idx in 0..5 {
      let mut row_bingo = true;
      let mut col_bingo = true;
      for offset in 0..5 {
        let row = (idx * 5) + offset;
        let col = idx + (5 * offset);
        match self.board.get(row).unwrap() {
          Tile::Marked(_) => (),
          Tile::Unmarked(_) => row_bingo = false,
        }
        match self.board.get(col).unwrap() {
          Tile::Marked(_) => (),
          Tile::Unmarked(_) => col_bingo = false,
        }
        if !col_bingo && !row_bingo {
          break;
        }
      }
      if row_bingo || col_bingo {
        return Some(self);
      }
    }
    None
  }

  fn score(&self, c: u32) -> u32 {
    self
      .board
      .iter()
      .filter_map(|t| match t {
        Tile::Marked(_) => None,
        Tile::Unmarked(v) => Some(v),
      })
      .sum::<u32>()
      * c
  }
}

#[derive(Clone, Debug)]
enum Tile {
  Unmarked(u32),
  Marked(u32),
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
    let mut boards: Vec<Board> = Vec::new();
    let mut next_board = Vec::new();
    let mut id = 0;
    lines.next();
    for line in lines {
      if line.is_empty() {
        boards.push(Board {
          board: next_board.clone(),
          id,
        });
        next_board = Vec::new();
        id += 1;
        continue;
      }
      line
        .split(' ')
        .filter(|f| !f.is_empty())
        .map(|f| f.parse::<u32>().unwrap())
        .for_each(|v| next_board.push(Tile::Unmarked(v)));
    }
    boards.push(Board {
      board: next_board.clone(),
      id,
    });
    Ok(Box::new(Solve { calls, boards }))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let (_, score) =
      self
        .calls
        .iter()
        .fold((self.boards.clone(), None), |(boards, score), call| {
          if let Some(_) = score {
            return (boards, score);
          }
          let new_boards: Vec<Board> = boards.iter().map(|b| b.call(*call)).collect();
          let new_score = new_boards
            .iter()
            .find_map(|board| board.maybe_bingo().map(|b| b.score(*call)));
          (new_boards, new_score)
        });
    match score {
      Some(s) => Ok(Box::new(s)),
      None => Err("No Solve Found".into()),
    }
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut boards = HashMap::new();
    self.boards.iter().for_each(|b| {
      boards.insert(b.id, b.clone());
    });
    for call in &self.calls {
      let mut boards_in_play = boards.clone();
      for (_, board) in boards.iter_mut() {
        *board = board.call(*call);
        boards_in_play.insert(board.id, board.clone());
        if let Some(board) = board.maybe_bingo() {
          if boards_in_play.len() == 1 {
            return Ok(Box::new(board.score(*call)));
          }
          boards_in_play.remove(&board.id);
        }
      }
      boards = boards_in_play.clone();
    }
    Err("No solve found".into())
  }
}
