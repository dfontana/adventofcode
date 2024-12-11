use rust_util::Day;
use std::{collections::VecDeque, error::Error, fmt::Display};

pub struct Solve {
    stones: Vec<usize>,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Solve {
            stones: value
                .split_whitespace()
                .filter_map(|v| v.parse::<usize>().ok())
                .collect(),
        })
    }
}
impl Day for Solve {
    // 229043
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(blink(self.stones.clone(), 25)))
    }

    // TBD
    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        // let mul = 75;
        let mul = 1;
        Ok(Box::new(blink(self.stones.clone(), mul)))
    }
}

// TODO: This is horrible for performance
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

fn blink(input: Vec<usize>, blinks: usize) -> usize {
    // Obviously will blow up space. How can I compress this?
    // - Is there a way to know the number of digits and only track the
    //   number of stones with a number of digits?
    let mut stones = VecDeque::from_iter(input);
    for _ in 0..blinks {
        for _ in 0..stones.len() {
            if let Some(s) = stones.pop_front() {
                if s == 0 {
                    stones.push_back(1);
                    continue;
                }

                let num_digits = digits(s);
                if num_digits % 2 == 0 {
                    let (l, r) = split(s, num_digits);
                    stones.push_back(r);
                    stones.push_back(l);
                    continue;
                }

                stones.push_back(s * 2024);
            }
        }
    }
    stones.len()
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
