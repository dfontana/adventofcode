use rust_util::{Day};
use std::{error::Error, fmt::Display, thread, time::Duration};

pub struct Solve {
  p1: usize,
  p2: usize,
}

const NEIGHBORS: [(isize, isize); 8] = [
  (-1, -1),
  (1, 1),
  (-1, 1),
  (1, -1),
  (0, -1),
  (-1, 0),
  (0, 1),
  (1, 0),
];

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let mut octo = value 
      .lines()
      .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
      .collect();

    let mut flashes = 0;
    let mut cstep = 0;
    loop {
      cstep += 1;
      clear_board();
      let crt = step(&mut octo);
      if cstep <= 100 {
        flashes += crt;
      }
      print_board(&octo);
      thread::sleep(Duration::from_millis(30));
      if crt == 100 {
        break;
      }
    }

    Ok(Solve {
      p1: flashes,
      p2: cstep,
    })
  }
}


impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(self.p1))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(self.p2))
  }
}

fn step(octo: &mut Vec<Vec<u32>>) -> usize {
  octo
    .iter_mut()
    .for_each(|l| l.iter_mut().for_each(|v| *v += 1));
  let mut flashes = 0;
  loop {
    let mut flashed = false;
    for y in 0..10 {
      for x in 0..10 {
        let d = &mut octo[y][x];
        if *d > 9 {
          flashed = true;
          *d = 0;
          flashes += 1;
          NEIGHBORS.iter().for_each(|(xx, yy)| {
            octo
              .get_mut(y.overflowing_add(*yy as usize).0)
              .and_then(|l| l.get_mut(x.overflowing_add(*xx as usize).0))
              .map(|v| {
                if *v != 0 {
                  *v += 1;
                }
              });
          });
        }
      }
    }
    if !flashed {
      break;
    }
  }
  flashes
}

fn clear_board() {
  // Magic control sequence
  print!("\x1B[2J\x1B[1;1H");
}

fn print_board(octo: &Vec<Vec<u32>>) {
  octo.iter().for_each(|l| {
    l.iter().for_each(|d| print!("{}", d));
    println!();
  })
}
