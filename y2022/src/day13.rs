use rust_util::Day;
use serde_json::Value;
use std::{cmp::Ordering, error::Error, fmt::Display};

pub struct Solve {
  pairs: Vec<(String, String)>,
}

impl TryFrom<String> for Solve {
  type Error = Box<dyn Error>;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Solve {
      pairs: value
        .split("\n\n")
        .filter_map(|l| l.split_once('\n'))
        .map(|(s1, s2)| (s1.to_owned(), s2.to_owned()))
        .collect(),
    })
  }
}

fn verify(left: Value, right: Value) -> Ordering {
  let (norm_left, norm_right) = match (&left, &right) {
    (Value::Number(_), Value::Array(_)) => return verify(Value::Array(vec![left]), right),
    (Value::Array(_), Value::Number(_)) => return verify(left, Value::Array(vec![right])),
    (Value::Number(v1), Value::Number(v2)) => return v1.as_u64().cmp(&v2.as_u64()),
    (Value::Array(lv), Value::Array(rv)) => (lv, rv),
    _ => unreachable!(),
  };

  if norm_left.is_empty() && norm_right.is_empty() {
    return Ordering::Equal;
  }
  for (i, v) in norm_left.iter().enumerate() {
    let Some(v2) = norm_right.get(i) else {
      return Ordering::Greater;
    };
    match verify(v.clone(), v2.clone()) {
      Ordering::Equal => continue,
      Ordering::Less => return Ordering::Less,
      Ordering::Greater => return Ordering::Greater,
    }
  }
  Ordering::Less
}

impl Day for Solve {
  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let solve: usize = self
      .pairs
      .iter()
      .enumerate()
      .filter(|(_, (s1, s2))| {
        let left: Value = serde_json::from_str(s1).expect("Valid s1");
        let right: Value = serde_json::from_str(s2).expect("Valid s2");
        verify(left, right) != Ordering::Greater
      })
      .map(|(i, _)| i + 1)
      .sum();
    Ok(Box::new(format!("{}", solve)))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut pairs = self.pairs.clone();
    pairs.push(("[[2]]".into(), "[[6]]".into()));
    let mut packets: Vec<Value> = pairs
      .iter()
      .flat_map(|(a, b)| vec![a, b])
      .map(|s| serde_json::from_str(s).unwrap())
      .collect();

    packets.sort_by(|l, r| verify(l.clone(), r.clone()));

    let solve = packets
      .iter()
      .enumerate()
      .filter_map(|(i, p)| {
        let Value::Array(a) = p else {
            return None;
        };
        if a.len() != 1 {
          return None;
        }
        let Value::Array(b) = &a[0] else {
            return None;
        };
        if b.len() != 1 {
          return None;
        }
        let Value::Number(v) = &b[0] else {
            return None;
        };
        if v.as_u64() == Some(2) || v.as_u64() == Some(6) {
          return Some(i + 1);
        }
        None
      })
      .reduce(|a, b| a * b);

    Ok(Box::new(format!("{:?}", solve)))
  }
}

#[cfg(test)]
mod tests {
  use rust_util::Day;

  #[test]
  fn idx_71() {
    let s = crate::day13::Solve {
      pairs: vec![("[[],[8]]".into(), "[[]]".into())],
    };
    assert_eq!(s.p1().unwrap().to_string(), "0".to_owned());
  }

  #[test]
  fn idx_78() {
    let s = crate::day13::Solve {
      pairs: vec![(
        "[[],[9,[1,1],[0,10],5,7],[6,[[7,8,8],[1,5],[8,9],0,1]],[5]]".into(),
        "[[],[[0,[0,0,5,0],6,0],1,2,[1,[6],7,2,0],[[6,3,7,5],[0,0,7,10]]]]".into(),
      )],
    };
    assert_eq!(s.p1().unwrap().to_string(), "0".to_owned());
  }

  #[test]
  fn idx_1() {
    let s = crate::day13::Solve {
      pairs: vec![("[1,1,3,1,1]".into(), "[1,1,5,1,1]".into())],
    };
    assert_eq!(s.p1().unwrap().to_string(), "1".to_owned());
  }

  #[test]
  fn idx_2() {
    let s = crate::day13::Solve {
      pairs: vec![("[[1],[2,3,4]]".into(), "[[1],4]".into())],
    };
    assert_eq!(s.p1().unwrap().to_string(), "1".to_owned());
  }

  #[test]
  fn idx_3() {
    let s = crate::day13::Solve {
      pairs: vec![("[9]".into(), "[[8,7,6]]".into())],
    };
    assert_eq!(s.p1().unwrap().to_string(), "0".to_owned());
  }

  #[test]
  fn idx_4() {
    let s = crate::day13::Solve {
      pairs: vec![("[[4,4],4,4]".into(), "[[4,4],4,4,4]".into())],
    };
    assert_eq!(s.p1().unwrap().to_string(), "1".to_owned());
  }

  #[test]
  fn idx_5() {
    let s = crate::day13::Solve {
      pairs: vec![("[7,7,7,7]".into(), "[7,7,7]".into())],
    };
    assert_eq!(s.p1().unwrap().to_string(), "0".to_owned());
  }

  #[test]
  fn idx_6() {
    let s = crate::day13::Solve {
      pairs: vec![("[]".into(), "[3]".into())],
    };
    assert_eq!(s.p1().unwrap().to_string(), "1".to_owned());
  }

  #[test]
  fn idx_7() {
    let s = crate::day13::Solve {
      pairs: vec![("[[[]]]".into(), "[[]]".into())],
    };
    assert_eq!(s.p1().unwrap().to_string(), "0".to_owned());
  }

  #[test]
  fn idx_8() {
    let s = crate::day13::Solve {
      pairs: vec![(
        "[1,[2,[3,[4,[5,6,7]]]],8,9]".into(),
        "[1,[2,[3,[4,[5,6,0]]]],8,9]".into(),
      )],
    };
    assert_eq!(s.p1().unwrap().to_string(), "0".to_owned());
  }
}
