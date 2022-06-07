# Solarized Dark

evaluate-commands %sh{
	base03='rgb:002b36'
	base02='rgb:073642'
	base01='rgb:586e75'
	base00='rgb:657b83'
	base0='rgb:839496'
	base1='rgb:93a1a1'
	base2='rgb:eee8d5'
	base3='rgb:fdf6e3'
	yellow='rgb:b58900'
	orange='rgb:cb4b16'
	red='rgb:dc322f'
	magenta='rgb:d33682'
	violet='rgb:6c71c4'
	blue='rgb:268bd2'
	cyan='rgb:2aa198'
	green='rgb:859900'

   echo "
        # code
        face global value              ${cyan}
        face global type               ${red}
        face global variable           ${blue}
        face global module             ${cyan}
        face global function           ${blue}
        face global string             ${cyan}
        face global keyword            ${green}
        face global operator           ${yellow}
        face global attribute          ${violet}
        face global comment            ${base01}
        face global documentation      comment
        face global meta               ${orange}
        face global builtin            default+b

        # markup
        face global title              ${blue}+b
        face global header             ${blue}
        face global bold               ${base0}+b
        face global italic             ${base0}+i
        face global mono               ${base1}
        face global block              ${cyan}
        face global link               ${base1}
        face global bullet             ${yellow}
        face global list               ${green}

        # builtin
        face global Default            ${base0},${base03}
        face global PrimarySelection   ${base03},${blue}+fg
        face global SecondarySelection ${base01},${base1}+fg
        face global PrimaryCursor      ${base03},${base0}+fg
        face global SecondaryCursor    ${base03},${base01}+fg
        face global PrimaryCursorEol   ${base03},${base2}+fg
        face global SecondaryCursorEol ${base03},${base3}+fg
        face global LineNumbers        ${base01},${base02}
        face global LineNumberCursor   ${base1},${base02}
        face global LineNumbersWrapped ${base02},${base02}
        face global MenuForeground     ${base03},${yellow}
        face global MenuBackground     ${base1},${base02}
        face global MenuInfo           ${base01}
        face global Information        ${base02},${base1}
        face global Error              ${red},default+b
        face global StatusLine         ${base1},${base02}+b
        face global StatusLineMode     ${orange}
        face global StatusLineInfo     ${cyan}
        face global StatusLineValue    ${green}
        face global StatusCursor       ${base00},${base3}
        face global Prompt             ${yellow}+b
        face global MatchingChar       ${red},${base01}+b
        face global BufferPadding      ${base01},${base03}
        face global Whitespace         ${base01}+f
    "
}

# # Walmate theme
# 
# evaluate-commands %sh{
	# # Color palette
	# base00="rgb:$base00"
	# base02="rgb:$base02"
	# base03="rgb:$base03"
	# base04="rgb:$base04"
	# base05="rgb:$base05"
	# base06="rgb:$base06"
	# base07="rgb:$base07"
	# base08="rgb:$base08"
	# base09="rgb:$base09"
	# base0A="rgb:$base0A"
	# base0B="rgb:$base0B"
	# base0C="rgb:$base0C"
	# base0D="rgb:$base0D"
	# base0E="rgb:$base0E"
	# base0F="rgb:$base0F"
# 

	# echo "
		# # Reference
		# # https://github.com/mawww/kakoune/blob/master/colors/default.kak
		# # For code
		# face global value 			${base08}
		# face global type 			  ${base09}
		# face global variable 		${base0E}
		# face global module 			${base0C}
		# face global function 		${base0D}
		# face global string 			${base0B}
		# face global keyword 		${base0A}
		# face global operator 		${base0D}
		# face global attribute  	${base09}
		# face global comment 		${base01}+i
		# face global meta 			  ${base08}
		# face global builtin 		${base0F}+b
# 
		# # For markup
		# face global title  ${base0D}
		# face global header ${base0C}
		# face global bold   ${base08}
		# face global italic ${base09}
		# face global mono   ${base0B}
		# face global block  ${base0A}
		# face global link   ${base0D}
		# face global bullet ${base0E}
		# face global list   ${base0F}
# 
		# # Builtin faces
		# face global Default            ${base06}, ${base01}
		# face global PrimarySelection   ${base07}, ${base0D}+fg
		# face global SecondarySelection ${base00}, ${base0D}+fg
		# face global PrimaryCursor      ${base00}, ${base07}+fg
		# face global SecondaryCursor    ${base00}, ${base07}+fg
		# face global PrimaryCursorEol   ${base00}, ${base0C}+fg
		# face global SecondaryCursorEol ${base00}, ${base0C}+fg
		# face global LineNumbers        ${base06}, ${base01}
		# face global LineNumberCursor   ${base06}, ${base02}+r
		# face global MenuForeground     ${base07}, ${base0D}
		# face global MenuBackground     ${base0D}, ${base07}
		# face global MenuInfo           ${base0C}
		# face global Information        ${base00}, ${base0A}
		# face global Error              ${base00}, ${base08}
		# face global StatusLine         ${base0C}, ${base02}
		# face global StatusLineMode     ${base0A}
		# face global StatusLineInfo     ${base0D}
		# face global StatusLineValue    ${base0B}
		# face global StatusCursor       ${base00}, ${base0C}
		# face global Prompt             ${base03}, ${base0C}
		# face global MatchingChar       ${base0C}, ${base01}+b
		# face global Whitespace         ${base05}
		# face global BufferPadding      ${base0D}, ${base02}
	# "
# }
# 
