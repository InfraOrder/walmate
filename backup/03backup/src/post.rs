use crate::util::command;

pub fn change_active_windows(display_type: String) {
    // change the display mode for the gdk theme
    // change the display mode for the light theme
    // load Xresources
    // reload urxvtd
    // reload emacs theme
    match display_type.as_ref() {
        "light" => {
            command("gsettings set org.gnome.desktop.interface icon-theme Papirus-Light");
            command("gsettings set org.gnome.desktop.interface gtk-theme Plata-Lumine");
        },
        "dark" => {
            command("gsettings set org.gnome.desktop.interface icon-theme Papirus-Dark");
            command("gsettings set org.gnome.desktop.interface gtk-theme Plata-Noir");
        },
        _ => panic!("unkown display type specified")
    }
    command("xrdb -load ~/.Xresources");
    command("kill -HUP $(pidof urxvtd)");
    command(r#"emacsclient -e "(load-theme 'walmate t)""#);
}
