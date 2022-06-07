use crate::util::command;

pub fn post() {
  // println!("post-=-=-=--=-=-=-=-=-=-=-=-==-==-=-==-=-=-=-=--=-=-");
  std::env::set_current_dir("/home/ord/prg/rust/walmate/layan-base16").unwrap();
  // println!("{:?}", command("cd ~/prg/rust/walmate/gtk/walmate-gtk-theme/"));
  // parse the new sass colors
  println!("{:?}", command("./parse-sass.sh"));
  // install the font
  println!("{:?}", command("./install.sh"));
  command("gsettings set org.gnome.desktop.interface gtk-theme Walmate");
  command("xrdb -load ~/.Xresources");
  command("kill -HUP $(pidof urxvtd)");
  command(r#"emacsclient -e "(load-theme 'walmate t)""#);
  command(r#"cat ~/.config/i3/heading > ~/.config/i3/config"#);
  command(r#"cat ~/.config/i3/colors >> ~/.config/i3/config"#);
  command(r#"cat ~/.config/i3/apply_colors >> ~/.config/i3/config"#);
  command(r#"cat ~/.config/i3/base >> ~/.config/i3/config"#);
}
