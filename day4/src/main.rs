use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

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

  fn is_valid2(&self) -> bool {
    if !self.is_valid() {
      return false;
    }

    macro_rules! validate_format {
      ($($field:ident => $rgx:expr),*) => {
        $(
          {{
            lazy_static! {
              static ref RE: Regex = Regex::new($rgx).unwrap();
            }
            if !RE.is_match(&self.$field.as_ref().unwrap()) {{
              return false;
            }}
          }}
        )*
      }
    };
    validate_format!(
      passport_id => r"^\d{9}$",
      birth_year => r"^\d{4}$",
      issue_year => r"^\d{4}$",
      expiration_year => r"^\d{4}$",
      height => r"^\d+(cm|in)$",
      hair_color => r"^#[0-9a-f]{6}$",
      eye_color => r"^(amb|blu|brn|gry|grn|hzl|oth)$"
    );

    // at this point we know all Options are Somes, so unwrapping to our
    // hearts content is fine!
    macro_rules! validate_limits {
      ($($field:ident => [$low:expr, $high:expr]),*) => {
        $(
          {{
            if let Ok(val) = self.$field.as_ref().unwrap().parse::<i32>() {{
              if val < $low || val > $high {{
                return false;
              }}
            }}
            else {{
              return false;
            }}
          }}
        )*
      }
    };

    validate_limits!(
      birth_year => [1920, 2002],
      issue_year => [2010, 2020],
      expiration_year => [2020, 2030]
    );

    lazy_static! {
      static ref RE: Regex = Regex::new(r"(\d+)(cm|in)").unwrap();
    };

    if let Some(cap) = RE.captures(&self.height.as_ref().unwrap()) {
      if &cap[2] == "cm" {
        if let Ok(num) = cap[1].to_string().parse::<i32>() {
          if num < 150 || num > 193 {
            return false;
          }
        } else {
          return false;
        }
      } else if &cap[2] == "in" {
        if let Ok(num) = cap[1].to_string().parse::<i32>() {
          if num < 59 || num > 76 {
            return false;
          }
        } else {
          return false;
        }
      }
    }

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

  let valid_cnt = input.iter().filter(|p| p.is_valid2()).count();
  println!("Answer (part2): {}", valid_cnt);
  Ok(())
}
