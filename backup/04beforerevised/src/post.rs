use crate::util::command;

pub fn post() {
    command("cd ~/prg/rust/walmate/gtk/walmate-gtk-theme/");
    // parse the new sass colors
    command("~/prg/rust/walmate/gtk/walmate-gtk-theme/parse-sass.sh");
    // install the font
    command("~/prg/rust/walmate/gtk/walmate-gtk-theme/install.sh");
    command("gsettings set org.gnome.desktop.interface gtk-theme walmate");
    command("xrdb -load ~/.Xresources");
    command("kill -HUP $(pidof urxvtd)");
    command(r#"emacsclient -e "(load-theme 'walmate t)""#);
}

pub fn change_active_windows(display_type: String) {
    // change the display mode for the gdk theme
    // change the display mode for the light theme
    // load Xresources
    // reload urxvtd
    // reload emacs theme
    match display_type.as_ref() {
        "light" => {
            // command("gsettings set org.gnome.desktop.interface icon-theme Papirus-Dark");
            command("gsettings set org.gnome.desktop.interface gtk-theme Layan-light");
        },
        "dark" => {
            // command("gsettings set org.gnome.desktop.interface icon-theme Papirus-Light");
            command("gsettings set org.gnome.desktop.interface gtk-theme Layan-dark");
        },
        _ => panic!("unkown display type specified")
    }
    command("xrdb -load ~/.Xresources");
    command("kill -HUP $(pidof urxvtd)");
    command(r#"emacsclient -e "(load-theme 'walmate t)" "#);
}
