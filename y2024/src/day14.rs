use crossterm::{cursor, terminal, ExecutableCommand};
use itertools::Itertools;
use rust_util::Day;
use std::{
    error::Error,
    fmt::Display,
    io::{stdout, Stdout, Write},
};

#[derive(Debug, Clone)]
struct Bot {
    p: (i64, i64),
    v: (i64, i64),
}
pub struct Solve {
    bots: Vec<Bot>,
}
impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Solve {
            bots: value
                .lines()
                .filter_map(|l| {
                    let (p, v) = l.split_once(" v=")?;
                    let (x, y) = p.strip_prefix("p=")?.split_once(",")?;
                    let (dx, dy) = v.split_once(",")?;

                    Some(Bot {
                        p: (x.parse().ok()?, y.parse().ok()?),
                        v: (dx.parse().ok()?, dy.parse().ok()?),
                    })
                })
                .collect(),
        })
    }
}

fn wrap(v: i64, m: i64) -> i64 {
    if v < 0 {
        m + v
    } else {
        v
    }
}

impl Bot {
    pub fn sim(&self, secs: i64, xm: i64, ym: i64) -> Bot {
        Bot {
            p: (
                wrap((self.p.0 + self.v.0 * secs) % xm, xm),
                wrap((self.p.1 + self.v.1 * secs) % ym, ym),
            ),
            v: self.v,
        }
    }

    pub fn quad(&self, xh: i64, yh: i64) -> Option<u8> {
        if self.p.0 < xh && self.p.1 < yh {
            Some(1)
        } else if self.p.0 < xh && self.p.1 > yh {
            Some(2)
        } else if self.p.0 > xh && self.p.1 < yh {
            Some(3)
        } else if self.p.0 > xh && self.p.1 > yh {
            Some(4)
        } else {
            None
        }
    }
}

impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(
            self.bots
                .iter()
                .map(|b| b.sim(100, 101, 103))
                .filter_map(|b| b.quad(50, 51))
                .into_grouping_map_by(|q| *q)
                .fold(0, |acc, _, _| acc + 1)
                .values()
                .fold(1, |acc, v| acc * v),
        ))
    }

    fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let mut printer = StdoutPrinter {
            height: 103,
            width: 101,
            stdout: &mut stdout(),
            enabled: true,
        };
        let sleep = 1; // Edit this to find value
        let brkpt = 103 * 101;
        let mut cnt = 43;
        let step = 103;
        let mut bots = self.bots.iter().map(|b| b.sim(cnt, 101, 103)).collect();
        while cnt < brkpt {
            printer.print(&bots, cnt, sleep);
            bots = bots.iter().map(|b| b.sim(step, 101, 103)).collect();
            cnt += step;
        }
        Ok(Box::new(1))
    }
}

struct StdoutPrinter<'a> {
    height: i64,
    width: i64,
    stdout: &'a mut Stdout,
    enabled: bool,
}

impl StdoutPrinter<'_> {
    fn print(&mut self, bots: &Vec<Bot>, loopn: i64, sleep: u64) {
        if !self.enabled {
            return;
        }
        self.stdout
            .execute(cursor::MoveUp((self.height + 1) as u16))
            .unwrap();
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::FromCursorDown))
            .unwrap();

        for y in 0..self.height {
            for x in 0..self.width {
                let mut written = false;
                for b in bots.iter() {
                    let kx = b.p.0;
                    let ky = b.p.1;
                    if x == kx && y == ky {
                        write!(self.stdout, "{}", "*").unwrap();
                        written = true;
                        break;
                    }
                }
                if !written {
                    write!(self.stdout, ".").unwrap();
                }
            }
            writeln!(self.stdout).unwrap();
        }
        writeln!(self.stdout, "Loop Num: {}", loopn).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(sleep));
    }
}
