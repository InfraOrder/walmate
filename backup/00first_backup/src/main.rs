use ini;
use dirs;

use clap;
use serde_yaml;
#[macro_use] use serde_derive;

mod util;
mod init;
use crate::init::{
    get_variables,
    get_args,
    get_colors,
    // get_cache_files,
};
use crate::util::{
    get_home_dir,
    get_files,
};

mod gen_files;
use crate::gen_files::{
    gen_files,
};

fn main() {
    // init
    let conf_location = get_home_dir(&"~/.config/walmate/walmate.ini".to_owned());
    let variables = get_variables(&conf_location);
    let files = get_files(&(variables.build_dir.to_owned() + "cache/"), vec!["jpg".to_owned(), "png".to_owned()]);
    let args = get_args();

    // colors and file
    let colors = get_colors(&args, &variables, &files);    
    let files = gen_files(&colors);

    // // writing to file
    for file in files {
        std::fs::write(format!("{}{}", get_home_dir(&variables.build_dir), file.0), file.1).expect("unable to write to file");
    }
}
