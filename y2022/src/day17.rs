use crossterm::{
  cursor,
  terminal::{self, Clear, ClearType},
  ExecutableCommand,
};
use rust_util::Day;
use std::{
  collections::HashSet,
  error::Error,
  fmt::Display,
  io::{Stdout, Write},
};

pub struct Solve {
  jets: Vec<char>,
}
enum Piece {
  Box,
  Vert,
  Horz,
  Plus,
  BackL,
}
enum Move {
  Down,
  Lateral(i32),
}

impl Piece {
  fn height(&self) -> i32 {
    match self {
      Piece::Box => 1,
      Piece::Vert => 3,
      Piece::Horz => 0,
      Piece::Plus => 2,
      Piece::BackL => 2,
    }
  }

  fn width(&self) -> i32 {
    match self {
      Piece::Box => 1,
      Piece::Vert => 0,
      Piece::Horz => 3,
      Piece::Plus => 2,
      Piece::BackL => 2,
    }
  }

  fn offsets(&self) -> Vec<(i32, i32)> {
    match self {
      Piece::Box => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
      Piece::Vert => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
      Piece::Horz => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
      Piece::Plus => vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
      Piece::BackL => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    }
  }
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      jets: value.trim().chars().collect(),
    })
  }
}

fn simulate(std: &mut Option<Stdout>, jets: &Vec<char>, stop_at_count: usize) -> i32 {
  if let Some(stdt) = std {
    stdt.execute(Clear(ClearType::All)).unwrap();
  };
  let pieces = [
    Piece::Horz,
    Piece::Plus,
    Piece::BackL,
    Piece::Vert,
    Piece::Box,
  ];
  let tot_jet = jets.len();

  let mut jet_cnt = 0;
  let mut top_of_stack = 1;
  let mut piece_pos = (3, top_of_stack + 3);
  let mut occupied: HashSet<(i32, i32)> = HashSet::new();

  for i in 0..stop_at_count {
    let piece = &pieces[i % 5];

    printb(std, &occupied, piece, &piece_pos);

    let mut settled = false;
    let mut step = 0;
    while !settled {
      let (xd, yd, mov) = match (step % 2, jets[jet_cnt % tot_jet]) {
        (0, '<') => (-1, 0, Move::Lateral(0)),
        (0, '>') => (1, 0, Move::Lateral(piece.width())),
        (1, _) => (0, -1, Move::Down),
        _ => unreachable!(),
      };
      step += 1;

      let new_pos = (piece_pos.0 + xd, piece_pos.1 + yd);
      let hit_piece = piece
        .offsets()
        .iter()
        .map(|(dx, dy)| (new_pos.0 + dx, new_pos.1 + dy))
        .any(|pos| occupied.contains(&pos));
      let can_move = !hit_piece
        && match mov {
          Move::Lateral(chk) => (1..=7).contains(&(new_pos.0 + chk)),
          Move::Down => new_pos.1 > 0,
        };

      if let Move::Lateral(_) = mov {
        jet_cnt += 1;
      };

      if can_move {
        piece_pos = new_pos;
        printb(std, &occupied, piece, &new_pos);
        continue;
      }

      if let Move::Down = mov {
        settled = true;
        piece
          .offsets()
          .iter()
          .map(|(dx, dy)| (piece_pos.0 + dx, piece_pos.1 + dy))
          .for_each(|p| {
            occupied.insert(p);
          });
        if piece_pos.1 + piece.height() > top_of_stack {
          top_of_stack = piece_pos.1 + piece.height();
        }
        printb(std, &occupied, piece, &(1000, 1000));
      };
    }
    piece_pos = (3, top_of_stack + 4);
  }
  top_of_stack
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(simulate(&mut None, &self.jets, 2022).to_string()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    // TODO: Needs optimizing to complete this part for the full amount
    Ok(Box::new(simulate(&mut None, &self.jets, 2022).to_string()))
  }
}

fn printb(
  stdout: &mut Option<Stdout>,
  occupied: &HashSet<(i32, i32)>,
  piece: &Piece,
  new_pos: &(i32, i32),
) {
  let Some(stdout) = stdout else {
      return;
  };
  stdout.execute(cursor::MoveUp(38)).unwrap();
  stdout
    .execute(terminal::Clear(terminal::ClearType::FromCursorDown))
    .unwrap();

  let piece_occupies: HashSet<_> = piece
    .offsets()
    .iter()
    .map(|(dx, dy)| (new_pos.0 + dx, new_pos.1 + dy))
    .collect();

  for y in (0..=37).rev() {
    for x in 0..=8 {
      if x == 0 || x == 8 {
        write!(stdout, "|").unwrap();
      } else if y == 0 {
        write!(stdout, "-").unwrap();
      } else if occupied.contains(&(x, y)) {
        write!(stdout, "#").unwrap();
      } else if piece_occupies.contains(&(x, y)) {
        write!(stdout, "@").unwrap();
      } else {
        write!(stdout, ".").unwrap();
      }
    }
    writeln!(stdout).unwrap();
  }
  std::thread::sleep(std::time::Duration::from_millis(10));
}
