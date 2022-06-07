use dirs::home_dir;
use std::process::Command;
use std::fs::{read_dir};

use std::io;


// simple command to clean up code
pub fn command(command: &str) -> std::process::Output {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command given")
}

pub fn get_home_dir(path: &String) -> String {
    path.replace("~", &home_dir().unwrap().to_string_lossy())
}

pub fn get_user_input(selection: &Vec<String>) -> String {
    let ammount = selection.len();
    for i in 1..ammount + 1 {
        println!("{}: {}", i, selection[i - 1]);
    }
    println!("please select one of the above by inputting the number.");

    let mut num: u32 = 1000;
    let mut input = String::new();
    while num == 1000 {
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                match input.trim().parse::<u32>() {
                    Ok(e) => {
                        if e <= 0 || e as usize > ammount {
                            println!("out of range");
                        } else {
                            num = e;
                        }
                        
                    }
                    Err(_e) => println!("non-number given, retry"),
                }
                // println!("{}", input);
            }
            Err(error) => println!("error: {}", error),
        }
    }
    selection[(num - 1)as usize].to_owned()
}

pub fn get_files(directory: &String, name: &String) -> Vec<String> {
  println!("dir is {}", directory);
  let paths = read_dir(format!("{}", get_home_dir(&directory.to_owned()))).unwrap();
  let mut files = Vec::new();
  for entry in paths {
      let dir = entry.unwrap();
      let path = dir.path();
      if !path.is_dir() {
          // let path_in_str = path.to_str()
          //     .expect("Could not convert path to string.");
          if dir.file_name().into_string().unwrap().contains(name) {
            files.push(dir.file_name().into_string().unwrap())
          }
      }
  }
  if files.len() < 1 {
    panic!("No scheme with the name: {} found in directory: {}", name, directory);
  }
  files
}

pub fn hex_to_rgb(hex: &String) -> (String, String, String) {
    let hex = color_convert::color::Color::new(hex);
    let rgb = hex.to_rgb().unwrap();
    let values: Vec<&str> = rgb.split(',').collect();
    let r = values[0][4..].to_string();
    let g = values[1].to_string();
    let i = values[2].len() - 1;
    let b = values[2][..i].to_string();
    (r, g, b)
}

pub fn link(build_dir: &str, file_name: &str, destination: &str) {
    println!("linking {} to {}", file_name, destination);
    // println!("command is: {}", &("ln -s ".to_string() + &get_home_dir(&build_dir.to_string()) + "'" + &file_name + "' " + destination));
    let complete_out = command(&("ln -s ".to_string() + &get_home_dir(&build_dir.to_string()) + "'" + &file_name + "' " + destination)).stderr.to_owned();
    let out = String::from_utf8_lossy(&complete_out);
    if out.contains("failed to ") {
        if out.contains("File exists") {
            println!("file exists");
            command(&("rm ".to_owned() + destination));
            println!("{:?}", String::from_utf8_lossy(&command(&("ln -s ".to_string() + &get_home_dir(&build_dir.to_string()) + "'" + &file_name + "' " + destination)).stderr));
            let complete_out = command(&("ln -s ".to_string() + &get_home_dir(&build_dir.to_string()) + "'" + &file_name + "' " + destination)).stderr.to_owned();
            let out = String::from_utf8_lossy(&complete_out);
        }
    };
}
