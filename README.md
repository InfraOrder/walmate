# Walmate
  *Base16 builder clone*
## Setup
  Please ensure you have rust installed with a working version of Cargo.
  [rustup](https://rustup.rs/)

  Sadly this will require a lot of manual work. Also, I am uncertain this will
  work for all systems as I have only tested on my own configs.
  Firstly you will need to get all the base16 themes. I Will need to
  update with more themes in the future as I did not grab a list from somewhere
  and instead manually grabbed them all.
  
  Simply cd into the base16 folder and run the file - commands. Then copy "schemes" to a
  folder called "walmate" in .config - so "~/.config/walmate/schemes"
  
  Then add something similar to this in a file called walmate.ini within the
  same directory (NOTE - Uncertain if I Create folders accordingly if you use
  different folders than specified. So just ensure the folders exist to avoid
  errors assuming you use different directories than default)
  ```ini
  [Directories]
  build_dir=~/.cache/walmate/
  base16_dir=~/.config/walmate/schemes/
  wallpaper_dir=~/Wallapers/dark/

  [GeneralSettings]
  saturation=0.3
  ```
  
  ### You should be good to go now
  ## Generating files
  cd into the src folder of walmate and run 
  * `cargo run -- -h` - to get the help though it is really simple anyway
  * -b *Scheme name* - to select the desired scheme
  * -t *Light or Dark* - to select wether the scheme is a light or dark one (this
  is not necessary if N/A)
  and thats basically it for generating the files.
  
  ## Applying files
  theres some automated things within "src/post.rs" that has some commands that
  generate the gtk theme as well as refresh certain applications - please add or
  remove where necessary

  Theres some more setup required though - like a lot
  Each theme needs to be explicitly added to each application you intend to use.
  
  ## adding your own themes for other applications
  this will hopefully be easy to understand.
  
  firstly - you mostly don't have to worry about creating the theme templates
  from scratch as a lot of people use base16 and theres multiple contributers
  adding templates.
  [Base16 Repo has more info](https://github.com/chriskempson/base16)
  
  I stupidly decided to change the way these templates are used however:
    Instead of {{base00-hex}} i used $base00.
    So you will need to move the mustache template into "converting" and use one
    of the python scripts to convert it appropriatly. Hopefully this is easy
    enough to understand.
  
  Afterwards you will then move it into templates, add it to the main.rs match
  function and system link it to the appropriate location.
  
  It should do the rest for you lol.
  
  
