" =============================================================================
" Filename: autoload/lightline/colorscheme/base16_eighties.vim
" Author: Chris Lasher
" License: MIT License
" Last Change: 2016-11-27
" =============================================================================
let s:base00 = "#$base00"
let s:base01 = "#$base01"
let s:base02 = "#$base02"
let s:base03 = "#$base03"
let s:base04 = "#$base04"
let s:base05 = "#$base05"
let s:base06 = "#$base06"
let s:base07 = "#$base07"
let s:base08 = "#$base08"
let s:base09 = "#$base09"
let s:base0A = "#$base0A"
let s:base0B = "#$base0B"
let s:base0C = "#$base0C"
let s:base0D = "#$base0D"
let s:base0E = "#$base0E"
let s:base0F = "#$base0F"

let s:p = {'normal': {}, 'inactive': {}, 'insert': {}, 'replace': {}, 'visual': {}, 'tabline': {}}
let s:p.normal.left = [ [ s:base00, s:base0D ], [ s:base05, s:base02 ] ]
let s:p.normal.right = [ [ s:base00, s:base04 ], [ s:base04, s:base02 ] ]
let s:p.inactive.right = [ [ s:base01, s:base03 ], [ s:base01, s:base03 ] ]
let s:p.inactive.left =  [ [ s:base02, s:base03 ], [ s:base01, s:base03 ] ]
let s:p.insert.left = [ [ s:base00, s:base0B ], [ s:base05, s:base02 ] ]
let s:p.replace.left = [ [ s:base00, s:base08 ], [ s:base05, s:base02 ] ]
let s:p.visual.left = [ [ s:base00, s:base0E ], [ s:base05, s:base02 ] ]
let s:p.normal.middle = [ [ s:base03, s:base01 ] ]
let s:p.inactive.middle = [ [ s:base03, s:base01 ] ]
let s:p.tabline.left = [ [ s:base00, s:base03 ] ]
let s:p.tabline.tabsel = [ [ s:base05, s:base01 ] ]
let s:p.tabline.middle = [ [ s:base01, s:base04 ] ]
let s:p.tabline.right = copy(s:p.normal.right)
let s:p.normal.error = [ [ s:base01, s:base08 ] ]
let s:p.normal.warning = [ [ s:base01, s:base0A ] ]

let g:lightline#colorscheme#walmate_lightline#palette = lightline#colorscheme#fill(s:p)
