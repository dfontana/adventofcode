use itertools::Itertools;
use rust_util::Day;
use std::{error::Error, fmt::Display, ops::Sub};

pub struct Solve {
    left: Vec<i64>,
    right: Vec<i64>,
}

impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (left, right) = value
            .lines()
            .filter_map(|l| l.split_once("   "))
            .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
            .fold((Vec::new(), Vec::new()), |mut acc, (l, r)| {
                acc.0.push(l);
                acc.1.push(r);
                acc
            });
        Ok(Solve { left, right })
    }
}

impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let ans = self
            .left
            .iter()
            .sorted()
            .zip(self.right.iter().sorted())
            .map(|(l, r)| l.max(&r).sub(l.min(&r)))
            .sum::<i64>();
        Ok(Box::new(ans))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let sim_score = self
            .right
            .iter()
            .map(|v| (v, 1))
            .into_grouping_map()
            .aggregate(|acc, _key, val| acc.or_else(|| Some(0)).map(|v| v + val));
        let ans = self
            .left
            .iter()
            .filter_map(|v| sim_score.get(v).map(|sim| sim * v))
            .sum::<i64>();
        Ok(Box::new(ans))
    }
}
