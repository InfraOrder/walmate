;;; walmate.el
(require 'doom-themes)

(defgroup walmate-theme nil
  "Options for doom-themes"
  :group 'doom-themes)

(defcustom walmate-padded-modeline doom-themes-padded-modeline
  "If non-nil, adds a 4px padding to the mode-line. Can be an integer to
determine the exact padding."
  :group 'walmate-theme
  :type '(choice integer boolean))

(def-doom-theme walmate
  "Doom emacs theme BASED on base16"

  ;; name        gui       256       16
  ((bg         '("#$base01" nil       nil          ))
   (bg-alt     '("#$base00" nil       nil          ))
   (base0      '("#$base01" "#$base01"              ))
   (base1      (doom-lighten base0 0.01))
   (base2      '("#$base02" "#$base02"              ))
   (base3      (doom-lighten base2 0.01))
   (base4      '("#$base03" "#$base03" "brightblack"))
   (base5      '("#$base04" "#$base04" "brightblack"))
   (base6      '("#$base05" "#$base05" "brightblack"))
   (base7      '("#$base06" "#$base06" "brightblack"))
   (base8      '("#$base07" "#$base07" "white"      ))
   (fg         '("#$base06" "#$base06" "white"))
   (fg-alt     (doom-darken fg 0.4))

   (grey       '("#$base04" "#$base04" "brightblack"))
   (red        '("#$base08" "#$base08" "red"))
   (orange     '("#$base09" "#$base09" "brightred"))
   (yellow     '("#$base0A" "#$base0A" "yellow"))
   (green      '("#$base0B" "#$base0B" "green"))
   (blue       '("#$base0C" "#$base0C" "brightblue"))
   (dark-blue  '("#$base0D" "#$base0D" "blue"))
   (teal       (doom-lighten blue 0.1)) ; FIXME replace with real teal
   (magenta    '("#$base0E" "#$base0E" "magenta"))
   (violet     (doom-darken blue 0.1))
   (cyan       '("#$base0F" "#$base0F" "cyan"))
   (dark-cyan  (doom-darken cyan 0.1))

   ;; face categories
   (highlight      blue)
   (vertical-bar   `("#$base00" ,@base0))
   (selection      `(,(car (doom-lighten bg 0.1)) ,@(cdr base2)))
   (builtin        blue)
   (comments       grey)
   (doc-comments   (doom-lighten grey 0.14))
   (constants      orange)
   (functions      blue)
   (keywords       violet)
   (methods        blue)
   (operators      fg)
   (type           yellow)
   (strings        green)
   (variables      red)
   (numbers        orange)
   (region         selection)
   (error          red)
   (warning        yellow)
   (success        green)
   (vc-modified    fg-alt)
   (vc-added       green)
   (vc-deleted     red)

   ;; custom categories
   (modeline-bg     `(,(doom-darken (car bg-alt) 0.3) ,@(cdr base1)))
   (modeline-bg-alt `(,(car bg) ,@(cdr base2)))
   (modeline-fg     base7)
   (modeline-fg-alt comments)
   (-modeline-pad
    (when walmate-padded-modeline
      (if (integerp walmate-padded-modeline)
          walmate-padded-modeline
        4))))

  ;; --- faces ------------------------------
  ((doom-modeline-buffer-path       :foreground violet :bold bold)
   (doom-modeline-buffer-major-mode :inherit 'doom-modeline-buffer-path)

   ((line-number &override) :foreground base4)
   ((line-number-current-line &override) :foreground blue :bold bold)

   ;; rainbow-delimiters
   (rainbow-delimiters-depth-1-face :foreground violet)
   (rainbow-delimiters-depth-2-face :foreground blue)
   (rainbow-delimiters-depth-3-face :foreground orange)
   (rainbow-delimiters-depth-4-face :foreground green)
   (rainbow-delimiters-depth-5-face :foreground magenta)
   (rainbow-delimiters-depth-6-face :foreground yellow)
   (rainbow-delimiters-depth-7-face :foreground teal)

   (mode-line
    :background modeline-bg :foreground modeline-fg
    :box (if -modeline-pad `(:line-width ,-modeline-pad :color ,modeline-bg)))
   (mode-line-inactive
    :background modeline-bg-alt :foreground modeline-fg-alt
    :box (if -modeline-pad `(:line-width ,-modeline-pad :color ,modeline-bg-alt))))

  ;; --- variables --------------------------
  ;; ()
  )

(provide 'walmate-theme)
;;; walmate-theme.el ends here
