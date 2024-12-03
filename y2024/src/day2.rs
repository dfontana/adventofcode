use itertools::Itertools;
use rust_util::Day;
use std::error::Error;

pub struct Solve {
    reports: Vec<Vec<i64>>,
}

impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Solve {
            reports: value
                .lines()
                .map(|l| {
                    l.split_whitespace()
                        .filter_map(|v| v.parse::<i64>().ok())
                        .collect_vec()
                })
                .collect_vec(),
        })
    }
}

impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn std::fmt::Display>, Box<dyn Error>> {
        Ok(Box::new(
            self.reports
                .iter()
                .filter(|report| is_safe_itn(true, &report))
                .count(),
        ))
    }

    fn p2(&self) -> Result<Box<dyn std::fmt::Display>, Box<dyn Error>> {
        Ok(Box::new(
            self.reports
                .iter()
                .filter(|report| is_safe_itn(false, &report))
                .count(),
        ))
    }
}

// TODO - There's definitely more idiomatic code to write, if not a better approach overall
//      ...but this works for now
fn is_safe_itn(skip_init: bool, report: &[i64]) -> bool {
    let mut dir = 0;
    let mut used_skip = skip_init;
    let mut idx = 0;
    loop {
        if idx + 1 == report.len() {
            break;
        }
        let a = report[idx];
        let b = report[idx + 1];
        let mc = report.get(idx + 2);
        let md = report.get(idx + 3);
        let didx = idx + 3;
        idx += 1;

        if mc.is_some() {
            let c = mc.unwrap();

            // is ABC?
            let (mdir, safe) = is_triplet_safe(dir, &a, &b, c);
            if safe {
                dir = mdir;
                continue;
            }

            // Must check for skips
            if used_skip {
                return false;
            }
            used_skip = true;

            // Handle case where C is at end and should be skipped
            if md.is_none() && is_safe(mdir, a - b) {
                return true;
            }

            // Should we skip to D?
            if md.is_none() {
                return false;
            }
            let d = md.unwrap();
            idx = didx;

            // is ABD?
            let (mdir, safe) = is_triplet_safe(dir, &a, &b, d);
            if safe {
                dir = mdir;
                continue;
            }
            // is ACD?
            let (mdir, safe) = is_triplet_safe(dir, &a, &c, d);
            if safe {
                dir = mdir;
                continue;
            }
            // is BCD?
            let (mdir, safe) = is_triplet_safe(dir, &b, &c, d);
            if safe {
                dir = mdir;
                continue;
            }
            return false;
        // is AB?
        } else if is_safe(dir, a - b) {
            continue;
        }
        return false;
    }
    true
}

fn is_triplet_safe(dir: i64, a: &i64, b: &i64, c: &i64) -> (i64, bool) {
    let mut mdir = dir;
    if mdir == 0 {
        mdir = a - b;
    }
    if is_safe(mdir, a - b) && is_safe(mdir, b - c) {
        return (mdir, true);
    }
    (dir, false)
}

fn is_safe(dir: i64, diff: i64) -> bool {
    if !(1 <= diff && diff <= 3) && !(-3 <= diff && diff <= -1) {
        return false;
    }
    dir == 0 || dir.clamp(-1, 1) == diff.clamp(-1, 1)
}
