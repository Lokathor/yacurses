#!/bin/sh
bindgen --disable-name-namespacing --impl-debug --no-doc-comments \
 --no-layout-tests --no-prepend-enum-name --size_t-is-usize --use-core \
 --ctypes-prefix chlorine --output src/bind.rs \
 --raw-line '#![allow(bad_style)]' \
 --raw-line '#[link(name = "ncurses")] extern "C" {}' \
 --rust-target "1.33" \
 --whitelist-var '(COLOR.*|KEY.*|ERR|stdscr|acs_map)' \
 --whitelist-function '(initscr|endwin|isendwin|def_prog_mode|color_content|pair_content)' \
 --whitelist-function '(start_color|has_colors|can_change_color|init_color|init_pair|COLOR_PAIR)' \
 --whitelist-function '(keypad|echo|noecho|cbreak|nocbreak|nl|nonl|raw|noraw|curs_set)' \
 --whitelist-function '(flushinp|ungetch|clearok|immedok|leaveok|scrollok|wechochar|curs_bkgd)' \
 --whitelist-function 'w(move|attron|attroff|timeout|getch|setscrreg|chgat|addch|addnstr|addchnstr|insch|delch|refresh|vline|hline|scrl)' \
 --whitelist-function 'get(curx|cury|maxx|maxy)' \
 --opaque-type WINDOW \
 wrapper.h
