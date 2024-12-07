use rust_util::Day;
use std::{collections::VecDeque, error::Error, fmt::Display};

pub struct Solve {
    eqs: Vec<Equation>,
}

struct Equation {
    total: i64,
    operands: Vec<i64>,
}

enum Operator {
    Add,
    Mul,
    Cat,
}

impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Solve {
            eqs: value
                .lines()
                .filter_map(|l| {
                    let (total, operands) = l.split_once(": ")?;
                    Some(Equation {
                        total: total.parse::<i64>().ok()?,
                        operands: operands
                            .split_ascii_whitespace()
                            .filter_map(|v| v.parse::<i64>().ok())
                            .collect(),
                    })
                })
                .collect(),
        })
    }
}
impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(sum_solvable_totals(
            &self.eqs,
            &vec![Operator::Add, Operator::Mul],
        )))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(sum_solvable_totals(
            &self.eqs,
            &vec![Operator::Add, Operator::Mul, Operator::Cat],
        )))
    }
}

fn sum_solvable_totals(eqs: &Vec<Equation>, ops: &Vec<Operator>) -> i64 {
    eqs.iter()
        .filter(|eq| eq.solvable(&ops))
        .map(|eq| eq.total)
        .sum::<i64>()
}

impl Equation {
    pub fn solvable(&self, operators: &Vec<Operator>) -> bool {
        let mut frontier = VecDeque::from_iter(vec![(0, &self.operands[..])]);
        while let Some((rtotal, ops)) = frontier.pop_front() {
            for operator in operators {
                let ntotal = match operator {
                    Operator::Add => rtotal + ops[0],
                    Operator::Mul => rtotal * ops[0],
                    Operator::Cat => format!("{}{}", rtotal, ops[0]).parse::<i64>().unwrap(),
                };
                if ops[1..].len() == 0 && ntotal == self.total {
                    return true;
                }
                if ops[1..].len() == 0 || ntotal > self.total {
                    continue;
                }
                frontier.push_back((ntotal, &ops[1..]));
            }
        }
        false
    }
}
