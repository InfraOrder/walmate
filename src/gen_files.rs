use std::fs::{read_dir, read};
use crate::settings::{Settings, Type};
use crate::util::hex_to_rgb;

pub struct TemplateFile {
  pub name: String,
  pub data: String
}

// use futures::executor::block_on;
pub fn gen_files(s: &Settings) -> Vec<TemplateFile> {
    let paths = read_dir("templates/").unwrap();
    // first value is name, second is data
    let mut files: Vec<TemplateFile> = Vec::new();
    for entry in paths {
        let dir = entry.unwrap();
        let path = dir.path();
        if !path.is_dir() {
            let dir_name = dir.file_name().into_string().unwrap();
            match dir_name.as_ref() {
                "walmate-theme-light.el" => {
                    match s.scheme_type {
                        Type::Dark => (),
                        Type::Light => {
                            let data = String::from_utf8_lossy(&read(dir.path()).unwrap()).to_string();
                            files.push(
                                TemplateFile {
                                  name: "walmate-theme.el".to_string(), 
                                  data: data 
                                })
                        }
                    }
                },
                "walmate-theme-dark.el" => {
                    match s.scheme_type {
                        Type::Dark => {
                            let data = String::from_utf8_lossy(&read(dir.path()).unwrap()).to_string();
                            files.push(
                                TemplateFile {
                                  name: "walmate-theme.el".to_string(), 
                                  data: data 
                                })
                        },
                        Type::Light => ()
                    }
                },
                _ => {
                    let data = String::from_utf8_lossy(&read(dir.path()).unwrap()).to_string();
                    files.push(TemplateFile {
                      name: dir_name, data: data
                    })
                }
            }
        }
    }
    for file in &mut files {
        gen_file(s, file);
    }
    files
}

fn gen_file(s: &Settings, file: &mut TemplateFile) {
    match file.name.as_ref() {
        "walmate.sublime-theme" => {
            for (i, color) in s.scheme.iter().enumerate() {
                let code = format!("0{:x}", i).to_uppercase();
                let rgb = hex_to_rgb(color);
                let color = "[".to_string() + &rgb.0 + ", " + &rgb.1 + ", " + &rgb.2 + "]";
                file.data = (file.data).replace(&format!("$base{}", code), &color);
            }
        },
        _ => {   
            for (i, color) in s.scheme.iter().enumerate() {
                let code = format!("0{:x}", i).to_uppercase();
                file.data = (file.data).replace(&format!("$base{}", code), &color);
            }
        }
    }
}
