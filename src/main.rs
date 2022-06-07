// WALMATE - Base16 Theme builder

mod settings;
use crate::settings::{get_args, get_json_settings, complete_settings};
mod gen_files;
use crate::gen_files::gen_files;

mod util;
use crate::util::{get_home_dir, link, command};
mod scheme;
mod post;
use crate::post::post;

fn main() {
    let cli_settings = get_args();
    let json_settings = get_json_settings();
    let settings = complete_settings(&cli_settings, &json_settings);
    println!("{:?}", settings);
    let gen = gen_files(&settings);

    // writing to file
    for file in &gen {
        std::fs::write(format!("{}{}", get_home_dir(&settings.build_dir), file.name), file.data.to_owned()).expect("unable to write to file");
        match file.name.as_ref() {
            "definitions" => {link(&settings.build_dir, &file.name, "~/.urxvt/colors/");},
            "rofi.rasi" => {link(&settings.build_dir, &file.name, "~/.config/rofi/");},
            "rofi.config" => {link(&settings.build_dir, &file.name, "~/.config/rofi/");},
            "walmate.vim" => {link(&settings.build_dir, &file.name, "~/.config/nvim/colors/");},
            "walmate_lightline.vim" => {link(&settings.build_dir, &file.name, "~/.config/nvim/plugged/lightline.vim/autoload/lightline/colorscheme");},
            "walmate-theme.el" => {link(&settings.build_dir, &file.name, "~/.emacs.d/themes/");},
            "dunstrc" => {link(&settings.build_dir, &file.name, "~/.config/dunst/");},
            "walmate_qutebrowser.py" => {link(&settings.build_dir, &file.name, "~/.config/qutebrowser/config.py");},
            "themerc" => {link(&settings.build_dir, &file.name, "~/.themes/walmate/openbox-3/");},
            "walmate.sublime-theme" => {link(&settings.build_dir, &file.name, "~/syc/Dropbox/dot/cnf/sublime-text-3/Packages/'Theme - Walmate'/walmate.sublime-theme");},
            "walmate.tmTheme" => {link(&settings.build_dir, &file.name, "~/syc/Dropbox/dot/cnf/sublime-text-3/Packages/'Theme - Walmate'/");},
            "Widget - Walmate.stTheme" => {link(&settings.build_dir, &file.name, "~/syc/Dropbox/dot/cnf/sublime-text-3/Packages/'Theme - Walmate'/widgets/");},
            "walmate-color-palette.scss" => {link(&settings.build_dir, &file.name, "~/syc/Dropbox/prg/rust/walmate/gtk/walmate-gtk-theme/src/_sass/_color-palette.scss")},
            "colors.conf" => {link(&settings.build_dir, &file.name, "~/syc/Dropbox/dot/cnf/kitty/colors.conf");},
            "_color-palette.scss" => {link(&settings.build_dir, &file.name, "~/prg/rust/walmate/layan-base16/src/_sass/_color-palette.scss");},
            "walmate.json" => {link(&settings.build_dir, &file.name, "~/.vscode-oss/extensions/infraorder.walmate-base16-themes-1.0.0/themes/")},
            "colors.i3" => {link(&settings.build_dir, &file.name, "~/.config/i3/colors")},
            "walmate.colorscheme" => {link(&settings.build_dir, &file.name, "~/.config/qterminal.org/color-schemes/walmate.colorscheme")},
            "walmate.kak" => {link(&settings.build_dir, &file.name, "~/.config/kak/colors/")},
            _ => {}
        }
    }

    command(&("ln -s ".to_string() + &get_home_dir(&settings.build_dir) + "colors " + "~/.config/vis/colors/walmate"));

    post();
}

