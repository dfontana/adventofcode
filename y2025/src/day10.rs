use itertools::Itertools;
use rust_util::Day;
use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fmt::Display,
};

// Don't forget these are "backwards" (right to left)
type Indicators = u16; // Bit array; [.##.] == 0110
type Button = u16; // Bit array; (0,2,3,4) == 10111
type Joltage = u128; // u9 array (up to 10, 9 bits wide each for 512 max value)

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

fn parse_joltage(v: &str) -> Joltage {
    v[1..v.len() - 1]
        .split(",")
        .enumerate()
        .map(|(i, c)| c.parse::<u128>().unwrap() << (9 * i))
        .fold(0, |acc, i| acc | i)
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
                    let jolt = parse_joltage(segments[len - 1]);
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
                .map(|(i, bs, _)| solve_indicator(*i, bs))
                .sum::<usize>(),
        ))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(
            self.input
                .iter()
                .map(|(_, bs, j)| solve_joltage(*j, bs))
                .sum::<usize>(),
        ))
    }
}

fn solve_indicator(ind: Indicators, buttons: &Vec<Button>) -> usize {
    let mut frontier = VecDeque::from_iter([(ind, 0)]);
    let mut seen = HashSet::new();
    while let Some((s, bs)) = frontier.pop_front() {
        let mut this_loop = Vec::new();
        for b in buttons {
            let ns = b ^ s;
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

fn to_str(r: u128, num: usize) -> String {
    let s = (0..num)
        .map(|s| format!("{}", 0b111111111 & (r >> (s * 9))))
        .join(",");
    format!("{{{s}}}")
}

fn press_jolt(bs: &Button, jolt: Joltage) -> Option<Joltage> {
    // b is an array of buttons, you have to do this per 1 bit in b
    let mut acc = jolt;

    let mut bs = *bs;
    let mut b = 0;
    while bs != 0 {
        while bs & (1 << b) == 0 {
            b += 1;
        }
        bs = bs & !(1 << b);
        // And press it
        let s = b * 9;
        let mask = 0b111111111 << s;
        let old_value = (mask & jolt) >> s;
        let new_value = old_value.checked_sub(1)?;
        acc = (acc & !mask) | ((new_value << s) & mask);
    }
    Some(acc)
}

// TODO: This is way too inefficient when joltage values are high. It's branching
//       button number of times and onyl decrementing 1 at a time. You'll need a different
//       approach, possibly one based on factoring the joltage values && the buttons?
fn solve_joltage(jolt: Joltage, buttons: &Vec<Button>) -> usize {
    // Buttons add 1 now && need to find the combo that adds to exact joltage sums
    // so going over immeditely DQ's that solution. That also means no cycles possible
    let mut frontier = VecDeque::from_iter([(jolt, 0)]);
    println!("----NEW SOLVE----");
    while let Some((s, bs)) = frontier.pop_front() {
        println!(
            "Pressed: {bs} times to get {}, backlog: {}",
            to_str(s, buttons.len()),
            frontier.len()
        );
        for b in buttons {
            match press_jolt(b, s) {
                None => continue,
                Some(ns) if ns == 0 => return bs + 1,
                Some(ns) => frontier.push_back((ns, bs + 1)),
            }
        }
    }
    unreachable!("Could not solve row")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_press_jolt() {
        let jolt = parse_joltage("{3,5,4,7}");
        let button = parse_button(&["(0,1,2,3)"])[0];
        let res = press_jolt(&button, jolt).unwrap();
        assert_eq!(to_str(res, 4), "{2,4,3,6}");
    }

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
            33
        );
    }
}
