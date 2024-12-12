use rust_util::{grid::Grid, Day};
use std::{error::Error, fmt::Display};

pub struct Solve {
    grid: Grid<char>,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Solve {
            grid: Grid::new_from(value),
        })
    }
}
impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(1))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(1))
    }
}

// High level ideas:
// 1. Build a map of each unique character seen to all the loc's where they are (this makes "layers")
// 2. BFS each character ("layer") to find all neighbors:
//    - Start the frontier with _all_ locs of that character present. Each entry should be a vec to track the "paths"
//    - Track a seen list, as you pop items off if they are in the seen then another item already has it as a neighor. Drop it.
//    - Expand to any neighbor locs that exist & continue...
//    - (How can you tell all possible neighbors have been seen vs a dead end?)
// ...
// BFS might not be the right algo. You want something like it though. Keep thinking.

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "".to_string();
        let solve = Solve::try_from(input).unwrap();
        assert_eq!(
            format!("{}", solve.p1().unwrap()).parse::<i64>().unwrap(),
            1
        );
        assert_eq!(
            format!("{}", solve.p2().unwrap()).parse::<i64>().unwrap(),
            1
        );
    }
}
