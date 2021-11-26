extern crate regex;
use regex::Regex;

use rust_util::{read_input, AocDay, Day};

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::ops::RangeInclusive;

const REQUIRED_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const VALID_EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
const VALID_BIRTH_YR: RangeInclusive<i32> = 1920..=2002;
const VALID_ISS_YEAR: RangeInclusive<i32> = 2010..=2020;
const VALID_EXP_YEAR: RangeInclusive<i32> = 2020..=2030;
const VALID_HGT_CM: RangeInclusive<i32> = 150..=193;
const VALID_HGT_IN: RangeInclusive<i32> = 59..=76;
lazy_static! {
  static ref VALID_HAIR_COLOR: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
  static ref VALID_PASS_ID: Regex = Regex::new("^\\d{9}$").unwrap();
}

pub struct Solve {
  passports: Vec<HashMap<String, String>>,
}

impl Day for Solve {
  fn new(d: AocDay) -> Result<Box<dyn Day>, Box<dyn Error>>
  where
    Self: Sized,
  {
    Ok(Box::new(Solve {
      passports: read_input(2020, d)?
        .split("\n\n")
        .map(|passport| {
          passport
            .split_whitespace()
            .map(|token| token.splitn(2, ":"))
            .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
        })
        .collect(),
    }))
  }

  fn p1(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .passports
        .iter()
        .filter(has_required_fields)
        .count()
        .to_string(),
    ))
  }

  fn p2(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
    Ok(Box::new(
      self
        .passports
        .iter()
        .filter(has_required_fields)
        .filter(has_valid_fields)
        .count()
        .to_string(),
    ))
  }
}

fn has_required_fields(pass: &&HashMap<String, String>) -> bool {
  REQUIRED_KEYS
    .iter()
    .filter(|k| !pass.contains_key(**k))
    .next()
    .is_none()
}

fn has_valid_fields(pass: &&HashMap<String, String>) -> bool {
  pass
    .iter()
    .filter(|(k, v)| !match k.as_ref() {
      "byr" => in_range(&v, VALID_BIRTH_YR),
      "iyr" => in_range(&v, VALID_ISS_YEAR),
      "eyr" => in_range(&v, VALID_EXP_YEAR),
      "hcl" => VALID_HAIR_COLOR.is_match(v),
      "ecl" => VALID_EYE_COLORS.contains(&v.as_str()),
      "pid" => VALID_PASS_ID.is_match(v),
      "cid" => true,
      "hgt" => {
        let len = v.len();
        match &v[len - 2..] {
          "cm" => in_range(&v[..len - 2], VALID_HGT_CM),
          "in" => in_range(&v[..len - 2], VALID_HGT_IN),
          _ => false,
        }
      }
      _ => false,
    })
    .next()
    .is_none()
}

fn in_range(v: &str, range: RangeInclusive<i32>) -> bool {
  v.parse::<i32>()
    .ok()
    .filter(|x| range.contains(x))
    .is_some()
}
