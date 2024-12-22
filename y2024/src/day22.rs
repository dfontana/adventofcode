use itertools::Itertools;
use rust_util::Day;
use std::{collections::HashMap, error::Error, fmt::Display};

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
        Ok(Box::new(most_banana(
            &self
                .snums
                .iter()
                .map(|n| deltas(&prices(*n, 2001)))
                .collect(),
        )))
    }
}

fn rotate(num: i64, times: usize) -> i64 {
    (0..times).fold(num, |acc, _| secret(acc))
}

fn most_banana(deltas: &Vec<Vec<(i64, i64)>>) -> i64 {
    // Naively: We could just track the total earned from each sequence encountered.
    // Worst case that means we're storing all unique permutations of (-9, 9) where elements
    // can repeat: 19^4 = 130,321 entries of i64, which fits in memory.
    let mut seq_yield = HashMap::new();

    for delta in deltas {
        for (seq, amt) in first_yields(delta).iter() {
            seq_yield
                .entry(*seq)
                .and_modify(|e| *e += *amt)
                .or_insert(*amt);
        }
    }

    *seq_yield.values().max().expect("Deltas must exist")
}

type Sequence = (i64, i64, i64, i64);
fn first_yields(delta: &Vec<(i64, i64)>) -> HashMap<Sequence, i64> {
    let mut seq_yield = HashMap::new();
    for (d1, d2, d3, d4) in delta.iter().tuple_windows() {
        seq_yield.entry((d1.1, d2.1, d3.1, d4.1)).or_insert(d4.0);
    }
    seq_yield
}

fn prices(num: i64, times: usize) -> Vec<i64> {
    let mut num = num;
    let mut ret = Vec::with_capacity(times);
    for _ in 0..times {
        ret.push(num % 10);
        num = secret(num);
    }
    ret
}

fn deltas(nums: &Vec<i64>) -> Vec<(i64, i64)> {
    let mut ret = Vec::with_capacity(nums.len());
    for i in 1..nums.len() {
        ret.push((nums[i], nums[i] - nums[i - 1]))
    }
    ret
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
