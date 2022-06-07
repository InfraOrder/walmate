;;; walmate-theme.el
;;; Commentary:
;; This file is part of walmate-themes
;;; Code:

(require 'doom-themes)

(defgroup walmate-theme nil
  "Options for doom-themes"
  :group 'doom-themes)

(defcustom walmate-padded-modeline doom-themes-padded-modeline
  "If non-nil, adds a 4px padding to the mode-line.
Can be an integer to determine the exact padding."
  :group 'walmate-theme
  :type '(or integer boolean))

(def-doom-theme walmate
  "Doom emacs theme BASED on base16"

  ;; name        gui       256       16
  ((bg         '("#$base01" "white"   "white" ))
   (bg-alt     '("#$base00" nil       nil     ))
   (base0      '("#$base01" "#$base01"        ))
   (base1      (doom-darken base0 0.01))
   (base2      '("#$base02" "#$base02"         ))
   (base3      (doom-darken base2 0.01))
   (base4      '("#$base03" "#$base03" "silver"))
   (base5      '("#$base04" "#$base04" "silver"))
   (base6      '("#$base05" "#$base05" "silver"))
   (base7      '("#$base06" "#$base06" "silver"))
   (base8      '("#$base07" "#$base07" "black" ))
   (fg         '("#$base06" "#$base06" "black"))
   (fg-alt     (doom-darken fg 0.6))

   (grey       '("#$base05" "#$base05" "silver"))
   (red        '("#$base08" "#$base08" "red"))
   (orange     '("#$base09" "#$base09" "brightred"))
   (yellow     '("#$base0A" "#$base0A" "yellow"))
   (green      '("#$base0B" "#$base0B" "green"))
   (blue       '("#$base0C" "#$base0C" "brightblue"))
   (dark-blue  '("#$base0D" "#$base0D" "blue"))
   (teal       (doom-lighten blue 0.1)) ; FIXME replace with real teal
   (magenta    '("#$base0E" "#$base0E" "magenta"))
   (violet     (doom-darken dark-blue 0.1))
   (cyan       '("#$base0F" "#$base0F" "cyan"))
   (dark-cyan  (doom-lighten cyan 0.1))

   ;; face categories
   (highlight      dark-blue)
   (vertical-bar   base0)
   (selection      base3)
   (builtin        blue)
   (comments       grey)
   (doc-comments   (doom-darken grey 0.1))
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
   (modeline-bg     `(,(doom-darken (car bg) 0.1) ,@(cdr base1)))
   (modeline-bg-alt `(,(doom-darken (car bg) 0.14) ,@(cdr base2)))
   (modeline-fg     base8)
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

   (ivy-current-match :background region :distant-foreground grey :weight 'ultra-bold)
   (ivy-minibuffer-match-face-1
    :foreground base5
    :weight 'light)
   (ivy-minibuffer-match-face-2 :inherit 'ivy-minibuffer-match-face-1 :foreground violet :weight 'ultra-bold)
   (ivy-minibuffer-match-face-3 :inherit 'ivy-minibuffer-match-face-2 :foreground blue)
   (ivy-minibuffer-match-face-4 :inherit 'ivy-minibuffer-match-face-2 :foreground red)

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
