// use serde::{Deserialize, Serialize};
use serde_derive::{Serialize, Deserialize};
use std::fs;

use crate::util::get_home_dir;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Theme {
  base00: String,
  base01: String,
  base02: String,
  base03: String,
  base04: String,
  base05: String,
  base06: String,
  base07: String,
  base08: String,
  base09: String,
  base0A: String,
  base0B: String,
  base0C: String,
  base0D: String,
  base0E: String,
  base0F: String
}

pub fn parse_scheme(dir: &String, scheme_name: &String) -> Vec<String> {
  let file = fs::read_to_string(get_home_dir(dir) + scheme_name).unwrap();
  let yaml: Theme = serde_yaml::from_str(&file).unwrap();
  let mut as_vec = Vec::new();
  // cant iterate through structs sadly
  // >:(
  as_vec.push(yaml.base00);
  as_vec.push(yaml.base01);
  as_vec.push(yaml.base02);
  as_vec.push(yaml.base03);
  as_vec.push(yaml.base04);
  as_vec.push(yaml.base05);
  as_vec.push(yaml.base06);
  as_vec.push(yaml.base07);
  as_vec.push(yaml.base08);
  as_vec.push(yaml.base09);
  as_vec.push(yaml.base0A);
  as_vec.push(yaml.base0B);
  as_vec.push(yaml.base0C);
  as_vec.push(yaml.base0D);
  as_vec.push(yaml.base0E);
  as_vec.push(yaml.base0F);
  as_vec
}