#!/bin/sh
bindgen --disable-name-namespacing --impl-debug --no-doc-comments \
 --no-layout-tests --no-prepend-enum-name --size_t-is-usize --use-core \
 --ctypes-prefix chlorine \
 --output src/ncurses_bind.rs \
 --raw-line '#![allow(bad_style)]' \
 --rust-target "1.33" \
 --whitelist-var '(COLOR.*|ERR|stdscr|acs_map)' \
 --whitelist-var 'KEY_(BACKSPACE|UP|DOWN|LEFT|RIGHT|UP|DOWN|IC|DC|HOME|END|PPAGE|NPAGE|RESIZE|F0|ENTER|B2)' \
 --whitelist-function '(initscr|endwin|isendwin|def_prog_mode|color_content|pair_content)' \
 --whitelist-function '(start_color|has_colors|can_change_color|init_color|init_pair)' \
 --whitelist-function '(keypad|echo|noecho|cbreak|curs_set|wbkgd|getbkgd)' \
 --whitelist-function '(flushinp|ungetch|scrollok|curs_bkgd)' \
 --whitelist-function 'w(move|attron|attroff|timeout|clear|getch|setscrreg|addch|addnstr|addchnstr|insch|delch|refresh|scrl|color_set)' \
 --whitelist-function 'get(curx|cury|maxx|maxy)' \
 --opaque-type WINDOW \
 wrapper.h
