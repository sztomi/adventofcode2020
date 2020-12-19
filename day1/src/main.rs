use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

const INPUT_FILE: &str = "input.txt";

fn get_input() -> Result<HashSet<i32>> {
  let file = File::open(&INPUT_FILE)?;
  let reader = BufReader::new(&file);
  let mut res = HashSet::new();
  for line in reader.lines() {
    res.insert(line?.parse::<i32>()?);
  }

  Ok(res)
}

fn main() -> Result<()> {
  let input = get_input()?;
  let mut found = false;

  // O(n log n) (O(n) amortized)
  for num in input.iter() {
    let diff = 2020 - num;
    // O(log n) (O(1) amortized)
    if input.contains(&diff) {
      println!("Answer (part1): {}", num * diff);
      found = true;
      break;
    }
  }

  if !found {
    println!(":(");
  }

  found = false;
  for num in input.iter() {
    let diff = 2020 - num;
    // now we need to find two numbers that sum to diff
    for num2 in input.iter() {
      let diff2 = diff - num2;
      if input.contains(&diff2) {
        println!("Answer (part2): {}", num2 * diff2 * num);
        found = true;
        break;
      }
    }
    if found {
      break;
    }
  }

  if !found {
    println!(":(");
  }

  Ok(())
}
