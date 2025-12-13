use itertools::Itertools;
use rust_util::Day;
use std::{error::Error, fmt::Display};

pub struct Solve {
    regions: Vec<(usize, usize, Vec<usize>)>,
}

impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let chunks = value.split("\n\n").collect_vec();
        let regions = chunks[chunks.len() - 1];
        Ok(Solve {
            regions: regions
                .lines()
                .filter_map(|l| l.split_once(": "))
                .filter_map(|(s, n)| s.split_once("x").map(move |s| (s, n)))
                .map(|((w, l), ns)| {
                    (
                        w.parse::<usize>().unwrap(),
                        l.parse::<usize>().unwrap(),
                        ns.split_whitespace()
                            .map(|n| n.parse::<usize>().unwrap())
                            .collect(),
                    )
                })
                .collect(),
        })
    }
}

impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(
            self.regions
                .iter()
                .filter(|(w, l, bxs)| w * l >= bxs.iter().sum::<usize>() * 9)
                .count(),
        ))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(1))
    }
}
