use crossterm::{cursor, terminal, ExecutableCommand};
use itertools::Itertools;
use rust_util::Day;
use std::io::{stdout, Stdout, Write};
use std::{collections::HashSet, error::Error, fmt::Display};

type Pnt = (i32, i32);
enum Instr {
  R(usize),
  L(usize),
  D(usize),
  U(usize),
}
impl Instr {
  fn amt(&self) -> usize {
    match self {
      Instr::R(v) | Instr::L(v) | Instr::D(v) | Instr::U(v) => *v,
    }
  }
}

impl From<(&str, &str)> for Instr {
  fn from((d, a): (&str, &str)) -> Instr {
    let v = a.parse::<usize>().expect("Could not parse step");
    match d {
      "R" => Instr::R(v),
      "L" => Instr::L(v),
      "D" => Instr::D(v),
      "U" => Instr::U(v),
      _ => unreachable!("Unknown str hit: {:?}", d),
    }
  }
}

pub struct Solve {
  instrs: Vec<Instr>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      instrs: value
        .lines()
        .filter_map(|l| l.split_once(' '))
        .map(Instr::from)
        .collect(),
    })
  }
}

struct StdoutPrinter<'a> {
  height: i32,
  width: i32,
  stdout: &'a mut Stdout,
  enabled: bool,
}

impl StdoutPrinter<'_> {
  fn step(&mut self, knots: &Vec<Pnt>) {
      if !self.enabled {
          return;
      }
    self
      .stdout
      .execute(cursor::MoveUp((self.height*2) as u16))
      .unwrap();
    self
      .stdout
      .execute(terminal::Clear(terminal::ClearType::FromCursorDown))
      .unwrap();

    for y in (-self.height..self.height).rev() {
      for x in -self.width..self.width {
        let mut written = false;
        for (i, (kx, ky)) in knots.iter().enumerate() {
          if x == *kx && y == *ky {
            write!(self.stdout, "{}", i).unwrap();
            written = true;
            break;
          }
        }
        if !written {
          write!(self.stdout, ".").unwrap();
        }
      }
      writeln!(self.stdout).unwrap();
    }

    std::thread::sleep(std::time::Duration::from_millis(30));
  }
}

fn simulate(printer: &mut StdoutPrinter, instrs: &Vec<Instr>, knot_count: usize) -> HashSet<Pnt> {
let mut t_positions: HashSet<Pnt> = HashSet::new();
    let mut knots = Vec::new();
    for _ in 0..knot_count {
        knots.push((0,0));
    }
    t_positions.insert(knots[knots.len() - 1]);
    printer.step(&knots);

    for instr in instrs.iter() {
      for _ in 0..instr.amt() {
        match instr {
          Instr::R(_) => knots[0].0 += 1,
          Instr::L(_) => knots[0].0 -= 1,
          Instr::D(_) => knots[0].1 -= 1,
          Instr::U(_) => knots[0].1 += 1,
        }
        printer.step(&knots);

        for (i1,i2) in (0..knots.len()).tuple_windows() {
          let lead_knot = knots[i1];
          let mut tail_knot = &mut knots[i2];
 
          let x_diff = lead_knot.0 - tail_knot.0;
          let y_diff = lead_knot.1 - tail_knot.1;
          if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
            continue;
          }
          match (x_diff, y_diff) {
            (0, y) => tail_knot.1 += y.clamp(-1, 1),
            (x, 0) => tail_knot.0 += x.clamp(-1, 1),
            (x, y) => {
              tail_knot.1 += y.clamp(-1, 1);
              tail_knot.0 += x.clamp(-1, 1);
            }
          }
        }
        printer.step(&knots);

        t_positions.insert(knots[knots.len() - 1]);
      }
    }
    return t_positions;
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut printer = StdoutPrinter {
      stdout: &mut stdout(),
      height: 20,
      width: 20,
      enabled: false,
    };
    let t_positions = simulate(&mut printer, &self.instrs, 2);
    Ok(Box::new(t_positions.len()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut printer = StdoutPrinter {
      stdout: &mut stdout(),
      height: 20,
      width: 20,
      enabled: false,
    };
    let t_positions = simulate(&mut printer, &self.instrs, 10);
    Ok(Box::new(t_positions.len()))
  }
}
