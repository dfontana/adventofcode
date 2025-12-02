use rust_util::Day;
use std::{error::Error, fmt::Display};

pub struct Solve {
    value: Vec<isize>,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Solve {
            value: value
                .lines()
                .map(|v| v.split_at(1))
                .map(|(a, b)| {
                    let val = b.parse::<isize>().unwrap();
                    match a {
                        "L" => -val,
                        "R" => val,
                        _ => unreachable!("Got: {}", a),
                    }
                })
                .collect(),
        })
    }
}
impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let cnt = self
            .value
            .iter()
            .fold((0, 50), |mut acc, v| {
                acc.1 = (acc.1 + v).rem_euclid(100);
                if acc.1 == 0 {
                    acc.0 += 1
                }
                acc
            })
            .0;
        Ok(Box::new(cnt))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let cnt = self
            .value
            .iter()
            .fold((0, 50), |mut acc, v| {
                let next = (acc.1 + v).rem_euclid(100);
                if (*v < 0 && next > acc.1 && acc.1 != 0) || (*v > 0 && next < acc.1 && next != 0) {
                    acc.0 += 1;
                }
                acc.1 = next;
                if acc.1 == 0 {
                    acc.0 += 1;
                }
                acc.0 += v.abs() / 100;
                acc
            })
            .0;
        Ok(Box::new(cnt))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82".to_string();
        let solve = Solve::try_from(input).unwrap();
        assert_eq!(
            format!("{}", solve.p1().unwrap()).parse::<i64>().unwrap(),
            3
        );
        assert_eq!(
            format!("{}", solve.p2().unwrap()).parse::<i64>().unwrap(),
            6
        );
    }
}
