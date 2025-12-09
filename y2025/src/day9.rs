use rust_util::Day;
use std::{error::Error, fmt::Display};

pub struct Solve {
    tiles: Vec<(usize, usize)>,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Solve {
            tiles: value
                .trim()
                .lines()
                .filter_map(|s| s.split_once(","))
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .collect(),
        })
    }
}
impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let mut max_area = 0;
        for i in 0..self.tiles.len() {
            for j in i..self.tiles.len() {
                let area = area(&self.tiles[i], &self.tiles[j]);
                if area > max_area {
                    max_area = area;
                }
            }
        }
        Ok(Box::new(max_area))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(1))
    }
}

fn area((x1, y1): &(usize, usize), (x2, y2): &(usize, usize)) -> usize {
    // +1 b/c the coordinates are "inclusive" so the length is off by 1
    (x2.max(x1) - x1.min(x2) + 1) * (y2.max(y1) - y1.min(y2) + 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
            "
        .to_string();
        let solve = Solve::try_from(input).unwrap();
        assert_eq!(
            format!("{}", solve.p1().unwrap()).parse::<i64>().unwrap(),
            50
        );
        assert_eq!(
            format!("{}", solve.p2().unwrap()).parse::<i64>().unwrap(),
            1
        );
    }
}
