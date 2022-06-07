let s:base00 = [ '#$base00',  0 ] " black
let s:base01 = [ '#$base01', 18 ]
let s:base02 = [ '#$base02', 19 ]
let s:base03 = [ '#$base03',  8 ]
let s:base04 = [ '#$base04', 20 ]
let s:base05 = [ '#$base05',  7 ]
let s:base06 = [ '#$base06', 21 ]
let s:base07 = [ '#$base07', 15 ] " white

let s:base08 = [ '#$base08',  1 ] " red
let s:base09 = [ '#$base09', 16 ] " orange
let s:base0A = [ '#$base0A',  3 ] " yellow
let s:base0B = [ '#$base0B',  2 ] " green
let s:base0C = [ '#$base0C',  6 ] " teal
let s:base0D = [ '#$base0D',  4 ] " blue
let s:base0E = [ '#$base0E',  5 ] " pink
let s:base0F = [ '#$base0F', 17 ] " brown

let s:p = {'normal': {}, 'inactive': {}, 'insert': {}, 'replace': {}, 'visual': {}, 'tabline': {}}

let s:p.normal.left     = [ [ s:base00, s:base0D ], [ s:base05, s:base02 ] ]
let s:p.insert.left     = [ [ s:base01, s:base0B ], [ s:base05, s:base02 ] ]
let s:p.visual.left     = [ [ s:base00, s:base09 ], [ s:base05, s:base02 ] ]
let s:p.replace.left    = [ [ s:base00, s:base08 ], [ s:base05, s:base02 ] ]
let s:p.inactive.left   = [ [ s:base02, s:base00 ] ]

let s:p.normal.middle   = [ [ s:base07, s:base00 ] ]
let s:p.inactive.middle = [ [ s:base01, s:base00 ] ]

let s:p.normal.right    = [ [ s:base01, s:base03 ], [ s:base06, s:base02 ] ]
let s:p.inactive.right  = [ [ s:base01, s:base00 ] ]

let s:p.normal.error    = [ [ s:base07, s:base08 ] ]
let s:p.normal.warning  = [ [ s:base07, s:base09 ] ]

let s:p.tabline.left    = [ [ s:base05, s:base02 ] ]
let s:p.tabline.middle  = [ [ s:base05, s:base01 ] ]
let s:p.tabline.right   = [ [ s:base05, s:base02 ] ]
let s:p.tabline.tabsel  = [ [ s:base02, s:base0A ] ]

let g:lightline#colorscheme#walmate#palette = lightline#colorscheme#flatten(s:p)
