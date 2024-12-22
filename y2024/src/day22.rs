use rust_util::Day;
use std::{error::Error, fmt::Display};

pub struct Solve {
    snums: Vec<i64>,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Solve {
            snums: value
                .lines()
                .filter_map(|s| s.parse::<i64>().ok())
                .collect(),
        })
    }
}
impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(
            self.snums.iter().map(|n| rotate(*n, 2000)).sum::<i64>(),
        ))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(1))
    }
}

fn rotate(num: i64, times: usize) -> i64 {
    (0..times).fold(num, |acc, _| secret(acc))
}

fn secret(num: i64) -> i64 {
    let mut num = num;
    num ^= num * 64;
    num %= 16777216;
    num ^= num / 32;
    num %= 16777216;
    num ^= num * 2048;
    num % 16777216
}
