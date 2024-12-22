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
            self.snums
                .iter()
                .map(|n| (0..2000).fold(*n, |acc, _| secret(acc)))
                .sum::<i64>(),
        ))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(most_banana(
            &self.snums.iter().map(|n| price_deltas(*n, 2000)).collect(),
        )))
    }
}

type Sequence = [i64; 4];
fn most_banana(deltas: &Vec<HashMap<Sequence, i64>>) -> i64 {
    // Naively: We could just track the total earned from each sequence encountered.
    // Worst case that means we're storing all unique permutations of (-9, 9) where elements
    // can repeat: 19^4 = 130,321 entries of i64, which fits in memory.
    let mut seq_yield = HashMap::new();

    for delta in deltas {
        for (seq, amt) in delta.iter() {
            seq_yield
                .entry(*seq)
                .and_modify(|e| *e += *amt)
                .or_insert(*amt);
        }
    }

    *seq_yield.values().max().expect("Deltas must exist")
}

fn price_deltas(num: i64, times: usize) -> HashMap<Sequence, i64> {
    let mut num = num;
    let mut p_price = num % 10;
    let mut seq_yield = HashMap::with_capacity(times);
    let mut k = [0; 4];
    for i in 0..times {
        num = secret(num);
        let price = num % 10;
        let change = price - p_price;
        if i < 3 {
            k[i] = change;
        } else if i == 3 {
            k[i] = change;
            seq_yield.entry(k).or_insert(price);
        } else {
            k[0] = k[1];
            k[1] = k[2];
            k[2] = k[3];
            k[3] = change;
            seq_yield.entry(k).or_insert(price);
        }
        p_price = price;
    }
    seq_yield
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
