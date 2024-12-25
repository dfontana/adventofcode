use itertools::Itertools;
use rust_util::Day;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fmt::Display,
};

#[derive(Debug)]
enum Gate {
    XOR,
    OR,
    AND,
}
type Signal = bool;
type Wire = String;
pub struct Solve {
    states: HashMap<Wire, Signal>,
    gates: Vec<(Wire, Gate, Wire, Wire)>,
    highest_output: String,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (states, wires) = value.split_once("\n\n").ok_or_else(|| "Malformed")?;
        let states = states
            .lines()
            .filter_map(|s| s.split_once(": "))
            .map(|(w, s)| {
                (
                    w.to_string(),
                    match s {
                        "1" => true,
                        "0" => false,
                        _ => unreachable!(),
                    },
                )
            })
            .collect();
        let gates = wires
            .lines()
            .map(|l| {
                let w1 = &l[0..3];
                let g = match &l[4..7] {
                    "XOR" => Gate::XOR,
                    "OR " => Gate::OR,
                    "AND" => Gate::AND,
                    x => unreachable!("Invalid gate: {}", x),
                };
                let w2 = l[7..11].trim();
                let w3 = &l[l.len() - 3..];
                (w1.to_string(), g, w2.to_string(), w3.to_string())
            })
            .collect();
        let highest_output = gates
            .iter()
            .filter(|(_, _, _, o)| o.starts_with('z'))
            .map(|(_, _, _, o)| o)
            .sorted_by(|a, b| a.0.cmp(b.0).reverse())
            .first();
        Ok(Solve {
            states,
            gates,
            highest_output,
        })
    }
}

impl Day for Solve {
    // 42049478636360 ~315us
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(resolve(&self.states, &self.gates)))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        // 4 pairs of gates (8 total outputs) are swapped incorrectly
        // No output got swapped twice; so it's quite literally 4 unique pairs.
        // Ans is the 8 involed output gates, sorted and CSV'd
        Ok(Box::new(1))
    }
}

fn resolve(states: &HashMap<Wire, Signal>, gates: &Vec<(Wire, Gate, Wire, Wire)>) -> i64 {
    let mut state = states.clone();
    let mut g_todo = VecDeque::from_iter(gates);
    while let Some(next) = g_todo.pop_front() {
        let (w1, gate, w2, out) = &next;
        let Some(s1) = state.get(w1) else {
            g_todo.push_back(next);
            continue;
        };
        let Some(s2) = state.get(w2) else {
            g_todo.push_back(next);
            continue;
        };
        let o1 = match gate {
            Gate::XOR => *s1 ^ *s2,
            Gate::OR => *s1 || *s2,
            Gate::AND => *s1 && *s2,
        };
        state.insert(out.clone(), o1);
    }
    state
        .iter()
        .filter(|(k, _)| k.starts_with('z'))
        .sorted_by(|a, b| a.0.cmp(b.0))
        .map(|(_, s)| if *s == true { 1 } else { 0 })
        .enumerate()
        .fold(0i64, |acc, (idx, s)| acc | s << idx)
}

fn find_wrong(gates: &Vec<(Wire, Gate, Wire, Wire)>) {
    let mut wrong = HashSet::new();
    todo!()
}

/*
https://en.wikipedia.org/wiki/Adder_(electronics)#Ripple-carry_adder

wrong = set()
for op1, op, op2, res in operations:
    if res[0] == "z" and op != "XOR" and res != highest_z:
        wrong.add(res)
    if (
        op == "XOR"
        and res[0] not in ["x", "y", "z"]
        and op1[0] not in ["x", "y", "z"]
        and op2[0] not in ["x", "y", "z"]
    ):
        wrong.add(res)
    if op == "AND" and "x00" not in [op1, op2]:
        for subop1, subop, subop2, subres in operations:
            if (res == subop1 or res == subop2) and subop != "OR":
                wrong.add(res)
    if op == "XOR":
        for subop1, subop, subop2, subres in operations:
            if (res == subop1 or res == subop2) and subop == "OR":
                wrong.add(res)
*/
