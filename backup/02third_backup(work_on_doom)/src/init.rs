use ini::Ini;
use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;
use std::io;
use crate::util::{command, get_files, send_to_file, get_home_dir};
use dirs::home_dir;
use std::collections::BTreeMap;

pub fn get_args() -> Args { 
    let matches = App::new("walmate")
        .version("0.1")
        .author("InfraOrder")
        .about("Generates color palette and sets apps to use that color scheme")
        .arg(Arg::with_name("base16")
            .short("b")
            .long("base16")
            .value_name("FILE")
            .help("use base16 scheme or not")
            .takes_value(true))
        .arg(Arg::with_name("input")
            .help("takes input image")
            .index(1))
        .get_matches();
    // if let Some(i) = matches.value_of("input") {
        // println!("Value for output: {}", i);
    // }

    // if let Some(b) = matches.value_of("base16") {
        // println!("Value for config: {}", b);
    // }
    let mut input_var = false;
    let input = match matches.value_of("input") {
        Some(x) => {
            input_var = true;
            Some(x.to_owned())
        },
        None => None,
    };
    let mut base16_var = false;
    let base16 = match matches.value_of("base16") {
        Some(x) => {
            base16_var = true;
            Some(x.to_owned())
        },
        None => None,
    };
    if !input_var && !base16_var {
        panic!("an input or a base16 color required.");
    }
    Args {
        input: input,
        base16: base16,
        // input: Some(String::new()),
        // base16: Some(String::new()),
    }
}

pub fn get_variables(conf_location: &String) -> Variables {
    let mut build_dir = "~/.cache/walmate/".to_string();
    let mut wallpaper_dir = "~/Wallapers/dark/".to_string();
    let mut base16_dir = "~/.config/walmate/schemes/".to_string();
    let mut saturation = 0.3;
    // simple ini config to keep user settings.
    let conf = match Ini::load_from_file(&conf_location) {
        Ok(x) => x,
        Err(_) => {
            // here we create an ini file if one doesn't exist
            let mut conf = Ini::new();
            conf.with_section(Some("Directories".to_owned()))
                .set("wallpaper_dir", wallpaper_dir.to_owned())
                .set("build_dir", build_dir.to_owned())
                .set("base16_dir", base16_dir.to_owned());
            conf.with_section(Some("GeneralSettings".to_owned()))
                .set("saturation", saturation.to_string());
            conf.write_to_file(conf_location).expect("Unable to save config to file.");
            conf
        },
    };

    let directory_section = conf.section(Some("Directories".to_owned()));
    match directory_section {
        Some(x) => {
            build_dir = match x.get("build_dir") {
                Some(y) => y.to_owned(),
                _ => build_dir,
            };
            wallpaper_dir = match x.get("wallpaper_dir") {
                Some(y) => y.to_owned(),
                _ => wallpaper_dir,
            };
            base16_dir = match x.get("base16_dir") {
                Some(y) => y.to_owned(),
                _ => base16_dir,
            };
        },
        _ => {},
    }
    let general_section = conf.section(Some("GeneralSettings".to_owned()));
    match general_section {
        Some(x) => {
            saturation = match x.get("saturation") {
                Some(y) => y.to_owned().parse::<f32>().expect("Error parsing through saturation"),
                _ => saturation,
            }
        },
        _ => {},
    }

    Variables {
        build_dir: build_dir,
        wallpaper_dir: wallpaper_dir,
        base16_dir: base16_dir,
        saturation: saturation,
    }
}

pub struct Variables {
    pub build_dir: String,
    pub wallpaper_dir: String,
    pub base16_dir: String,
    pub saturation: f32,
}

pub struct Args {
    pub input: Option<String>,
    pub base16: Option<String>,
}

