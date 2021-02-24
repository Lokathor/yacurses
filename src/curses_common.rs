#![allow(bad_style)]

//! Declarations common to both pdcurses and ncurses (which is most things).

// Note(Lokathor): Normally I wouldn't use the standard library to get the C
// types, that way we could be no_std compatible, but we're already tied to the
// standard library because we juggle around the panic hook. It would take extra
// work to make that part optional, so for now this lets has no dependencies.
use std::os::raw::*;

pub type chtype = c_uint;

pub const COLOR_BLACK: u32 = 0;
pub const COLOR_RED: u32 = 1;
pub const COLOR_GREEN: u32 = 2;
pub const COLOR_YELLOW: u32 = 3;
pub const COLOR_BLUE: u32 = 4;
pub const COLOR_MAGENTA: u32 = 5;
pub const COLOR_CYAN: u32 = 6;
pub const COLOR_WHITE: u32 = 7;
pub const ERR: i32 = -1;
pub const KEY_DOWN: u32 = 258;
pub const KEY_UP: u32 = 259;
pub const KEY_LEFT: u32 = 260;
pub const KEY_RIGHT: u32 = 261;
pub const KEY_HOME: u32 = 262;
pub const KEY_BACKSPACE: u32 = 263;
pub const KEY_F0: u32 = 264;
pub const KEY_DC: u32 = 330;
pub const KEY_IC: u32 = 331;
pub const KEY_NPAGE: u32 = 338;
pub const KEY_PPAGE: u32 = 339;
pub const KEY_ENTER: u32 = 343;
pub const KEY_RESIZE: u32 = 410;

#[repr(transparent)]
pub struct WINDOW(c_void);

// Note(Lokathor): READ ONLY!
extern "C" {
  pub static mut stdscr: *mut WINDOW;

  pub static mut COLORS: c_int;

  pub static mut COLOR_PAIRS: c_int;
}

extern "C" {
  pub fn can_change_color() -> bool;

  pub fn cbreak() -> c_int;

  pub fn color_content(
    arg1: c_short, arg2: *mut c_short, arg3: *mut c_short, arg4: *mut c_short,
  ) -> c_int;

  pub fn curs_set(arg1: c_int) -> c_int;

  pub fn def_prog_mode() -> c_int;

  pub fn echo() -> c_int;

  pub fn endwin() -> c_int;

  pub fn flushinp() -> c_int;

  pub fn getbkgd(arg1: *mut WINDOW) -> chtype;

  pub fn has_colors() -> bool;

  pub fn initscr() -> *mut WINDOW;

  pub fn init_color(
    arg1: c_short, arg2: c_short, arg3: c_short, arg4: c_short,
  ) -> c_int;

  pub fn init_pair(arg1: c_short, arg2: c_short, arg3: c_short) -> c_int;

  pub fn isendwin() -> bool;

  pub fn keypad(arg1: *mut WINDOW, arg2: bool) -> c_int;

  pub fn noecho() -> c_int;

  pub fn pair_content(
    arg1: c_short, arg2: *mut c_short, arg3: *mut c_short,
  ) -> c_int;

  pub fn scrollok(arg1: *mut WINDOW, arg2: bool) -> c_int;

  pub fn start_color() -> c_int;

  pub fn ungetch(arg1: c_int) -> c_int;

  pub fn waddch(arg1: *mut WINDOW, arg2: chtype) -> c_int;

  pub fn waddchnstr(
    arg1: *mut WINDOW, arg2: *const chtype, arg3: c_int,
  ) -> c_int;

  pub fn waddnstr(arg1: *mut WINDOW, arg2: *const c_char, arg3: c_int)
    -> c_int;

  pub fn wattron(arg1: *mut WINDOW, arg2: c_int) -> c_int;

  pub fn wattroff(arg1: *mut WINDOW, arg2: c_int) -> c_int;

  pub fn wbkgd(arg1: *mut WINDOW, arg2: chtype) -> c_int;

  pub fn wclear(arg1: *mut WINDOW) -> c_int;

  pub fn wcolor_set(
    arg1: *mut WINDOW, arg2: c_short, arg3: *mut c_void,
  ) -> c_int;

  pub fn wdelch(arg1: *mut WINDOW) -> c_int;

  pub fn wgetch(arg1: *mut WINDOW) -> c_int;

  pub fn winsch(arg1: *mut WINDOW, arg2: chtype) -> c_int;

  pub fn wmove(arg1: *mut WINDOW, arg2: c_int, arg3: c_int) -> c_int;

  pub fn wrefresh(arg1: *mut WINDOW) -> c_int;

  pub fn wscrl(arg1: *mut WINDOW, arg2: c_int) -> c_int;

  pub fn wsetscrreg(arg1: *mut WINDOW, arg2: c_int, arg3: c_int) -> c_int;

  pub fn wtimeout(arg1: *mut WINDOW, arg2: c_int);

  pub fn getcurx(arg1: *const WINDOW) -> c_int;

  pub fn getcury(arg1: *const WINDOW) -> c_int;

  pub fn getmaxx(arg1: *const WINDOW) -> c_int;

  pub fn getmaxy(arg1: *const WINDOW) -> c_int;
}
