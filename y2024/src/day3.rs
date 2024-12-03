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

impl Day for Solve {
    fn p1(&self) -> Result<Box<dyn std::fmt::Display>, Box<dyn Error>> {
        let input = self.input.as_bytes();
        Ok(Box::new(
            try_through_eof(input)
                .iter()
                .map(|mul| mul.0 * mul.1)
                .sum::<i64>(),
        ))
    }

    fn p2(&self) -> Result<Box<dyn std::fmt::Display>, Box<dyn Error>> {
        Ok(Box::new(1))
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
struct Mul(i64, i64);

fn try_parse_mul<'a>(bytes: &'a [u8]) -> Result<(&'a [u8], Mul), PErr<'a>> {
    let rem = drop_through(bytes, "mul(")?;
    let (mnum, rem) = take_until_nan(rem, ',')?;
    let left = parse_i64(mnum).map_err(|_| PErr::NotFound(rem))?;
    let (mnum, rem) = take_until_nan(rem, ')')?;
    let right = parse_i64(mnum).map_err(|_| PErr::NotFound(rem))?;
    Ok((rem, Mul(left, right)))
}

fn try_through_eof<'a>(bytes: &[u8]) -> Vec<Mul> {
    let mut ptr = bytes;
    let mut ret = Vec::new();
    loop {
        if ptr.is_empty() {
            break;
        }
        match try_parse_mul(ptr) {
            Ok((pn, v)) => {
                ret.push(v);
                ptr = pn;
            }
            Err(PErr::NotFound(pn)) => {
                ptr = pn;
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            try_parse_mul("mul(123,4)".as_bytes()).unwrap().1,
            Mul(123, 4)
        );
        assert_eq!(
            try_parse_mul("mul(12,42)".as_bytes()).unwrap().1,
            Mul(12, 42)
        );
        assert_eq!(
            try_parse_mul("mul(1,423)".as_bytes()).unwrap().1,
            Mul(1, 423)
        );
    }

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
        );
        assert_eq!(ret, vec![Mul(2, 4), Mul(5, 5), Mul(11, 8), Mul(8, 5)]);
        assert_eq!(ret.iter().map(|mul| mul.0 * mul.1).sum::<i64>(), 161);
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
}
