use itertools::Itertools;
use rust_util::Day;
use std::{
  collections::{HashMap, VecDeque},
  error::Error,
  fmt::Display,
};

#[derive(Debug)]
pub struct Solve {
  mods: HashMap<String, Module>,
  init: Vec<String>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let mut init = Vec::new();
    let mut outs_ins: HashMap<String, Vec<String>> = HashMap::new();
    let mut mods: HashMap<_, _> = value
      .lines()
      .filter_map(|l| l.split_once(" -> "))
      .filter_map(|(tyid, outs)| {
        let out = outs.split(',').map(|s| s.trim().to_string()).collect_vec();
        if tyid == "broadcaster" {
          init.extend(out);
          None
        } else {
          let typ = &tyid[0..1];
          let id = &tyid[1..];
          for ot in out.iter() {
            outs_ins
              .entry(ot.to_string())
              .and_modify(|v| {
                v.push(id.to_string());
              })
              .or_insert(vec![id.to_string()]);
          }
          Some((
            id.to_string(),
            match typ {
              "%" => Module::flip(out),
              "&" => Module::cjn(out),
              _ => unreachable!(),
            },
          ))
        }
      })
      .collect();

    // Update all inputs on CJNs
    for (id, md) in mods.iter_mut() {
      if let Some(ins) = outs_ins.get(id) {
        md.set_inputs(ins.clone());
      }
    }

    Ok(Solve { mods, init })
  }
}

#[derive(Clone, Debug)]
enum Module {
  CJN(HashMap<String, bool>, Vec<String>),
  FLP(bool, Vec<String>),
}

impl Module {
  fn cjn(outputs: Vec<String>) -> Module {
    Module::CJN(HashMap::new(), outputs)
  }

  fn flip(outputs: Vec<String>) -> Module {
    Module::FLP(false, outputs)
  }

  fn set_inputs(&mut self, ins: Vec<String>) {
    match self {
      Module::CJN(mp, _) => {
        for inp in ins.iter() {
          mp.insert(inp.to_string(), false);
        }
      }
      Module::FLP(_, _) => {}
    }
  }

  fn send(&mut self, from: &String, pulse: bool) -> Vec<(String, bool)> {
    match self {
      Module::CJN(inp, out) => {
        inp.insert(from.to_owned(), pulse);
        let pls = inp.values().map(|p| *p).reduce(|a, b| a && b).unwrap();
        out.iter().map(|id| (id.to_string(), !pls)).collect()
      }
      Module::FLP(state, out) if !pulse => {
        *state = !*state;
        out.iter().map(|id| (id.to_string(), *state)).collect()
      }
      _ => Vec::new(),
    }
  }
}

impl Solve {
  fn push_button(&self, mod_state: &mut HashMap<String, Module>) -> (usize, usize) {
    let mut low_cnt = self.init.len() + 1; // Button counts too
    let mut high_cnt = 0;

    let mut pulses = VecDeque::from_iter(
      self
        .init
        .iter()
        .map(|m| ("bcst".to_string(), false, m.clone())),
    );
    while let Some((from, pulse, to)) = pulses.pop_front() {
      // println!("{} -{:?}-> {} ({}, {})", from, pulse, to, low_cnt, high_cnt);
      let Some(module) = mod_state.get_mut(&to) else {
        continue;
      };

      for (mid, pulse) in module.send(&from, pulse).iter() {
        match pulse {
          true => high_cnt += 1,
          false => low_cnt += 1,
        };
        pulses.push_back((to.clone(), *pulse, mid.clone()));
      }
    }
    (low_cnt, high_cnt)
  }
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut mod_state = self.mods.clone();
    Ok(Box::new(
      (0..1000)
        .map(|_| self.push_button(&mut mod_state))
        .reduce(|(low, high), (nl, nh)| (low + nl, high + nh))
        .map(|(low, high)| low * high)
        .unwrap(),
    ))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(1))
  }
}
