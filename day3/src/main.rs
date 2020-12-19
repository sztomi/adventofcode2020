use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

const INPUT_FILE: &str = "input.txt";
const TREE: char = '#';


fn get_input() -> Result<Vec<Vec<char>>> {
  let mut lines = Vec::new();

  let file = File::open(&INPUT_FILE)?;
  let reader = BufReader::new(&file);

  for line in reader.lines() {
    lines.push(line?.chars().collect());
  }
  Ok(lines)
}

fn count_trees(data: &Vec<Vec<char>>, right: usize, down: usize) -> usize {
  let mut current_x = 0;
  let mut current_y = 0;
  let mut tree_count = 0;
  let line_len = data[0].len();

  loop {
    if data[current_y][current_x] == TREE {
      tree_count += 1;
    }
    current_y += down;
    current_x = (current_x + right) % line_len;
    if current_y > data.len() - 1 {
      break;
    }
  }

  tree_count
}


fn main() -> Result<()> {
  let input = get_input()?;
  let mut cnt = count_trees(&input, 3, 1);
  println!("Answer (part1): {}", cnt);

  let rules = [
    (1, 1),
    // already have this in cnt: (3, 1),
    (5, 1),
    (7, 1),
    (1, 2)
  ];

  for &rule in rules.iter() {
    cnt *= count_trees(&input, rule.0, rule.1);
  }
  println!("Answer (part2): {}", cnt);

  Ok(())
}