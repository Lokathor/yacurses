/* automatically generated by rust-bindgen 0.54.1 */

#![allow(bad_style)]
#[link(name = "ncurses")] extern "C" {}

pub const COLOR_BLACK: u32 = 0;
pub const COLOR_RED: u32 = 1;
pub const COLOR_GREEN: u32 = 2;
pub const COLOR_YELLOW: u32 = 3;
pub const COLOR_BLUE: u32 = 4;
pub const COLOR_MAGENTA: u32 = 5;
pub const COLOR_CYAN: u32 = 6;
pub const COLOR_WHITE: u32 = 7;
pub const ERR: i32 = -1;
pub const KEY_CODE_YES: u32 = 256;
pub const KEY_MIN: u32 = 257;
pub const KEY_BREAK: u32 = 257;
pub const KEY_SRESET: u32 = 344;
pub const KEY_RESET: u32 = 345;
pub const KEY_DOWN: u32 = 258;
pub const KEY_UP: u32 = 259;
pub const KEY_LEFT: u32 = 260;
pub const KEY_RIGHT: u32 = 261;
pub const KEY_HOME: u32 = 262;
pub const KEY_BACKSPACE: u32 = 263;
pub const KEY_F0: u32 = 264;
pub const KEY_DL: u32 = 328;
pub const KEY_IL: u32 = 329;
pub const KEY_DC: u32 = 330;
pub const KEY_IC: u32 = 331;
pub const KEY_EIC: u32 = 332;
pub const KEY_CLEAR: u32 = 333;
pub const KEY_EOS: u32 = 334;
pub const KEY_EOL: u32 = 335;
pub const KEY_SF: u32 = 336;
pub const KEY_SR: u32 = 337;
pub const KEY_NPAGE: u32 = 338;
pub const KEY_PPAGE: u32 = 339;
pub const KEY_STAB: u32 = 340;
pub const KEY_CTAB: u32 = 341;
pub const KEY_CATAB: u32 = 342;
pub const KEY_ENTER: u32 = 343;
pub const KEY_PRINT: u32 = 346;
pub const KEY_LL: u32 = 347;
pub const KEY_A1: u32 = 348;
pub const KEY_A3: u32 = 349;
pub const KEY_B2: u32 = 350;
pub const KEY_C1: u32 = 351;
pub const KEY_C3: u32 = 352;
pub const KEY_BTAB: u32 = 353;
pub const KEY_BEG: u32 = 354;
pub const KEY_CANCEL: u32 = 355;
pub const KEY_CLOSE: u32 = 356;
pub const KEY_COMMAND: u32 = 357;
pub const KEY_COPY: u32 = 358;
pub const KEY_CREATE: u32 = 359;
pub const KEY_END: u32 = 360;
pub const KEY_EXIT: u32 = 361;
pub const KEY_FIND: u32 = 362;
pub const KEY_HELP: u32 = 363;
pub const KEY_MARK: u32 = 364;
pub const KEY_MESSAGE: u32 = 365;
pub const KEY_MOVE: u32 = 366;
pub const KEY_NEXT: u32 = 367;
pub const KEY_OPEN: u32 = 368;
pub const KEY_OPTIONS: u32 = 369;
pub const KEY_PREVIOUS: u32 = 370;
pub const KEY_REDO: u32 = 371;
pub const KEY_REFERENCE: u32 = 372;
pub const KEY_REFRESH: u32 = 373;
pub const KEY_REPLACE: u32 = 374;
pub const KEY_RESTART: u32 = 375;
pub const KEY_RESUME: u32 = 376;
pub const KEY_SAVE: u32 = 377;
pub const KEY_SBEG: u32 = 378;
pub const KEY_SCANCEL: u32 = 379;
pub const KEY_SCOMMAND: u32 = 380;
pub const KEY_SCOPY: u32 = 381;
pub const KEY_SCREATE: u32 = 382;
pub const KEY_SDC: u32 = 383;
pub const KEY_SDL: u32 = 384;
pub const KEY_SELECT: u32 = 385;
pub const KEY_SEND: u32 = 386;
pub const KEY_SEOL: u32 = 387;
pub const KEY_SEXIT: u32 = 388;
pub const KEY_SFIND: u32 = 389;
pub const KEY_SHELP: u32 = 390;
pub const KEY_SHOME: u32 = 391;
pub const KEY_SIC: u32 = 392;
pub const KEY_SLEFT: u32 = 393;
pub const KEY_SMESSAGE: u32 = 394;
pub const KEY_SMOVE: u32 = 395;
pub const KEY_SNEXT: u32 = 396;
pub const KEY_SOPTIONS: u32 = 397;
pub const KEY_SPREVIOUS: u32 = 398;
pub const KEY_SPRINT: u32 = 399;
pub const KEY_SREDO: u32 = 400;
pub const KEY_SREPLACE: u32 = 401;
pub const KEY_SRIGHT: u32 = 402;
pub const KEY_SRSUME: u32 = 403;
pub const KEY_SSAVE: u32 = 404;
pub const KEY_SSUSPEND: u32 = 405;
pub const KEY_SUNDO: u32 = 406;
pub const KEY_SUSPEND: u32 = 407;
pub const KEY_UNDO: u32 = 408;
pub const KEY_MOUSE: u32 = 409;
pub const KEY_RESIZE: u32 = 410;
pub const KEY_EVENT: u32 = 411;
pub const KEY_MAX: u32 = 511;
pub type chtype = chlorine::c_uint;
extern "C" {
  pub static mut acs_map: [chtype; 0usize];
}
pub type WINDOW = [u64; 11usize];
pub type attr_t = chtype;
extern "C" {
  pub fn can_change_color() -> bool;
}
extern "C" {
  pub fn cbreak() -> chlorine::c_int;
}
extern "C" {
  pub fn clearok(arg1: *mut WINDOW, arg2: bool) -> chlorine::c_int;
}
extern "C" {
  pub fn color_content(
    arg1: chlorine::c_short, arg2: *mut chlorine::c_short,
    arg3: *mut chlorine::c_short, arg4: *mut chlorine::c_short,
  ) -> chlorine::c_int;
}
extern "C" {
  pub fn COLOR_PAIR(arg1: chlorine::c_int) -> chlorine::c_int;
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
  pub fn has_colors() -> bool;
}
extern "C" {
  pub fn immedok(arg1: *mut WINDOW, arg2: bool);
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
  pub fn leaveok(arg1: *mut WINDOW, arg2: bool) -> chlorine::c_int;
}
extern "C" {
  pub fn nl() -> chlorine::c_int;
}
extern "C" {
  pub fn nocbreak() -> chlorine::c_int;
}
extern "C" {
  pub fn noecho() -> chlorine::c_int;
}
extern "C" {
  pub fn nonl() -> chlorine::c_int;
}
extern "C" {
  pub fn noraw() -> chlorine::c_int;
}
extern "C" {
  pub fn pair_content(
    arg1: chlorine::c_short, arg2: *mut chlorine::c_short,
    arg3: *mut chlorine::c_short,
  ) -> chlorine::c_int;
}
extern "C" {
  pub fn raw() -> chlorine::c_int;
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
  pub fn wchgat(
    arg1: *mut WINDOW, arg2: chlorine::c_int, arg3: attr_t,
    arg4: chlorine::c_short, arg5: *const chlorine::c_void,
  ) -> chlorine::c_int;
}
extern "C" {
  pub fn wdelch(arg1: *mut WINDOW) -> chlorine::c_int;
}
extern "C" {
  pub fn wechochar(arg1: *mut WINDOW, arg2: chtype) -> chlorine::c_int;
}
extern "C" {
  pub fn wgetch(arg1: *mut WINDOW) -> chlorine::c_int;
}
extern "C" {
  pub fn whline(
    arg1: *mut WINDOW, arg2: chtype, arg3: chlorine::c_int,
  ) -> chlorine::c_int;
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
  pub fn wvline(
    arg1: *mut WINDOW, arg2: chtype, arg3: chlorine::c_int,
  ) -> chlorine::c_int;
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
