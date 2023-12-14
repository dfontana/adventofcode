use rust_util::Day;
use std::{error::Error, fmt::Display};
use tracing::debug;

#[derive(Debug)]
pub struct Solve {
  input: Vec<(Vec<SpaState>, Vec<usize>)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum SpaState {
  OK,
  BR,
  UK,
}

impl From<char> for SpaState {
  fn from(value: char) -> Self {
    match value {
      '.' => SpaState::OK,
      '#' => SpaState::BR,
      '?' => SpaState::UK,
      _ => unreachable!(),
    }
  }
}

fn parse_spa(s: &str) -> (Vec<SpaState>, Vec<usize>) {
  let (spa, grp) = s.split_once(' ').unwrap();
  (
    spa.chars().map(SpaState::from).collect(),
    grp
      .split(',')
      .map(|c| c.parse::<usize>().unwrap())
      .collect(),
  )
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      input: value.lines().map(parse_spa).collect(),
    })
  }
}

fn combinations((spas, grps): &(Vec<SpaState>, Vec<usize>)) -> usize {
  let num_grps = grps.len();
  let num_spas = spas.len();

  let mut combos = 0;

  let mut frontier = vec![(spas.clone(), 0, 0)];
  while let Some((spa, mut offset, grp_idx)) = frontier.pop() {
    // Layout logic
    let grp = grps[grp_idx];
    debug!("0: POP {:?} <- {:?} ({:?})", disp(&spa), grp, offset);
    'outer: while offset < num_spas && offset + grp <= num_spas {
      let mut nspa = spa.clone();

      debug!("\ta: {:?} <- {:?} ({:?})", disp(&nspa), grp, offset);

      for i in offset..offset + grp {
        if nspa[i] == SpaState::OK {
          // This layout is no good, it bisects '.'
          debug!("\tf: {:?} <> {:?}", disp(&nspa), i);
          offset += 1;
          continue 'outer;
        }
        nspa[i] = SpaState::BR;
      }

      debug!("\tb: {:?}", disp(&nspa));

      if grp_idx != num_grps - 1 && offset + grp + 1 >= num_spas {
        // This layout is no good, more groups to go and not enough space to add another
        debug!(
          "\tc: {:?}!={:?} && {:?}+{:?}+2 >= {:?}",
          grp_idx, num_grps, offset, grp, num_spas
        );
        offset += 1;
        continue;
      }

      if nspa
        .get(offset + grp)
        .filter(|c| **c == SpaState::BR)
        .is_some()
        || offset
          .checked_sub(1)
          .and_then(|c| nspa.get(c))
          .filter(|c| **c == SpaState::BR)
          .is_some()
      {
        debug!("\td: {:?} == #", offset + grp);
        // This layout is no good, as we've made too long a chain
        offset += 1;
        continue;
      }

      if grp_idx >= num_grps - 1 {
        debug!("\tVALID {:?}", disp(&nspa));
        combos += 1;
      } else {
        debug!(
          "   0: PUSH {:?} <- {:?} ({:?})",
          disp(&nspa),
          grps[grp_idx + 1],
          offset + grp + 1
        );
        frontier.push((nspa, offset + grp + 1, grp_idx + 1))
      }

      offset += 1;
    }
  }

  // attempt to lay down the first group, if "valid" push to "frontier"
  // "valid":
  //    1) must not bisect any '.'
  //    2) if more there's another group left, must have '.' at end
  //    3) must be the exact right size
  // increase "offset" by 1 & repeat above
  // stop when offset == vec len

  // then move to next item in group and start from 1 space after end
  combos
}

fn disp(spas: &Vec<SpaState>) -> String {
  let mut s = String::new();
  for sa in spas {
    let p = match sa {
      SpaState::OK => '.',
      SpaState::BR => '#',
      SpaState::UK => '?',
    };
    s.push(p);
  }
  s
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(self.input.iter().map(combinations).sum::<usize>()))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(1))
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use tracing_test::traced_test;

  #[traced_test]
  #[test]
  fn case1() {
    assert_eq!(combinations(&parse_spa("???.### 1,1,3")), 1);
  }

  #[traced_test]
  #[test]
  fn case2() {
    assert_eq!(combinations(&parse_spa(".??..??...?##. 1,1,3")), 4);
  }

  #[traced_test]
  #[test]
  fn case3() {
    assert_eq!(combinations(&parse_spa("?#?#?#?#?#?#?#? 1,3,1,6")), 1);
  }

  #[traced_test]
  #[test]
  fn case4() {
    assert_eq!(combinations(&parse_spa("????.#...#... 4,1,1")), 1);
  }

  #[traced_test]
  #[test]
  fn case5() {
    assert_eq!(combinations(&parse_spa("????.######..#####. 1,6,5")), 4);
  }

  #[traced_test]
  #[test]
  fn case6() {
    assert_eq!(combinations(&parse_spa("?###???????? 3,2,1")), 10);
  }
}
