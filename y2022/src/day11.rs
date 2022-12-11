use itertools::Itertools;
use rust_util::Day;
use std::collections::VecDeque;
use std::{error::Error, fmt::Display};

type Worry = usize;
pub struct Solve {
  monkeys: Vec<Monkey>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      monkeys: value
        .split("\n\n")
        .map(|block| {
          let mut lines = block.lines();
          lines.next(); // Burn the first line
          Monkey {
            items: lines
              .next()
              .unwrap()
              .trim_start_matches("  Starting items:")
              .split(',')
              .map(|v| v.trim().parse::<Worry>().unwrap())
              .collect::<VecDeque<Worry>>(),
            operation: Operation::from(lines.next().unwrap()),
            test: Test::from((
              lines.next().unwrap(),
              lines.next().unwrap(),
              lines.next().unwrap(),
            )),
            inspected: 0,
          }
        })
        .collect(),
    })
  }
}

#[derive(Clone)]
enum Operation {
  AddConst(Worry),
  MulConst(Worry),
  Square,
}

impl From<&str> for Operation {
  fn from(value: &str) -> Self {
    let mut instr = value
      .trim_start_matches("  Operation: new = old ")
      .split(' ');
    match (instr.next().unwrap(), instr.next().unwrap()) {
      ("*", "old") => Operation::Square,
      ("*", v) => Operation::MulConst(v.parse::<Worry>().unwrap()),
      ("+", v) => Operation::AddConst(v.parse::<Worry>().unwrap()),
      _ => unreachable!("Invalid line: {}", value),
    }
  }
}

impl Operation {
  fn apply(&self, inp: Worry) -> Worry {
    match self {
      Operation::AddConst(v) => inp + v,
      Operation::MulConst(v) => inp * v,
      Operation::Square => inp * inp,
    }
  }
}

#[derive(Clone)]
struct Test {
  divide: Worry,
  ttrue: usize,
  tfalse: usize,
}

impl From<(&str, &str, &str)> for Test {
  fn from((div, tt, tf): (&str, &str, &str)) -> Self {
    Test {
      divide: div
        .trim_start_matches("  Test: divisible by ")
        .parse::<Worry>()
        .unwrap(),
      ttrue: tt
        .trim_start_matches("    If true: throw to monkey ")
        .parse::<usize>()
        .unwrap(),
      tfalse: tf
        .trim_start_matches("    If false: throw to monkey ")
        .parse::<usize>()
        .unwrap(),
    }
  }
}

impl Test {
  fn apply(&self, inp: Worry) -> usize {
    if inp % self.divide == 0 {
      self.ttrue
    } else {
      self.tfalse
    }
  }
}

#[derive(Clone)]
struct Monkey {
  items: VecDeque<Worry>,
  operation: Operation,
  test: Test,
  inspected: usize,
}

fn simulate(monkeys: &[Monkey], rounds: usize, reducer: impl Fn(Worry) -> Worry) -> usize {
  let mut monkeys = monkeys.to_owned();

  for _ in 0..rounds {
    for id in 0..monkeys.len() {
      let mut new_monkeys = monkeys.clone();
      let monkey = &mut monkeys[id];
      while let Some(item) = monkey.items.pop_front() {
        let new_item = reducer(monkey.operation.apply(item));
        new_monkeys[monkey.test.apply(new_item)]
          .items
          .push_back(new_item);
        monkey.inspected += 1;
      }
      new_monkeys[id] = monkey.clone();
      monkeys = new_monkeys;
    }
  }

  monkeys
    .iter()
    .map(|mk| mk.inspected)
    .sorted()
    .rev()
    .take(2)
    .reduce(|acc, x| acc * x)
    .unwrap()
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(format!(
      "{:?}",
      simulate(&self.monkeys, 20, |inp| inp / 3)
    )))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let reducer = self
      .monkeys
      .iter()
      .map(|mk| mk.test.divide)
      .reduce(|acc, x| acc * x)
      .unwrap();
    Ok(Box::new(format!(
      "{:?}",
      simulate(&self.monkeys, 10000, |inp| inp % reducer)
    )))
  }
}
