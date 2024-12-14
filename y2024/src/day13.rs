use rust_util::Day;
use std::{error::Error, fmt::Display};

struct System {
    x: [i64; 3],
    y: [i64; 3],
}
impl From<&str> for System {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let (al, ar) = lines.next().and_then(|l| l.split_once(", ")).unwrap();
        let (bl, br) = lines.next().and_then(|l| l.split_once(", ")).unwrap();
        let (pl, pr) = lines.next().and_then(|l| l.split_once(", ")).unwrap();
        let x1 = al
            .strip_prefix("Button A: X+")
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap();
        let y1 = ar
            .strip_prefix("Y+")
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap();
        let x2 = bl
            .strip_prefix("Button B: X+")
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap();
        let y2 = br
            .strip_prefix("Y+")
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap();
        let x = pl
            .strip_prefix("Prize: X=")
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap();
        let y = pr
            .strip_prefix("Y=")
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap();
        System {
            x: [x1, x2, x],
            y: [y1, y2, y],
        }
    }
}
pub struct Solve {
    systems: Vec<System>,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Solve {
            systems: value.split("\n\n").map(System::from).collect(),
        })
    }
}
impl System {
    fn solve(&self, add_on: i64) -> Option<i64> {
        let [x1, x2, x] = self.x;
        let [y1, y2, y] = self.y;
        let x = add_on + x;
        let y = add_on + y;

        let d = det(x1, x2, y1, y2);
        if d == 0 {
            return None;
        }
        let d1 = det(x, y, x2, y2);
        if d1 % d != 0 {
            return None;
        }
        let d2 = det(x1, y1, x, y);
        if d2 % d != 0 {
            return None;
        }
        let px = d1 / d;
        let py = d2 / d;
        Some(3 * px + py)
    }
}

fn det(x1: i64, x2: i64, y1: i64, y2: i64) -> i64 {
    x1 * y2 - x2 * y1
}

impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(
            self.systems.iter().filter_map(|s| s.solve(0)).sum::<i64>(),
        ))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(
            self.systems
                .iter()
                .filter_map(|s| s.solve(10000000000000))
                .sum::<i64>(),
        ))
    }
}