pub fn get_colors(args: &Args, variables: &Variables, files: &Vec<String>) -> Vec<String> {
    // generate a filename
    let home_dir: String = home_dir().unwrap().to_string_lossy().to_string();

    match &args.input {
        Some(x) => {
            let filename = (x.replace(&home_dir, "")).replace("/", "+");
                
            let mut found: (bool, String) = (false, String::new());
            for file in files {
                let file = file.to_owned();
                // println!("file is {}, filename is {}", file, filename);
                found = if file == filename {
                    println!("palette found!");
                    (true, file)
                } else {
                    found
                };
            }
        
            match found {
                (true, x) => {
                    // found match not generating palette
                    command(&format!("cp {}/cache/{} {}colors", variables.build_dir, x, variables.build_dir));
                },
                _ => {
                    println!("generating");
                    // no match creating palette and cache
                    command(&format!("gen_palette {} {}colors -s {}", get_home_dir(&x), variables.build_dir, variables.saturation));
                    // copy colors to filename as a cache for colors
                    command(&format!("cp {}colors {}/cache/{}", variables.build_dir, variables.build_dir, filename));
                }
            }  
        },
        _ => {
            // assumes base16 is true if we get to this section
            let base16 = args.base16.to_owned().unwrap();

            // let filename = (base16.replace(&home_dir, "")).replace("/", "+");
            let mut filename: String;
                
            let mut found: Vec<String> = vec![];
            let mut selected = String::new();
 
            let files = get_files(&variables.base16_dir, Vec::new());
            // println!("files are {:?}", files);

            for file in &files {
                let file = file.to_owned();
                // println!("file is {}, filename is {}", file, filename);
                if file.contains(&base16) {
                    // println!("found this {}", file);
                    found.push(file)
                } else {};
            }
            if found.len() < 1 {
                panic!("Unable to find base16 color scheme with that name");
            } else if found.len() > 1 {
                selected = get_user_input(&found);
            } else {
                selected = found[0].to_owned();
            }
            
            println!("base is {}", selected);
            let yaml_dir = get_home_dir(&format!("{}{}", variables.base16_dir, selected));
            // println!("yaml_dir is {}", yaml_dir);
            let f = File::open(yaml_dir).expect("Unable to open yaml file");
            let mut colors_yaml: BTreeMap<String, String> = serde_yaml::from_reader(f).expect("Not a valid yaml file, or other Error");
            colors_yaml.remove("author");
            colors_yaml.remove("scheme");
            // println!("Read YAML string: {:?}", colors_yaml);
            let mut colors: Vec<String> = vec![];
            for (name, color) in &colors_yaml {
                colors.push("#".to_owned() + &color.to_owned());
            }
            // println!("new colors are {:?}", colors);
            send_to_file(&format!("{}colors", variables.build_dir), colors);
            // command(&format!("cp {}{} {}/colors", variables.base16_dir, selected, variables.build_dir));
        },
    }


    let mut colors_file = File::open(format!("{}colors", get_home_dir(&variables.build_dir.to_owned())))
        .expect("unable to open generated file");
    let mut colors = String::new();
    colors_file.read_to_string(&mut colors)
        .expect("unable to read from generated file");
    let mut colors_separated: Vec<String> = vec![];
    for line in colors.lines() { colors_separated.push(line.to_owned()); } 
    // let mut colors = colors.lines().iter().collect(); // ????????
    // println!("colors are {:?}", colors_separated);
    colors_separated
}

fn get_user_input(files: &Vec<String>) -> String {
    let ammount = files.len();
    for i in 1..ammount + 1 {
        println!("{}: {}", i, files[i - 1]);
    }
    println!("please select one of the above by inputting the number.");

    let mut num: u32 = 1000;
    let mut input = String::new();
    while num == 1000 {
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                match input.trim().parse::<u32>() {
                    Ok(e) => {
                        if e <= 0 || e as usize > ammount {
                            println!("out of range");
                        } else {
                            num = e;
                        }
                        
                    }
                    Err(e) => println!("non-number given, retry"),
                }
                // println!("{}", input);
            }
            Err(error) => println!("error: {}", error),
        }
    }
    files[(num - 1)as usize].to_owned()
}
