use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

const INPUT_FILE: &str = "input.txt";

#[derive(Default, Debug)]
struct Passport {
  birth_year: Option<String>,
  issue_year: Option<String>,
  expiration_year: Option<String>,
  height: Option<String>,
  hair_color: Option<String>,
  eye_color: Option<String>,
  passport_id: Option<String>,
  country_id: Option<String>,
}

impl Passport {
  fn new(data: &Vec<String>) -> Self {
    let mut items = HashMap::new();
    for line in data.iter() {
      for item in line.split(" ") {
        let parts: Vec<&str> = item.split(":").collect();
        items.insert(parts[0], parts[1]);
      }
    }

    let mut inst = Passport::default();

    macro_rules! gen_fields {
      ($($key:expr => $field:ident),*) => {
        $(
          if items.contains_key($key) {
            inst.$field = Some(items[$key].to_string());
          }
        )*
      };
    };

    gen_fields!(
      "byr" => birth_year,
      "iyr" => issue_year,
      "eyr" => expiration_year,
      "hgt" => height,
      "hcl" => hair_color,
      "ecl" => eye_color,
      "pid" => passport_id,
      "cid" => country_id
    );

    inst
  }

  fn is_valid(&self) -> bool {
    macro_rules! validate_fields {
      ($($field:ident),*) => {
        $(
          if self.$field.is_none() {{
            return false;
          }}
        )*
      }
    };
    validate_fields!(
      birth_year,
      issue_year,
      expiration_year,
      height,
      hair_color,
      eye_color,
      passport_id
    );
    true
  }
}

fn get_input() -> Result<Vec<Passport>> {
  let file = File::open(&INPUT_FILE)?;
  let reader = BufReader::new(&file);

  let mut line_buf = Vec::new();
  let mut passports = Vec::new();

  for line in reader.lines() {
    let line = line?;
    if line == "" {
      passports.push(Passport::new(&line_buf));
      line_buf.clear();
    } else {
      line_buf.push(line);
    }
  }
  if !line_buf.is_empty() {
    passports.push(Passport::new(&line_buf));
  }
  Ok(passports)
}

fn main() -> Result<()> {
  let input = get_input()?;
  let valid_cnt = input.iter().filter(|p| p.is_valid()).count();
  println!("Answer (part1): {}", valid_cnt);
  Ok(())
}
