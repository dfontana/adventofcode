use itertools::Itertools;
use rust_util::Day;
use std::{error::Error, fmt::Display};

pub struct Solve {
    value: String,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug)]
enum Op {
    MIA,
    Add,
    Mul,
    Num(usize),
}

fn eval(p: &Vec<Op>) -> usize {
    p.iter()
        .sorted()
        .fold((Op::MIA, 0), |mut acc, v| {
            match v {
                Op::Add | Op::Mul => acc.0 = *v,
                Op::Num(x) if acc.1 == 0 => acc.1 = *x,
                Op::Num(x) if acc.0 == Op::Add => acc.1 += *x,
                Op::Num(x) if acc.0 == Op::Mul => acc.1 *= *x,
                _ => unreachable!("Invalid problem"),
            };
            acc
        })
        .1
}

fn transpose(mut matrix: Vec<Vec<String>>) -> Vec<Vec<String>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }

    let cols = matrix[0].len();

    (0..cols)
        .map(|col| {
            matrix
                .iter_mut()
                .map(|row| std::mem::take(&mut row[col]))
                .collect()
        })
        .collect()
}

impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Solve { value })
    }
}
impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let lines = self
            .value
            .trim()
            .lines()
            .map(|s| {
                s.trim()
                    .split_whitespace()
                    .map(|s| s.to_owned())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let problems: Vec<Vec<Op>> = transpose(lines)
            .iter()
            .map(|p| {
                p.iter()
                    .map(|s| match s.as_str() {
                        "*" => Op::Mul,
                        "+" => Op::Add,
                        x => Op::Num(x.parse::<usize>().unwrap()),
                    })
                    .collect()
            })
            .collect();
        Ok(Box::new(problems.iter().map(|p| eval(p)).sum::<usize>()))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let (h, w) = self
            .value
            .trim_matches('\n')
            .lines()
            .map(|l| l.len())
            .enumerate()
            .fold((0, 0), |acc, v| (acc.0.max(v.0), acc.1.max(v.1)));

        let lines = self.value.trim_matches('\n').lines().collect_vec();
        let mut problems = Vec::new();
        let mut p = Vec::new();
        for c in 0..w {
            let col: String = (0..=h).filter_map(|r| lines[r].get(c..=c)).collect();
            if col.trim().is_empty() {
                problems.push(p.clone());
                p.clear();
            } else {
                p.extend(match &col[col.len() - 1..] {
                    "*" => vec![
                        Op::Num(col[..col.len() - 1].trim().parse::<usize>().unwrap()),
                        Op::Mul,
                    ],
                    "+" => vec![
                        Op::Num(col[..col.len() - 1].trim().parse::<usize>().unwrap()),
                        Op::Add,
                    ],
                    _ => vec![Op::Num(col.trim().parse::<usize>().unwrap())],
                });
            }
        }
        problems.push(p.clone());
        Ok(Box::new(problems.iter().map(|p| eval(p)).sum::<usize>()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
            "
        .to_string();
        let solve = Solve::try_from(input).unwrap();
        assert_eq!(
            format!("{}", solve.p1().unwrap()).parse::<i64>().unwrap(),
            4277556
        );
        assert_eq!(
            format!("{}", solve.p2().unwrap()).parse::<i64>().unwrap(),
            3263827
        );
    }
}
