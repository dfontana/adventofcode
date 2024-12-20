use rust_util::Day;
use std::{error::Error, fmt::Display};

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
        Ok(Box::new(1))
    }
}

fn is_possible(a: &str, types: &Vec<TType>) -> bool {
    if a.len() == 0 {
        return true;
    }
    for t in types {
        if a == t {
            return true;
        }
        if a.starts_with(t) && is_possible(&a[t.len()..], types) {
            return true;
        }
    }
    false
}
