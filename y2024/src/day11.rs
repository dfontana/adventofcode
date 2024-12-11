use rust_util::Day;
use std::{collections::HashMap, error::Error, fmt::Display};

pub struct Solve {
    stones: HashMap<usize, usize>,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Solve {
            stones: value
                .split_whitespace()
                .filter_map(|v| v.parse::<usize>().ok())
                .fold(HashMap::new(), |mut acc, v| {
                    acc.entry(v).and_modify(|c| *c += 1).or_insert(1);
                    acc
                }),
        })
    }
}
impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(blink(&self.stones, 25)))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(blink(&self.stones, 75)))
    }
}

// Is there a better way?
fn digits(s: usize) -> u32 {
    let mut v = 10;
    let mut n = 1;
    loop {
        if s < v {
            return n;
        }
        n += 1;
        v *= 10;
    }
}

fn split(s: usize, len: u32) -> (usize, usize) {
    let div = 10_usize.pow(len / 2);
    let right = s % div;
    let left = s / div;
    (left, right)
}

fn blink(input: &HashMap<usize, usize>, blinks: usize) -> usize {
    let mut stones = input.clone();
    for _ in 0..blinks {
        let keys = stones
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<(usize, usize)>>();
        for (sp, cp) in keys {
            let s = sp;
            let count = cp;
            stones.entry(s).and_modify(|c| *c -= count);
            if s == 0 {
                stones.entry(1).and_modify(|c| *c += count).or_insert(count);
                continue;
            }

            let num_digits = digits(s);
            if num_digits % 2 == 0 {
                let (l, r) = split(s, num_digits);
                stones.entry(l).and_modify(|c| *c += count).or_insert(count);
                stones.entry(r).and_modify(|c| *c += count).or_insert(count);
                continue;
            }

            stones
                .entry(s * 2024)
                .and_modify(|c| *c += count)
                .or_insert(count);
        }
    }
    stones.values().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_digits() {
        assert_eq!(digits(1), 1);
        assert_eq!(digits(10), 2);
        assert_eq!(digits(100), 3);
    }

    #[test]
    fn t_split() {
        assert_eq!(split(10, 2), (1, 0));
        assert_eq!(split(1000, 4), (10, 0));
        assert_eq!(split(2024, 4), (20, 24));
        assert_eq!(split(24, 2), (2, 4));
    }
}
