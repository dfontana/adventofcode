use itertools::Itertools;
use rust_util::Day;
use std::{error::Error, fmt::Display, ops::RangeInclusive};

pub struct Solve {
    ranges: Vec<RangeInclusive<usize>>,
    ids: Vec<usize>,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (ranges, values) = value.trim().split_once("\n\n").unwrap();

        Ok(Solve {
            ranges: ranges
                .lines()
                .filter_map(|l| l.split_once("-"))
                .map(|(l, r)| (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
                .map(|(s, e)| RangeInclusive::new(s, e))
                .collect(),
            ids: values
                .lines()
                .map(|s| s.parse::<usize>().unwrap())
                .collect(),
        })
    }
}
impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(
            self.ids
                .iter()
                .filter(|id| self.ranges.iter().any(|r| r.contains(id)))
                .count(),
        ))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        // Sort ranges by start index
        // For any ranges whose start is in the prior range, fuse into that range
        // map to size of ranges
        // sum.
        let mut ranges = self.ranges.clone();
        ranges.sort_by_key(|r| *r.start());
        Ok(Box::new(
            ranges
                .iter()
                .fold(vec![RangeInclusive::new(0, 0)], |mut acc, r| {
                    let last_idx = acc.len() - 1;
                    let last = &acc[last_idx];
                    if last.contains(r.start()) {
                        let last = RangeInclusive::new(*last.start(), *last.end().max(r.end()));
                        acc[last_idx] = last;
                    } else {
                        acc.push(r.clone());
                    }
                    acc
                })
                .iter()
                .filter_map(|r| r.try_len().ok())
                .sum::<usize>()
                - 1,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"
        .to_string();
        let solve = Solve::try_from(input).unwrap();
        assert_eq!(
            format!("{}", solve.p1().unwrap()).parse::<i64>().unwrap(),
            3
        );
        assert_eq!(
            format!("{}", solve.p2().unwrap()).parse::<i64>().unwrap(),
            14
        );
    }
}
