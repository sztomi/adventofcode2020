use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{bail, Result};
use lazy_static::lazy_static;
use regex::Regex;

const INPUT_FILE: &str = "input.txt";

struct Entry {
  num1: usize,
  num2: usize,
  rule_char: char,
  password: String,
}

impl Entry {
  fn new(line: &str) -> Result<Self> {
    lazy_static! {
      static ref RGX: Regex = Regex::new(r"(\d+)\-(\d+)\s(.):\s(\w+)").unwrap();
    }
    let cap = match RGX.captures(&line) {
      Some(val) => val,
      None => bail!("Could not parse rule line {}", line),
    };
    Ok(Self {
      num1: cap[1].to_string().parse::<usize>()?,
      num2: cap[2].to_string().parse::<usize>()?,
      rule_char: cap[3].chars().next().unwrap(),
      password: cap[4].to_string(),
    })
  }

  fn is_valid(&self) -> bool {
    let cnt = self.password.chars().fold(
      0,
      |acc, ch| if ch == self.rule_char { acc + 1 } else { acc },
    );
    cnt >= self.num1 && cnt <= self.num2
  }

  fn is_valid2(&self) -> bool {
    let ch1 = self.password.chars().nth(self.num1 - 1);
    let ch2 = self.password.chars().nth(self.num2 - 1);
    match (ch1, ch2) {
      (None, None) => false,
      (None, Some(c)) => c == self.rule_char,
      (Some(c), None) => c == self.rule_char,
      (Some(c1), Some(c2)) => (c1 == self.rule_char) ^ (c2 == self.rule_char),
    }
  }
}

fn get_input() -> Result<Vec<Entry>> {
  let file = File::open(&INPUT_FILE)?;
  let reader = BufReader::new(&file);
  let mut res = Vec::new();
  for line in reader.lines() {
    res.push(Entry::new(&line?)?);
  }
  Ok(res)
}

fn main() -> Result<()> {
  let input = get_input()?;
  let valid_cnt = input.iter().filter(|item| item.is_valid()).count();
  println!("Answer (part1): {}", valid_cnt);

  let valid_cnt = input.iter().filter(|item| item.is_valid2()).count();
  println!("Answer (part2): {}", valid_cnt);

  Ok(())
}
