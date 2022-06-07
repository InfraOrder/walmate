use std::fs::{File, read_dir, read};

pub fn gen_files(colors: &Vec<String>, display_time: &String) -> Vec<(String, String)> {
    let paths = read_dir("templates/").unwrap();
    let mut files: Vec<(String, String)> = Vec::new(); // first value is name second is data
    for entry in paths {
        let dir = entry.unwrap();
        let path = dir.path();
        if !path.is_dir() {
            let dir_name = dir.file_name().into_string().unwrap();

            // check if the file is the wrong type
            match display_time.as_ref()
            {
                "dark" => {
                    match dir_name.as_ref() {
                        "walmate-theme-light.el" => (),
                        "walmate-theme-dark.el" => {
                            let data = String::from_utf8_lossy(&read(dir.path()).unwrap()).to_string();
                            files.push(("walmate-theme.el".to_string(), data))
                        },
                        _ => {
                            let data = String::from_utf8_lossy(&read(dir.path()).unwrap()).to_string();
                            files.push((dir_name, data))
                        }
                    }
                },
                "light" => {
                    match dir_name.as_ref() {
                        "walmate-theme-dark.el" => (),
                        "walmate-theme-light.el" => {
                            let data = String::from_utf8_lossy(&read(dir.path()).unwrap()).to_string();
                            files.push(("walmate-theme.el".to_string(), data))
                        },
                        _ => {
                            let data = String::from_utf8_lossy(&read(dir.path()).unwrap()).to_string();
                            files.push((dir_name, data))
                        }
                    }
                },
                _ => panic!("how did you get here lol")
            }
        }
    }
    for file in &mut files {
        for (i, color) in colors.iter().enumerate() {
            let in_string = match i {
                0  => "$base00",
                1  => "$base01",
                2  => "$base02",
                3  => "$base03",
                4  => "$base04",
                5  => "$base05",
                6  => "$base06",
                7  => "$base07",
                8  => "$base08",
                9  => "$base09",
                10 => "$base0A",
                11 => "$base0B",
                12 => "$base0C",
                13 => "$base0D",
                14 => "$base0E",
                15 => "$base0F",
                _  => "",
            };
            let mut color = color.clone();
            color.remove(0);
            file.1 = (file.1).replace(in_string, &color);
        }
    }
    files
}
