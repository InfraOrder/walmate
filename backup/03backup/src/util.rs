use std::process::Command;
use std::fs::{read_dir};
use dirs::home_dir;
// simple command to clean up code
pub fn command(command: &str) -> std::process::Output {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command given")
}

pub fn get_files(directory: &String, args: Vec<String>) -> Vec<String> {
    let paths = read_dir(format!("{}", get_home_dir(&directory.to_owned()))).unwrap();
    let mut files = Vec::new();
    for entry in paths {
        let dir = entry.unwrap();
        let path = dir.path();
        if !path.is_dir() {
            let path_in_str = path.to_str()
                .expect("Could not convert path to string.");
            for arg in &args {
                if path_in_str.contains(arg) {
                    files.push(dir.file_name().into_string().unwrap());
                    break;
                }
            }
            if args.len() < 1 {
                files.push(dir.file_name().into_string().unwrap());
            }
        }
    }
    files
}

pub fn send_to_file(output_file: &str, hex: Vec<String>) {
    let output_file = get_home_dir(&output_file.to_string()).to_string();
    let mut data = String::new();
    for value in hex {
        // data.push(data + "\n");
        data = data + &value + "\n";
    }
    std::fs::write(output_file, data).expect("Unable to write file");
}

pub fn get_home_dir(path: &String) -> String {
    path.replace("~", &home_dir().unwrap().to_string_lossy())
}
