use rust_util::Day;
use std::{error::Error, fmt::Display, ops::RangeInclusive};

pub struct Solve {
    ranges: Vec<RangeInclusive<usize>>,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Solve {
            ranges: value
                .trim()
                .split(',')
                .filter_map(|s| s.split_once("-"))
                .map(|(a, b)| {
                    RangeInclusive::new(a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
                })
                .collect(),
        })
    }
}

fn places(n: usize) -> u32 {
    let mut nref = n;
    let mut pl = 0;
    while nref > 0 {
        pl += 1;
        nref /= 10;
    }
    pl
}

/// Take the first p places from n (left-to-right tho) and repeat that number
/// until a length of p_n is hit. Return said number.
fn num(n: usize, p: u32, p_n: u32) -> usize {
    let mask = 10usize.pow(p_n - p);
    let base = n / mask;
    let mut fin = base;
    for offset in 1..p_n / p {
        let mask = 10usize.pow(offset * p);
        fin += mask * base
    }
    fin
}

impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let mut sum = 0;
        for r in self.ranges.iter() {
            // Only need to check the first "half" of the number range
            // since we're doubling them; and skip non-even placed numbers
            for s in r.clone() {
                let p_n = places(s);
                let p = p_n.div_ceil(2);
                if p_n / p != 2 {
                    // Can only repeat once
                    continue;
                }
                let n = num(s, p, p_n);
                if s != n {
                    continue;
                }
                sum += n;
            }
        }
        Ok(Box::new(sum))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let mut sum = 0;
        for r in self.ranges.iter() {
            for s in r.clone() {
                let p_n = places(s);
                // So in this version we just need to test all possible repetition
                // options until one yields
                for p in 1..p_n {
                    if p_n % p != 0 {
                        // No good, it won't evenly create a pattern
                        continue;
                    }
                    // See if repeating p digits makes this number
                    let n: usize = num(s, p, p_n);
                    if n != s {
                        continue;
                    }
                    sum += n;
                    break;
                }
            }
        }
        Ok(Box::new(sum))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tplaces() {
        assert_eq!(places(100), 3);
        assert_eq!(places(1000), 4);
    }

    #[test]
    fn tnum() {
        assert_eq!(num(1188511880, 5, 10), 1188511885);
        assert_eq!(num(123456, 1, 6), 111111);
        assert_eq!(num(123456, 2, 6), 121212);
        assert_eq!(num(123456, 3, 6), 123123);
    }

    #[test]
    fn example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            .to_string();
        let solve = Solve::try_from(input).unwrap();
        assert_eq!(
            format!("{}", solve.p1().unwrap()).parse::<i64>().unwrap(),
            1227775554
        );
        assert_eq!(
            format!("{}", solve.p2().unwrap()).parse::<i64>().unwrap(),
            4174379265
        );
    }
}
