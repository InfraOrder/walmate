use ini;
use dirs;

use clap;
use serde_yaml;
#[macro_use] use serde_derive;

mod util;
use crate::util::{
    get_home_dir,
    get_files,
    command,
};
mod init;
use crate::init::{
    get_variables,
    get_args,
    get_colors,
    // get_cache_files,
};

mod post;
use crate::post::change_active_windows;

mod gen_files;
use crate::gen_files::{
    gen_files,
};

fn main() {
    // list of structs

    // init
    let conf_location = get_home_dir(&"~/.config/walmate/walmate.ini".to_owned());
    let variables = get_variables(&conf_location);
    let files = get_files(&(variables.build_dir.to_owned() + "cache/"), vec!["jpg".to_owned(), "png".to_owned()]);
    let args = get_args();

    // colors and file
    let colors = get_colors(&args, &variables, &files);
    let files = gen_files(&colors, &args.display_type);

    // writing to file
    for file in &files {
        std::fs::write(format!("{}{}", get_home_dir(&variables.build_dir), file.0), file.1.to_owned()).expect("unable to write to file");
        // println!("file is {:?}", file);
        // create links and run other external commands
        match file.0.as_ref() {
            "definitions" => {command(&("ln -s ".to_string() + &get_home_dir(&variables.build_dir) + &file.0 + " " + "~/.urxvt/colors/definitions"));},
            "rofi.rasi" => {command(&("ln -s ".to_string() + &get_home_dir(&variables.build_dir) + &file.0 + " " + "~/.config/rofi/rofi.rasi"));},
            "rofi.config" => {command(&("ln -s ".to_string() + &get_home_dir(&variables.build_dir) + &file.0 + " " + "~/.config/rofi/config"));},
            "walmate.vim" => {command(&("ln -s ".to_string() + &get_home_dir(&variables.build_dir) + &file.0 + " " + "~/.vim/colors/"));},
            "walmate_lightline.vim" => {command(&("ln -s ".to_string() + &get_home_dir(&variables.build_dir) + &file.0 + " " + "~/.vim/autoload/lightline/colorscheme"));},
            // commented out for now
            // "oomox" => {command("oomox-cli -o walmate -m all ~/.cache/walmate/oomox");},
            "walmate.tmTheme" => {command(&("ln -s ".to_string() + &get_home_dir(&variables.build_dir) + &file.0 + " " + "~/dot/cnf/sublime-text-3/Packages/User/theme - walmate/"));},
            "walmate_theme" => {command(&("ln -s ".to_string() + &get_home_dir(&variables.build_dir) + &file.0 + " " + "~/dot/cnf/sublime-text-3/Packages/User"));},
            "walmate-theme.el" => {command(&("ln -s ".to_string() + &get_home_dir(&variables.build_dir) + &file.0 + " " + "~/.emacs.d/themes/"));},
            "dunstrc" => {command(&("ln -s ".to_string() + &get_home_dir(&variables.build_dir) + &file.0 + " " + "~/.config/dunst/"));},
            "walmate_qutebrowser.py" => {command(&("ln -s ".to_string() + &get_home_dir(&variables.build_dir) + &file.0 + " " + "~/.config/qutebrowser/config.py"));},
            _ => {},
        }
    }

    change_active_windows(args.display_type);
}
