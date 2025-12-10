use rust_util::Day;
use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fmt::Display,
};

// Don't forget these are "backwards" (right to left)
type Indicators = u16; // Bit array; [.##.] == 0110
type Button = u16; // Bit array; (0,2,3,4) == 10111
type Joltage = String; // for now

pub struct Solve {
    input: Vec<(Indicators, Vec<Button>, Joltage)>,
}

fn parse_ind(v: &str) -> Indicators {
    v[1..v.len() - 1]
        .chars()
        .enumerate()
        .map(|(i, c)| match c {
            '#' => 1 << i,
            _ => 0,
        })
        .fold(0, |acc, i| acc | i)
}

fn parse_button(v: &[&str]) -> Vec<Button> {
    v.iter()
        .map(|s| {
            s[1..s.len() - 1]
                .split(",")
                .map(|c| c.parse::<u16>().unwrap())
                .map(|i| 1 << i)
                .reduce(|a, b| a | b)
                .unwrap()
        })
        .collect()
}

impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Solve {
            input: value
                .trim()
                .lines()
                .map(|l| {
                    let segments: Vec<&str> = l.split_whitespace().collect();
                    let len = segments.len();
                    let inds = parse_ind(segments[0]);
                    let buts = parse_button(&segments[1..len - 1]);
                    let jolt = segments[len - 1].to_string();
                    (inds, buts, jolt)
                })
                .collect(),
        })
    }
}
impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(
            self.input
                .iter()
                .map(|(i, bs, _)| solve_buttons(*i, bs))
                .sum::<usize>(),
        ))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(1))
    }
}

fn press(b: &Button, s: Indicators) -> Indicators {
    b ^ s
}

fn solve_buttons(ind: Indicators, buttons: &Vec<Button>) -> usize {
    let mut frontier = VecDeque::from_iter([(ind, 0)]);
    let mut seen = HashSet::new();
    while let Some((s, bs)) = frontier.pop_front() {
        let mut this_loop = Vec::new();
        for b in buttons {
            let ns = press(b, s);
            if seen.contains(&ns) {
                continue;
            }
            this_loop.push(ns);
            if ns == 0 {
                // Since we BFS it's assumed the first solution we find is the shortest
                return bs + 1;
            }
            // Pressing didn't get us there so let's push to the search
            frontier.push_back((ns, bs + 1));
        }
        seen.extend(this_loop);
    }
    unreachable!("Could not solve row")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
            "
        .to_string();
        let solve = Solve::try_from(input).unwrap();
        assert_eq!(
            format!("{}", solve.p1().unwrap()).parse::<i64>().unwrap(),
            7
        );
        assert_eq!(
            format!("{}", solve.p2().unwrap()).parse::<i64>().unwrap(),
            1
        );
    }
}
