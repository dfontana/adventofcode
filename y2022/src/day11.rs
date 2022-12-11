use itertools::Itertools;
use rust_util::Day;
use std::collections::VecDeque;
use std::{error::Error, fmt::Display};

pub struct Solve {
  monkeys: Vec<Monkey>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(_value: String) -> Result<Self, Self::Error> {
    let monkeys = vec![
      Monkey {
        items: VecDeque::from([71, 56, 50, 73]),
        operation: Operation::MulConst(11),
        test: Test {divide: 13, ttrue: 1, tfalse: 7,},
        inspected: 0,
      },
       Monkey {
        items: VecDeque::from([70, 89, 82]),
        operation: Operation::AddConst(1),
        test: Test {divide: 7, ttrue: 3, tfalse: 6,},
        inspected: 0,
      },
       Monkey {
        items: VecDeque::from([52, 95]),
        operation: Operation::Square,
        test: Test {divide: 3, ttrue: 5, tfalse: 4,},
        inspected: 0,
      },
       Monkey {
        items: VecDeque::from([94, 64, 69, 87, 70]),
        operation: Operation::AddConst(2),
        test: Test {divide: 19, ttrue: 2, tfalse: 6,},
        inspected: 0,
      },
       Monkey {
        items: VecDeque::from([98, 72, 98, 53, 97, 51]),
        operation: Operation::AddConst(6),
        test: Test {divide: 5, ttrue: 0, tfalse: 5,},
        inspected: 0,
      },
       Monkey {
        items: VecDeque::from([79]),
        operation: Operation::AddConst(7),
        test: Test {divide: 2, ttrue: 7, tfalse: 0,},
        inspected: 0,
      },
       Monkey {
        items: VecDeque::from([77, 55, 63, 93, 66, 90, 88, 71]),
        operation: Operation::MulConst(7),
        test: Test {divide: 11, ttrue: 2, tfalse: 4,},
        inspected: 0,
      },
       Monkey {
        items: VecDeque::from([54, 97, 87, 70, 59, 82, 59]),
        operation: Operation::AddConst(8),
        test: Test {divide: 17, ttrue: 1, tfalse: 3,},
        inspected: 0,
      },
    ];

    Ok(Solve { monkeys })
  }
}

#[derive(Clone)]
enum Operation {
  AddConst(i32),
  MulConst(i32),
  Square,
}

impl Operation {
  fn apply(&self, inp: i32) -> i32 {
    match self {
      Operation::AddConst(v) => inp + v,
      Operation::MulConst(v) => inp * v,
      Operation::Square => inp * inp,
    }
  }
}

#[derive(Clone)]
struct Test {
  divide: i32,
  ttrue: usize,
  tfalse: usize,
}

impl Test {
  fn apply(&self, inp: i32) -> usize {
    if inp % self.divide == 0 {
      self.ttrue
    } else {
      self.tfalse
    }
  }
}

#[derive(Clone)]
struct Monkey {
  items: VecDeque<i32>,
  operation: Operation,
  test: Test,
  inspected: usize,
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut monkeys = self.monkeys.clone();

    for _ in 0..20 {
      for id in 0..monkeys.len() {
        let mut new_monkeys = monkeys.clone();
        let monkey = &mut monkeys[id];
        while let Some(item) = monkey.items.pop_front() {
          let new_item = monkey.operation.apply(item) / 3;
          new_monkeys[monkey.test.apply(new_item)]
            .items
            .push_back(new_item);
          monkey.inspected += 1;
        }
        new_monkeys[id] = monkey.clone();
        monkeys = new_monkeys;
      }
    }

    let monkey_biz = monkeys
      .iter()
      .map(|mk| mk.inspected)
      .sorted()
      .rev()
      .take(2)
      .reduce(|acc, x| acc * x);
    Ok(Box::new(format!("{:?}", monkey_biz)))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new("y"))
  }
}
