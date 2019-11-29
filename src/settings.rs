use clap::{App, Arg};

use std::fs;
// use serde::{Deserialize, Serialize};
use serde_derive::{Serialize, Deserialize};

use crate::util::{get_files, get_user_input};
use crate::scheme::{parse_scheme};

#[derive(Serialize, Deserialize)]
pub struct JsonSettings {
  build_dir: String,
  base16_dir: String
}

#[derive(Debug)]
pub struct Args {
  verbose: bool,
  base: String,
  base_type: Type
}

#[derive(Debug, Copy, Clone)]
pub enum Type {
  Dark,
  Light
}

#[derive(Debug)]
pub struct Settings {
  pub verbose: bool,
  pub scheme: Vec<String>,
  pub scheme_type: Type,
  pub build_dir: String
}


pub fn get_args() -> Args {
  let matches = App::new("walmate")
    .version("0.2")
    .author("infraorder")
    .about("Generates color palette and sets apps to use that color scheme")
    .arg(Arg::with_name("verbose")
      .short("v").long("verbose")
      .help("verbose log output"))
    .arg(Arg::with_name("base16")
      .short("b").long("base16")
      .value_name("BASE")
      .help("which theme should be used")
      .takes_value(true))
    .arg(Arg::with_name("type")
      .short("t").long("type")
      .value_name("TYPE")
      .help("dark or light"))
    .get_matches();

    let base = matches.value_of("base16")
      .expect("base16 theme name required");
    let base_type = match matches.value_of("type") {
      Some(x) => if x.contains("d") {
        Type::Dark
      } else {
        Type::Light
      },
      None => Type::Dark
    };
    let verbose = matches.is_present("verbose");

    Args {
      verbose: verbose,
      base: base.to_string(),
      base_type: base_type
    }
}

pub fn get_json_settings() -> JsonSettings {
  // get the file from main folder
  let file = fs::read_to_string("settings.json").unwrap();
  let settings: JsonSettings = match serde_json::from_str(&file) {
    Ok(x) => x,
    Err(x) => panic!("error is {}", x)
  };
  settings
}

pub fn complete_settings(args: &Args, json: &JsonSettings) -> Settings {
  // now we need to prompt the user for the selection of the 
  // theme if there is not an exact match

  // get all the filenames in the base16 dir
  let files = get_files(&json.base16_dir, &args.base);
  let file = if files.len() > 1 {
    get_user_input(&files)
  } else {
    files[0].to_string()
  };
  println!("file is {}", file);
  // parse_scheme
  let scheme = parse_scheme(&json.base16_dir, &file);
  // println!("scheme is {:?}", scheme);
  Settings {
    verbose: args.verbose,
    scheme: scheme,
    scheme_type: args.base_type,
    build_dir: json.build_dir.to_string()
  }
}
