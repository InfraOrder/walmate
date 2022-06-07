# Walmate
evaluate-commands %sh{
	base00="rgb:$base0100"
	base01="rgb:$base0000"
	base02="rgb:$base02"
	base03="rgb:$base03"
	base04="rgb:$base04"
	base05="rgb:$base05"
	base06="rgb:$base06"
	base07="rgb:$base07"
	base08="rgb:$base08"
	base09="rgb:$base09"
	base0A="rgb:$base0A"
	base0B="rgb:$base0B"
	base0C="rgb:$base0C"
	base0D="rgb:$base0D"
	base0E="rgb:$base0E"
	base0F="rgb:$base0F"

  echo "
  	# code
  	face global value              ${base08}
  	face global type               ${base09}
  	face global variable           ${base0E}
  	face global module             ${base0C}
  	face global function           ${base0D}
  	face global string             ${base0B}
  	face global keyword            ${base0A}
  	face global operator           ${base0D}
  	face global attribute          ${base09}
  	face global comment            ${base02}+i
  	face global documentation      ${base08}
  	face global meta               ${base0F}+b
  	face global builtin            ${base0F}+b

  	# markup
		face global title  						 ${base0D}
		face global header 						 ${base0C}
		face global bold   						 ${base08}
		face global italic 						 ${base09}
		face global mono   						 ${base0B}
		face global block  						 ${base0A}
		face global link   						 ${base0D}
		face global bullet 						 ${base0E}
		face global list   						 ${base0F}

		# builtin
		face global Default            ${base06},${base01}
		face global PrimarySelection   ${base07},${base0D}+fg
		face global SecondarySelection ${base00},${base0D}+fg
		face global PrimaryCursor      ${base00},${base07}+fg
		face global SecondaryCursor    ${base00},${base07}+fg
		face global PrimaryCursorEol   ${base00},${base0C}+fg
		face global SecondaryCursorEol ${base00},${base0C}+fg
		face global LineNumbers        ${base06},${base01}
		face global LineNumberCursor   ${base06},${base02}+r
		face global MenuForeground     ${base07},${base0D}
		face global MenuBackground     ${base0D},${base07}
		face global MenuInfo           ${base0C}
		face global Information        ${base00},${base0A}
		face global Error              ${base00},${base08}
		face global StatusLine         ${base0C},${base02}
		face global StatusLineMode     ${base0A}
		face global StatusLineInfo     ${base0D}
		face global StatusLineValue    ${base0B}
		face global StatusCursor       ${base00},${base0C}
		face global Prompt             ${base03},${base0C}
		face global MatchingChar       ${base0C},${base01}+b
		face global Whitespace         ${base01}
		face global BufferPadding      ${base0D},${base02}
	"
}
