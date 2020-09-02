/* automatically generated by rust-bindgen 0.54.1 */

#![allow(bad_style)]
#[cfg_attr(unix, link(name = "ncurses"))]
#[cfg_attr(windows, link(name = "pdcurses"))]
extern "C" {}

pub const COLOR_BLACK: u32 = 0;
pub const COLOR_RED: u32 = 1;
pub const COLOR_GREEN: u32 = 2;
pub const COLOR_YELLOW: u32 = 3;
pub const COLOR_BLUE: u32 = 4;
pub const COLOR_MAGENTA: u32 = 5;
pub const COLOR_CYAN: u32 = 6;
pub const COLOR_WHITE: u32 = 7;
pub const ERR: i32 = -1;
pub const KEY_ENTER: u32 = 0x157;
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
pub const KEY_B2: u32 = 350;
pub const KEY_END: u32 = 360;
pub const KEY_RESIZE: u32 = 410;
pub type chtype = chlorine::c_uint;
extern "C" {
  pub static mut acs_map: [chtype; 0usize];
}
pub type WINDOW = [u64; 11usize];
extern "C" {
  pub fn can_change_color() -> bool;
}
extern "C" {
  pub fn cbreak() -> chlorine::c_int;
}
extern "C" {
  pub fn color_content(
    arg1: chlorine::c_short, arg2: *mut chlorine::c_short,
    arg3: *mut chlorine::c_short, arg4: *mut chlorine::c_short,
  ) -> chlorine::c_int;
}
extern "C" {
  pub fn curs_set(arg1: chlorine::c_int) -> chlorine::c_int;
}
extern "C" {
  pub fn def_prog_mode() -> chlorine::c_int;
}
extern "C" {
  pub fn echo() -> chlorine::c_int;
}
extern "C" {
  pub fn endwin() -> chlorine::c_int;
}
extern "C" {
  pub fn flushinp() -> chlorine::c_int;
}
extern "C" {
  pub fn getbkgd(arg1: *mut WINDOW) -> chtype;
}
extern "C" {
  pub fn has_colors() -> bool;
}
extern "C" {
  pub fn initscr() -> *mut WINDOW;
}
extern "C" {
  pub fn init_color(
    arg1: chlorine::c_short, arg2: chlorine::c_short, arg3: chlorine::c_short,
    arg4: chlorine::c_short,
  ) -> chlorine::c_int;
}
extern "C" {
  pub fn init_pair(
    arg1: chlorine::c_short, arg2: chlorine::c_short, arg3: chlorine::c_short,
  ) -> chlorine::c_int;
}
extern "C" {
  pub fn isendwin() -> bool;
}
extern "C" {
  pub fn keypad(arg1: *mut WINDOW, arg2: bool) -> chlorine::c_int;
}
extern "C" {
  pub fn noecho() -> chlorine::c_int;
}
extern "C" {
  pub fn pair_content(
    arg1: chlorine::c_short, arg2: *mut chlorine::c_short,
    arg3: *mut chlorine::c_short,
  ) -> chlorine::c_int;
}
extern "C" {
  pub fn scrollok(arg1: *mut WINDOW, arg2: bool) -> chlorine::c_int;
}
extern "C" {
  pub fn start_color() -> chlorine::c_int;
}
extern "C" {
  pub fn ungetch(arg1: chlorine::c_int) -> chlorine::c_int;
}
extern "C" {
  pub fn waddch(arg1: *mut WINDOW, arg2: chtype) -> chlorine::c_int;
}
extern "C" {
  pub fn waddchnstr(
    arg1: *mut WINDOW, arg2: *const chtype, arg3: chlorine::c_int,
  ) -> chlorine::c_int;
}
extern "C" {
  pub fn waddnstr(
    arg1: *mut WINDOW, arg2: *const chlorine::c_char, arg3: chlorine::c_int,
  ) -> chlorine::c_int;
}
extern "C" {
  pub fn wattron(arg1: *mut WINDOW, arg2: chlorine::c_int) -> chlorine::c_int;
}
extern "C" {
  pub fn wattroff(arg1: *mut WINDOW, arg2: chlorine::c_int) -> chlorine::c_int;
}
extern "C" {
  pub fn wbkgd(arg1: *mut WINDOW, arg2: chtype) -> chlorine::c_int;
}
extern "C" {
  pub fn wclear(arg1: *mut WINDOW) -> chlorine::c_int;
}
extern "C" {
  pub fn wcolor_set(
    arg1: *mut WINDOW, arg2: chlorine::c_short, arg3: *mut chlorine::c_void,
  ) -> chlorine::c_int;
}
extern "C" {
  pub fn wdelch(arg1: *mut WINDOW) -> chlorine::c_int;
}
extern "C" {
  pub fn wgetch(arg1: *mut WINDOW) -> chlorine::c_int;
}
extern "C" {
  pub fn winsch(arg1: *mut WINDOW, arg2: chtype) -> chlorine::c_int;
}
extern "C" {
  pub fn wmove(
    arg1: *mut WINDOW, arg2: chlorine::c_int, arg3: chlorine::c_int,
  ) -> chlorine::c_int;
}
extern "C" {
  pub fn wrefresh(arg1: *mut WINDOW) -> chlorine::c_int;
}
extern "C" {
  pub fn wscrl(arg1: *mut WINDOW, arg2: chlorine::c_int) -> chlorine::c_int;
}
extern "C" {
  pub fn wsetscrreg(
    arg1: *mut WINDOW, arg2: chlorine::c_int, arg3: chlorine::c_int,
  ) -> chlorine::c_int;
}
extern "C" {
  pub fn wtimeout(arg1: *mut WINDOW, arg2: chlorine::c_int);
}
extern "C" {
  pub fn getcurx(arg1: *const WINDOW) -> chlorine::c_int;
}
extern "C" {
  pub fn getcury(arg1: *const WINDOW) -> chlorine::c_int;
}
extern "C" {
  pub fn getmaxx(arg1: *const WINDOW) -> chlorine::c_int;
}
extern "C" {
  pub fn getmaxy(arg1: *const WINDOW) -> chlorine::c_int;
}
extern "C" {
  pub static mut stdscr: *mut WINDOW;
}
extern "C" {
  pub static mut COLORS: chlorine::c_int;
}
extern "C" {
  pub static mut COLOR_PAIRS: chlorine::c_int;
}
