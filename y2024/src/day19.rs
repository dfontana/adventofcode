use rust_util::Day;
use std::{collections::HashMap, error::Error, fmt::Display};

type TType = String;
type Arrangement = String;
pub struct Solve {
    types: Vec<TType>,
    arrangements: Vec<Arrangement>,
}

impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut lines = value.lines();
        let types: Vec<TType> = lines
            .next()
            .map(|s| s.split(", ").map(|s| s.to_string()).collect())
            .unwrap();
        lines.next();
        let arrangements: Vec<Arrangement> = lines.map(|s| s.to_string()).collect();
        Ok(Solve {
            types,
            arrangements,
        })
    }
}

impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(
            self.arrangements
                .iter()
                .filter(|a| is_possible(a, &self.types))
                .count(),
        ))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let mut memo = HashMap::new();
        Ok(Box::new(
            self.arrangements
                .iter()
                .map(|a| count_possible(a, &mut memo, &self.types))
                .sum::<usize>(),
        ))
    }
}

fn is_possible(a: &str, types: &Vec<TType>) -> bool {
    if a.len() == 0 {
        return true;
    }
    for t in types {
        if a.starts_with(t) && is_possible(&a[t.len()..], types) {
            return true;
        }
    }
    false
}

fn count_possible<'a>(a: &'a str, memo: &mut HashMap<&'a str, usize>, types: &Vec<TType>) -> usize {
    if a.len() == 0 {
        return 1;
    }
    let mut total_subbranches = 0;
    for t in types {
        if a.starts_with(t) {
            let key = &a[t.len()..];
            let sub_count = match memo.get(key) {
                Some(v) => *v,
                None => {
                    let sub_count = count_possible(key, memo, types);
                    memo.insert(key, sub_count);
                    sub_count
                }
            };
            total_subbranches += sub_count;
        }
    }
    total_subbranches
}
