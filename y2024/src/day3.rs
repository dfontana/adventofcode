use core::str;
use rust_util::Day;
use std::error::Error;

pub struct Solve {
    input: String,
}

impl TryFrom<String> for Solve {
    type Error = Box<dyn Error>;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        Ok(Solve { input })
    }
}

const OPS_P1: [&'static str; 1] = ["mul("];
const OPS_P2: [&'static str; 3] = ["mul(", "don't(", "do("];

impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn std::fmt::Display>, Box<dyn Error>> {
        let input = self.input.as_bytes();
        Ok(Box::new(
            try_through_eof(input, OPS_P1.to_vec())
                .iter()
                .filter_map(|op| match op {
                    Operation::Mul(a, b) => Some(a * b),
                    _ => None,
                })
                .sum::<i64>(),
        ))
    }

    fn p2(&self) -> Result<Box<dyn std::fmt::Display>, Box<dyn Error>> {
        let input = self.input.as_bytes();
        let mut total = 0;
        let mut ddo = true;
        for op in try_through_eof(input, OPS_P2.to_vec()).iter() {
            match op {
                Operation::Mul(a, b) if ddo => {
                    total += a * b;
                }
                Operation::Do => ddo = true,
                Operation::Dont => ddo = false,
                _ => {}
            }
        }
        Ok(Box::new(total))
    }
}

fn drop_through<'a>(bytes: &'a [u8], sub: &str) -> Result<&'a [u8], PErr<'a>> {
    let mut bidx = 0;
    let buf = sub.as_bytes();
    let buf_len = buf.len() - 1;
    for (idx, ch) in bytes.iter().enumerate() {
        if *ch == buf[bidx] {
            if bidx == buf_len {
                return Ok(&bytes[idx + 1..]);
            }
            bidx += 1;
        } else if *ch == buf[0] {
            bidx = 0;
            // Didn't match reset the search and verify it's not the start
            if bidx == buf_len {
                return Ok(&bytes[idx + 1..]);
            }
            bidx += 1;
        } else {
            bidx = 0;
        }
    }
    Err(PErr::NotFound(&bytes[bytes.len()..]))
}

enum Either<T, V> {
    A(T),
    B(V),
}

fn take_until_nan<'a>(bytes: &'a [u8], bound: char) -> Result<(&'a [u8], &'a [u8]), PErr<'a>> {
    let mut break_or_fail = Either::B(0);
    for (idx, ch) in bytes.iter().enumerate() {
        if ch.is_ascii_digit() {
            continue;
        } else if idx == 0 {
            // Need at least 1 number
            break_or_fail = Either::B(idx);
            break;
        } else if *ch == bound as u8 {
            // We have at least 1 number and found the bound
            break_or_fail = Either::A(idx);
            break;
        } else {
            // Didn't find the bound and it's not a number so bail
            break_or_fail = Either::B(idx);
            break;
        }
    }
    match break_or_fail {
        Either::A(idx) => {
            // Include the last digit of the number, exclude the bound
            Ok((&bytes[..idx], &bytes[idx + 1..]))
        }
        Either::B(idx) => Err(PErr::NotFound(&bytes[idx..])),
    }
}

fn parse_i64<'a>(bytes: &'a [u8]) -> Result<i64, PErr<'a>> {
    str::from_utf8(bytes)
        .map_err(|e| format!("{:?}", e))
        .and_then(|st| i64::from_str_radix(st, 10).map_err(|e| format!("{:?}", e)))
        .map_err(|_e| PErr::Noti64)
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Mul(i64, i64),
    Do,
    Dont,
}
#[derive(Debug, PartialEq, Eq)]
enum OpType {
    Mul,
    Do,
    Dont,
}
impl From<&str> for OpType {
    fn from(value: &str) -> Self {
        match value {
            "mul(" => OpType::Mul,
            "do(" => OpType::Do,
            "don't(" => OpType::Dont,
            _ => unreachable!(),
        }
    }
}

