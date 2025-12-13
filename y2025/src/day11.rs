use rust_util::Day;
use std::{collections::HashMap, error::Error, fmt::Display};

pub struct Solve {
    nodes: HashMap<String, Vec<String>>,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Solve {
            nodes: value
                .trim()
                .lines()
                .filter_map(|s| s.split_once(": "))
                .map(|(from, to)| {
                    (
                        from.to_string(),
                        to.split_whitespace().map(|s| s.to_string()).collect(),
                    )
                })
                .collect(),
        })
    }
}
impl Solve {
    fn count_paths(&self, current: &str, memo: &mut HashMap<String, usize>) -> usize {
        if current == "out" {
            return 1;
        }
        if let Some(&cached) = memo.get(current) {
            return cached;
        }
        let total = self
            .nodes
            .get(current)
            .map(|neighbors| {
                neighbors
                    .iter()
                    .map(|next| self.count_paths(next.as_str(), memo))
                    .sum()
            })
            .unwrap_or(0);
        memo.insert(current.to_string(), total);
        total
    }
}
impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let mut memo = HashMap::new();
        let count = self.count_paths("you", &mut memo);
        Ok(Box::new(count))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(1))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
            "
        .to_string();
        let solve = Solve::try_from(input).unwrap();
        assert_eq!(
            format!("{}", solve.p1().unwrap()).parse::<i64>().unwrap(),
            5
        );
        assert_eq!(
            format!("{}", solve.p2().unwrap()).parse::<i64>().unwrap(),
            1
        );
    }
}
