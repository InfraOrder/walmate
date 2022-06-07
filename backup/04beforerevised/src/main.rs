// use ini;
// use dirs;

// use clap;
// use serde_yaml;
// #[macro_use] use serde_derive;

mod util;
use crate::util::{
    get_home_dir,
    get_files,
    command,
    link,
};
mod init;
use crate::init::{
    get_variables,
    get_args,
    get_colors,
    // get_cache_files,
};

mod post;
// use crate::post::change_active_windows;
use crate::post::post;

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
            "definitions" => {link(&variables.build_dir, &file.0, "~/.urxvt/colors/");},
            "rofi.rasi" => {link(&variables.build_dir, &file.0, "~/.config/rofi/");},
            "rofi.config" => {link(&variables.build_dir, &file.0, "~/.config/rofi/");},
            "walmate.vim" => {link(&variables.build_dir, &file.0, "~/.vim/colors/");},
            "walmate_lightline.vim" => {link(&variables.build_dir, &file.0, "~/.vim/autoload/lightline/colorscheme");},
            "walmate-theme.el" => {link(&variables.build_dir, &file.0, "~/.emacs.d/themes/");},
            "dunstrc" => {link(&variables.build_dir, &file.0, "~/.config/dunst/");},
            "walmate_qutebrowser.py" => {link(&variables.build_dir, &file.0, "~/.config/qutebrowser/config.py");},
            "themerc" => {link(&variables.build_dir, &file.0, "~/.themes/walmate/openbox-3/");},
            "walmate.sublime-theme" => {link(&variables.build_dir, &file.0, "~/syc/Dropbox/dot/cnf/sublime-text-3/Packages/'Theme - Walmate'/walmate.sublime-theme");},
            "walmate.tmTheme" => {link(&variables.build_dir, &file.0, "~/syc/Dropbox/dot/cnf/sublime-text-3/Packages/'Theme - Walmate'/");},
            "Widget - Walmate.stTheme" => {link(&variables.build_dir, &file.0, "~/syc/Dropbox/dot/cnf/sublime-text-3/Packages/'Theme - Walmate'/widgets/");},
            "walmate-color-palette.scss" => {link(&variables.build_dir, &file.0, "~/syc/Dropbox/prg/rust/walmate/gtk/walmate-gtk-theme/src/_sass/_color-palette.scss")},
            "colors.conf" => {link(&variables.build_dir, &file.0, "~/syc/Dropbox/dot/cnf/kitty/colors.conf");},
            "_color-palette.scss" => {link(&variables.build_dir, &file.0, "~/prg/rust/walmate/walmate-gtk-theme/src/_sass/_color-palette.scss");},
            _ => {},
        }
    }

    command(&("ln -s ".to_string() + &get_home_dir(&variables.build_dir) + "colors " + "~/.config/vis/colors/walmate"));

    post();
}