// Returns everything after the parsed op or (if an error), where it stopped
fn try_parse_mul<'a>(bytes: &'a [u8]) -> Result<(&'a [u8], Operation), PErr<'a>> {
    let (mnum, rem) = take_until_nan(bytes, ',')?;
    let left = parse_i64(mnum).map_err(|_| PErr::NotFound(rem))?;
    let (mnum, rem) = take_until_nan(rem, ')')?;
    let right = parse_i64(mnum).map_err(|_| PErr::NotFound(rem))?;
    Ok((rem, Operation::Mul(left, right)))
}

fn match_next_into<'a>(
    bytes: &'a [u8],
    c: char,
    op: Operation,
) -> Result<(&'a [u8], Operation), PErr<'a>> {
    bytes
        .iter()
        .next()
        .filter(|b| **b == c as u8)
        .map(|_| (&bytes[1..], op))
        .ok_or(PErr::NotFound(bytes))
}

fn find_next_op<'a>(
    bytes: &'a [u8],
    ops: std::slice::Iter<'a, &str>,
) -> Result<(&'a [u8], OpType), PErr<'a>> {
    ops.map(|pfx| (drop_through(bytes, pfx), OpType::from(*pfx)))
        .filter_map(|(mv, op)| match mv {
            Ok(pn) => Some((pn, op)),
            _ => None,
        })
        // Find earliest prefix which means most remainder
        .max_by_key(|(pn, _)| pn.len())
        .ok_or(PErr::NoMoreOps)
}

fn try_through_eof<'a>(bytes: &[u8], ops: Vec<&'a str>) -> Vec<Operation> {
    let mut ptr = bytes;
    let mut ret = Vec::new();
    loop {
        if ptr.is_empty() {
            break;
        }

        let (op_end, op_type) = match find_next_op(ptr, ops.iter()) {
            Ok(v) => v,
            // No more ops means no more work to parse
            _ => break,
        };

        let maybe_op = match op_type {
            OpType::Mul => try_parse_mul(op_end),
            OpType::Do => match_next_into(op_end, ')', Operation::Do),
            OpType::Dont => match_next_into(op_end, ')', Operation::Dont),
        };

        match maybe_op {
            Ok((pn, v)) => {
                ret.push(v);
                ptr = pn;
            }
            Err(PErr::NotFound(i_op_end)) => {
                ptr = i_op_end;
            }
            _ => {}
        }
    }
    ret
}

#[derive(Debug, Eq, PartialEq)]
enum PErr<'a> {
    NotFound(&'a [u8]),
    Noti64,
    NoMoreOps,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case2() {
        assert!(try_parse_mul("mul(4*".as_bytes()).is_err());
        assert!(try_parse_mul("mul(6,9!".as_bytes()).is_err());
        assert!(try_parse_mul("?(12,34)".as_bytes()).is_err());
        assert!(try_parse_mul("mul ( 2 , 4 )".as_bytes()).is_err());
    }

    #[test]
    fn case3() {
        let ret = try_through_eof(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".as_bytes(),
            OPS_P1.to_vec(),
        );
        assert_eq!(
            ret,
            vec![
                Operation::Mul(2, 4),
                Operation::Mul(5, 5),
                Operation::Mul(11, 8),
                Operation::Mul(8, 5)
            ]
        );
    }

    #[test]
    fn case4() {
        assert_eq!(drop_through("mul(".as_bytes(), "mul("), Ok("".as_bytes()));
        assert_eq!(
            drop_through("mmul(21".as_bytes(), "mul("),
            Ok("21".as_bytes())
        );
        assert_eq!(
            drop_through("miul(21".as_bytes(), "mul("),
            Err(PErr::NotFound("".as_bytes()))
        );
        assert_eq!(
            drop_through("mmul".as_bytes(), "mud"),
            Err(PErr::NotFound("".as_bytes()))
        );
    }

    #[test]
    fn case5() {
        let ret = try_through_eof(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".as_bytes(),
            OPS_P1.to_vec(),
        );
        assert_eq!(
            ret,
            vec![
                Operation::Mul(2, 4),
                Operation::Mul(5, 5),
                Operation::Mul(11, 8),
                Operation::Mul(8, 5)
            ]
        );
    }
}
