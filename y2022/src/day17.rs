use crossterm::{cursor, terminal::{self, Clear, ClearType}, ExecutableCommand};
use rust_util::Day;
use std::{
  collections::HashSet,
  error::Error,
  fmt::Display,
  io::{stdout, Stdout, Write},
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

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> { 
    let mut std = stdout();
    // std.execute(Clear(ClearType::All)).unwrap();
    let stop_at_count = 2022;
    let pieces = [
      Piece::Horz,
      Piece::Plus,
      Piece::BackL,
      Piece::Vert,
      Piece::Box,
    ];
    let tot_jet = self.jets.len();
    let mut jet_cnt = 0;

    let mut top_of_stack = 1;
    let mut piece_pos = (3, top_of_stack + 3);

    let mut occupied: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..stop_at_count {
      // Pick rock shape to spawn
      let piece = &pieces[i % 5];

      printb(&mut std, &occupied, piece, &piece_pos);
      let mut settled = false;
      let mut step = 0;
      while !settled {
        match step % 2 {
          0 => {
            // Pushed by jet
            let (diff, chk) = match self.jets[jet_cnt % tot_jet] {
              '<' => (-1, 0),
              '>' => (1, piece.width()),
              _ => unreachable!("{:?}", self.jets[jet_cnt % tot_jet]),
            };
            let bound = piece_pos.0 + diff + chk;
            let new_pos = (piece_pos.0 + diff, piece_pos.1);
            let piece_occupies: Vec<_> = piece
              .offsets()
              .iter()
              .map(|(dx, dy)| (new_pos.0 + dx, new_pos.1 + dy))
              .collect();
            if (1..=7).contains(&bound) && !piece_occupies.iter().any(|pos| occupied.contains(pos))
            {
              printb(&mut std, &occupied, piece, &new_pos);
              piece_pos = new_pos
            } else {
              printb(&mut std, &occupied, piece, &(1000, 1000));
            }
            jet_cnt += 1;
          }
          1 => {
            // Fall down 1, or stop if it will go into something
            let new_pos = (piece_pos.0, piece_pos.1 - 1);
            let hits_piece = piece
              .offsets()
              .iter()
              .map(|(dx, dy)| (new_pos.0 + dx, new_pos.1 + dy))
              .any(|pos| occupied.contains(&pos));
            if new_pos.1 > 0 && !hits_piece {
              piece_pos = new_pos;
              printb(&mut std, &occupied, piece, &new_pos);
            } else {
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
              printb(&mut std, &occupied, piece, &(1000, 1000));
            }
          }
          _ => unreachable!(),
        }
        step += 1;
      }
      piece_pos = (3, top_of_stack + 4);
    }

    Ok(Box::new(top_of_stack.to_string()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new("y"))
  }
}

fn printb(
  stdout: &mut Stdout,
  occupied: &HashSet<(i32, i32)>,
  piece: &Piece,
  new_pos: &(i32, i32),
) {
  // stdout.execute(cursor::MoveUp(38)).unwrap();
  // stdout
  //   .execute(terminal::Clear(terminal::ClearType::FromCursorDown))
  //   .unwrap();

  // let piece_occupies: HashSet<_> = piece
  //   .offsets()
  //   .iter()
  //   .map(|(dx, dy)| (new_pos.0 + dx, new_pos.1 + dy))
  //   .collect();

  // for y in (0..=37).rev() {
  //   for x in 0..=8 {
  //     if x == 0 || x == 8 {
  //       write!(stdout, "|").unwrap();
  //     } else if y == 0 {
  //       write!(stdout, "-").unwrap();
  //     } else if occupied.contains(&(x, y)) {
  //       write!(stdout, "#").unwrap();
  //     } else if piece_occupies.contains(&(x, y)) {
  //       write!(stdout, "@").unwrap();
  //     } else {
  //       write!(stdout, ".").unwrap();
  //     }
  //   }
  //   writeln!(stdout).unwrap();
  // }
  // std::thread::sleep(std::time::Duration::from_millis(10));
}
