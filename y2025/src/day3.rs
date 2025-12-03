use rust_util::Day;
use std::{error::Error, fmt::Display};

pub struct Solve {
    shelves: Vec<Vec<u8>>,
}

// ~330us
// But If we instead scan the sz up to sz-bz[i..] for each
// mi - because we can only pick a candidate that can allow the rest of the
// battery to fit - then we reduce our search space to some log, like szlogbz?
fn min_max_fast(shelf: &Vec<u8>, bz: usize) -> Vec<u8> {
    let mut bat = vec![0; bz];
    let mut st = 0;
    for m in 0..bz {
        let shelf_left = &shelf[st..shelf.len() - bz + m + 1];
        let mut max = (0, &shelf_left[0]);
        for i in shelf_left.iter().enumerate() {
            if i.1 > max.1 {
                max = i;
            }
        }
        st += max.0 + 1;
        bat[m] = *max.1;
    }
    bat
}

fn mk_bat(bat: &Vec<u8>, bz: usize) -> usize {
    bat.iter()
        .enumerate()
        .map(|(i, v)| 10_usize.pow((bz - 1 - i) as u32) * (*v as usize))
        .sum()
}

impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Solve {
            shelves: value
                .trim()
                .lines()
                .map(|l| l.trim().chars().map(|c| (c as u8) - 48).collect())
                .collect(),
        })
    }
}
impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let bz = 2;
        Ok(Box::new(
            self.shelves
                .iter()
                .map(|s| min_max_fast(s, bz))
                .map(|bat| mk_bat(&bat, bz))
                .sum::<usize>(),
        ))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let bz = 12;
        Ok(Box::new(
            self.shelves
                .iter()
                .map(|s| min_max_fast(s, bz))
                .map(|bat| mk_bat(&bat, bz))
                .sum::<usize>(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "
987654321111111
811111111111119
234234234234278
818181911112111
            "
        .to_string();
        let solve = Solve::try_from(input).unwrap();
        assert_eq!(
            format!("{}", solve.p1().unwrap()).parse::<i64>().unwrap(),
            357
        );
        assert_eq!(
            format!("{}", solve.p2().unwrap()).parse::<i64>().unwrap(),
            3121910778619
        );
    }
}
