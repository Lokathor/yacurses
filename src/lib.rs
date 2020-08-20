#![no_std]
#![warn(missing_docs)]
#![allow(unused_imports)]

//! Yet another curses library.

use core::convert::TryFrom;
use core::{
  convert::TryInto,
  ops::*,
  sync::atomic::{AtomicBool, Ordering},
};
use core::num::NonZeroU8;

#[allow(dead_code)]
mod bind;
use bind::*;

/// We're doing an unsafe call, then turning the `c_int` into a `Result`.
macro_rules! unsafe_call_result {
  ($func:ident($($tree:tt)*)) => {
    if ERR == unsafe { $func($($tree)*) } {
      Err(())
    } else {
      Ok(())
    }
  }
}

/// We're doing an unsafe call that is documented to never error.
/// Just to be sure, we will at least `debug_assert` that we didn't get an error.
macro_rules! unsafe_always_ok {
  ($func:ident($($tree:tt)*)) => {{
    let ret = unsafe { $func($($tree)*) };
    debug_assert!(ret != ERR);
  }}
}

/// Handle to the terminal's curses interface.
#[repr(transparent)]
pub struct Curses {
  ptr: *mut WINDOW,
}
static CURSES_ACTIVE: AtomicBool = AtomicBool::new(false);
impl Drop for Curses {
  fn drop(&mut self) {
    // Save the settings before we shut down curses, in case it's resumed later.
    // in case of error, we just accept it.
    let _ = unsafe { def_prog_mode() };
    // this should only error if curses wasn't initialized.
    assert_ne!(unsafe { endwin() }, ERR);
    CURSES_ACTIVE.store(false, Ordering::SeqCst);
  }
}
impl Curses {
  /// Initializes curses.
  ///
  /// * Automatically enables color, if available.
  /// * Automatically enables keypad keys (arrow keys, function keys, etc).
  ///
  /// ## Panics
  /// * If you double-initialize curses this will panic.
  /// * If your previous curses handle has been dropped it **is** legal to init another one.
  ///   Curses mode will resume just fine.
  /// ## Other
  /// * If this fails on the curses side, curses will "helpfully" print an error
  ///   and abort the process for you.
  pub fn init() -> Self {
    if !CURSES_ACTIVE.compare_and_swap(false, true, Ordering::SeqCst) {
      if unsafe { isendwin() } {
        let mut w = Self { ptr: unsafe { stdscr } };
        w.refresh().unwrap();
        w
      } else {
        let win = Self { ptr: unsafe { initscr() } };
        assert!(!win.ptr.is_null());
        // technically this could fail to allocate the color table, but if so
        // we'll just get other errors if people do use color later on. If we
        // failed to allocate the color table but color isn't used, then there's
        // no reason to raise a fuss.
        let _ = unsafe_call_result!(start_color());
        // this only fails if curses isn't init or the ptr is null, so it should
        // never fail here since we checked for null already. However, if it
        // somehow does fail anyway, then the worst that happens is that the user
        // can't use the keypad keys.
        let _ = unsafe_call_result!(keypad(win.ptr, true));
        // We always want to operate in cbreak mode, so set it here and don't
        // expose this option to the user. In this case, if `cbreak` isn't set then
        // things will be weird as hell, so we panic on failure.
        unsafe_call_result!(cbreak()).expect("Couldn't set `cbreak` mode.");
        win
      }
    } else {
      panic!("Curses is already active.")
    }
  }
  
  /// Pushes all updates out to the physical screen, refreshing the display.
  pub fn refresh(&mut self) -> Result<(), ()> {
    unsafe_call_result!(wrefresh(self.ptr))
  }
  
  /// Sets if user inputs should automatically echo to the screen or not.
  ///
  /// * Initially this is enabled.
  pub fn set_echo(&mut self, echoing: bool) -> Result<(), ()> {
    if echoing {
      unsafe_call_result!(echo())
    } else {
      unsafe_call_result!(noecho())
    }
  }
  
  /// Get the cursor's current row and column.
  pub fn get_cursor_position(&self) -> Position {
    let x = unsafe { getcurx(self.ptr) as u32 };
    let y = unsafe { getcury(self.ptr) as u32 };
    Position { x, y }
  }
  
  /// Get the size of the terminal.
  ///
  /// Cursor positions can range in `0..COUNT` in each dimension.
  pub fn get_terminal_size(&self) -> TerminalSize {
    let x_count = unsafe { getmaxx(self.ptr) as u32 };
    let y_count = unsafe { getmaxy(self.ptr) as u32 };
    TerminalSize { x_count, y_count }
  }
  
  /// Move the cursor to the position given.
  pub fn move_cursor(&mut self, p: Position) -> Result<(), ()> {
    unsafe_call_result!(wmove(self.ptr, p.y as _, p.x as _))
  }

  /// Prints the character given, advancing the cursor.
  ///
  /// * Wraps to the next line if in the final col.
  /// * Will scroll the terminal if in the final row, if scrolling is enabled.
  pub fn print_ch<C: Into<CursesGlyph>>(&mut self, c: C) -> Result<(), ()> {
    unsafe_call_result!(waddch(self.ptr, c.into().as_chtype()))
  }

  /// Gets an input event.
  ///
  /// * Ascii keys are returned as their ascii value.
  /// * Keypad keys are values >= 256.
  /// * If the terminal is resized, that shows up as a special key in this queue.
  /// * If you have a timeout set and the time expires, you get `None` back.
  pub fn poll_events(&mut self) -> Option<u32> {
    // TODO: make the events a proper enum.
    const ERR_U32: u32 = ERR as u32;
    let k = unsafe { wgetch(self.ptr) };
    match k as u32 {
      ERR_U32 => None,
      other => Some(other as _)
    }
  }

  /// Return the terminal to shell mode temporarily.
  pub fn shell_mode<'a>(&'a mut self) -> Result<CursesShell<'a>, ()> {
    unsafe_always_ok!(def_prog_mode());
    unsafe_call_result!(endwin()).map(move |_|{
      CursesShell { win: self }
    })
  }
  
  /// If the terminal is able to change the RGB values of a given [`ColorID`]
  pub fn can_change_colors(&self) -> bool {
    unsafe { can_change_color() }
  }
  
  /// Gets the highest allowed color id for this terminal.
  pub fn get_max_color_id_inclusive(&self) -> Option<ColorID> {
    let colors = unsafe { COLORS };
    if colors > 0 {
      Some(ColorID(unsafe { (COLORS-1).try_into().unwrap_or(u8::MAX) }))
    } else {
      None
    }
  }
  
  /// Gets the highest allowed color pair for this terminal.
  pub fn get_max_color_pair_inclusive(&self) -> Option<ColorPair> {
    NonZeroU8::new(unsafe { (COLOR_PAIRS-1).try_into().unwrap_or(u8::MAX) }).map(ColorPair)
  }
  
  /// Sets the color id to use the RGB values given, or closest approximation available.
  ///
  /// Inputs are clamped to the range `0.0 ..= 1.0`
  pub fn set_color_id_rgb(&mut self, c: ColorID, [r, g, b]: [f32; 3]) -> Result<(), ()> {
    let r_i16 = (r.max(0.0).min(1.0) * 1000.0) as i16;
    let g_i16 = (g.max(0.0).min(1.0) * 1000.0) as i16;
    let b_i16 = (b.max(0.0).min(1.0) * 1000.0) as i16;
    unsafe_call_result!(init_color(c.0.into(), r_i16, g_i16, b_i16))
  }
  
  /// Gets the RGB values of the given color id.
  pub fn get_color_id_rgb(&self, c: ColorID) -> Result<[f32; 3], ()> {
    let mut r_i16 = 0;
    let mut g_i16 = 0;
    let mut b_i16 = 0;
    unsafe_call_result!(color_content(c.0.into(), &mut r_i16, &mut g_i16, &mut b_i16)).map(|_| {
      let r = r_i16 as f32 / 1000.0;
      let g = g_i16 as f32 / 1000.0;
      let b = b_i16 as f32 / 1000.0;
      [r,g,b]
    })
  }
  
  /// Assigns the selected color pair to use the foreground and background specified.
  ///
  /// A character cell is associated to a given color pair, so changing any color pair will
  /// immediately change all character cells displaying the color pair.
  pub fn set_color_pair_content(&mut self, pair: ColorPair, fg: ColorID, bg: ColorID) -> Result<(), ()> {
    unsafe_call_result!(init_pair(pair.0.get().into(), fg.0.into(), bg.0.into()))
  }
  
  /// Gets the RGB values of the given color id.
  pub fn get_color_pair_content(&self, c: ColorID) -> Result<(ColorID, ColorID), ()> {
    let mut f_i16 = 0;
    let mut b_i16 = 0;
    unsafe_call_result!(pair_content(c.0.into(), &mut f_i16, &mut b_i16)).and_then(|_|{
      match (u8::try_from(f_i16), u8::try_from(b_i16)) {
        (Ok(f), Ok(b)) => Ok((ColorID(f), ColorID(b))),
        _ => Err(())
      }
    })
  }
}

/// A position on the screen.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Position {
  /// The `x` position (aka `col`)
  pub x: u32,
  /// The `y` position (aka `row`)
  pub y: u32,
}

/// Used to return info about the upper bounds of the screen.
///
/// A struct because it's 2-dimensional instead of just a single length or whatever.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TerminalSize {
  /// The number of `x` positions on the screen, valid `x` will be `0..x_count`
  pub x_count: u32,
  /// The number of `y` positions on the screen, valid `y` will be `0..y_count`
  pub y_count: u32,
}

/// A single ascii value to draw to the screen, along with color and attributes.
#[derive(Debug, Clone, Copy)]
#[repr(C, align(4))]
// Note(Lokathor): align is 4 so that &CursesGlyph is also a valid &chtype
pub struct CursesGlyph {
  /// The text to show (`0..128`)
  pub ascii: u8,
  
  /// The color pairing to use, if any.
  pub opt_color_pair: Option<ColorPair>,
  
  /// The other attributes to use.
  pub attributes: Attributes,
}
impl From<u8> for CursesGlyph {
  fn from(ascii: u8) -> Self {
    Self { ascii, opt_color_pair: None, attributes: Attributes(0) }
  }
}
impl From<char> for CursesGlyph {
  fn from(ch: char) -> Self {
    let ascii = ch as u8;
    Self { ascii, opt_color_pair: None, attributes: Attributes(0) }
  }
}
impl CursesGlyph {
  /// Turn into a `chtype` for sending to ncurses.
  fn as_chtype(self) -> chtype {
    unsafe { core::mem::transmute(self) }
  }
}

/// Names a color within curses.
///
/// This is **not** an actual RGB color value. It's just an index into a color palette.
/// * Assuming that a terminal supports color at all, you usually get at least 8 palette slots.
/// * The linux console generally has only 8 colors, but terminal emulators often have far more.
///
/// This type has some associated constants.
/// Each constant names the id value that is most likely to display as that color by default.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct ColorID(pub u8);
#[allow(missing_docs)]
impl ColorID {
  pub const BLACK: ColorID = ColorID(COLOR_BLACK as u8);
  pub const RED: ColorID = ColorID(COLOR_RED as u8);
  pub const GREEN: ColorID = ColorID(COLOR_GREEN as u8);
  pub const YELLOW: ColorID = ColorID(COLOR_YELLOW as u8);
  pub const BLUE: ColorID = ColorID(COLOR_BLUE as u8);
  pub const MAGENTA: ColorID = ColorID(COLOR_MAGENTA as u8);
  pub const CYAN: ColorID = ColorID(COLOR_CYAN as u8);
  pub const WHITE: ColorID = ColorID(COLOR_WHITE as u8);
}

/// Names a foreground / background color pairing within curses.
///
/// Curses remembers a color pair id for each character cell in the terminal.
/// Each color pair maps to a foreground color and background color id palette value.
/// The pair id is a single byte, but 0 is a special "no color" value.
/// It uses whatever the terminal's default colors were before curses was initialized.
/// Accordingly, we model the color pair in Rust as a NonZeroU8, and wrap it in an Option as appropriate.
///
/// You can generally change what color ids that are associated with a color pair id.
/// The number of color pairs actually available varies by terminal, though generally at least 64 are supported.
///
/// If you change the colors of a color pair, all character cells on the screen using that pairing will
/// have their displayed colors immediately changed accordingly.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct ColorPair(pub NonZeroU8);

/// Attributes that can be applied to a character's cell (a bitflag value).
///
/// Useful attributes have named constants.
/// Other bits are generally ineffective.
/// None of the bits can cause a safety concern.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Attributes(pub u16);
impl Attributes {
  /// The text should stand out in some way.
  ///
  /// * Usually the same as `REVERSE`.
  pub const STANDOUT: Attributes = Attributes(1 << 0);
  
  /// The text should be underlined.
  ///
  /// * Linux Console: No.
  /// * Terminal Emulator: Maybe.
  pub const UNDERLINE: Attributes = Attributes(1 << 1);
  
  /// The textshould have foreground and background colors reversed.
  ///
  /// * Works basically everywhere color does.
  pub const REVERSE: Attributes = Attributes(1 << 2);
  
  /// The text should blink.
  ///
  /// * Linux Console: Visually distinct, but doesn't actually blink.
  /// * Terminal Emulator: Probably actually blinks.
  pub const BLINK: Attributes = Attributes(1 << 3);
  
  /// The text should be dim.
  ///
  /// * Linux Console: No.
  /// * Terminal Emulator: Maybe.
  pub const DIM: Attributes = Attributes(1 << 4);
  
  /// The text should be bold.
  ///
  /// * Works basically everywhere color does.
  pub const BOLD: Attributes = Attributes(1 << 5);
  
  /// The text should use the alternative character set.
  ///
  /// * This always "works", in that you'll see the alternate character for
  ///   the given byte, but what actually displays is up to the terminal.
  ///   Each ACS character is named after the *intended* appearance, at least.
  pub const ALT_CHAR_SET: Attributes = Attributes(1 << 6);
  
  /// The text should be invisible (foreground and background the same).
  ///
  /// * Linux Console: No.
  /// * Terminal Emulator: Maybe.
  pub const INVIS: Attributes = Attributes(1 << 7);
  
  /// The text should be italic.
  ///
  /// * Linux Console: No.
  /// * Terminal Emulator: Maybe.
  pub const ITALIC: Attributes = Attributes(1 << 15);
  
  /*
  /// This would protect against "selective erase", not used in modern terminals.
  pub const PROTECT: Attributes = Attributes(1 << 8);
  pub const HORIZONTAL: Attributes = Attributes(1 << 9);
  pub const LEFT: Attributes = Attributes(1 << 10);
  pub const LOW: Attributes = Attributes(1 << 11);
  pub const RIGHT: Attributes = Attributes(1 << 12);
  pub const TOP: Attributes = Attributes(1 << 13);
  pub const VERTICAL: Attributes = Attributes(1 << 14);
  */
}
impl BitAnd for Attributes {
  type Output = Self;
  #[inline]
  fn bitand(self, rhs: Self) -> Self {
    Self(self.0 & rhs.0)
  }
}
impl BitAndAssign for Attributes {
  #[inline]
  fn bitand_assign(&mut self, rhs: Self) {
    self.0 &= rhs.0
  }
}
impl BitOr for Attributes {
  type Output = Self;
  #[inline]
  fn bitor(self, rhs: Self) -> Self {
    Self(self.0 | rhs.0)
  }
}
impl BitOrAssign for Attributes {
  #[inline]
  fn bitor_assign(&mut self, rhs: Self) {
    self.0 |= rhs.0
  }
}
impl BitXor for Attributes {
  type Output = Self;
  #[inline]
  fn bitxor(self, rhs: Self) -> Self {
    Self(self.0 ^ rhs.0)
  }
}
impl BitXorAssign for Attributes {
  #[inline]
  fn bitxor_assign(&mut self, rhs: Self) {
    self.0 ^= rhs.0
  }
}

/// While you hold this, the terminal is in shell mode.
///
/// In other words, `stdout` and `stderr` will work normally.
///
/// When you drop this, the terminal returns to curses mode.
///
/// The `Deref` impl of this type lets you read info about the curses mode
/// even while in shell mode.
#[repr(transparent)]
pub struct CursesShell<'a> {
  win: &'a mut Curses,
}
impl<'a> Drop for CursesShell<'a> {
  fn drop(&mut self) {
    unsafe_call_result!(wrefresh(self.win.ptr)).unwrap();
  }
}
impl<'a> Deref for CursesShell<'a> {
    type Target = Curses;
    #[inline]
    fn deref(&self) -> &Self::Target {
      self.win
    }
}

/*
/// This is how you check the ACS map in general. Specific usages are below.
fn ncurses_acs(c: char) -> u8 {
  (unsafe { *acs_map.as_ptr().add(c as u8 as usize) }) as u8
}

pub fn acs_ulcorner() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('l'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_llcorner() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('m'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_urcorner() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('k'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_lrcorner() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('j'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_ltee() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('t'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_rtee() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('u'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_btee() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('v'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_ttee() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('w'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_hline() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('q'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_vline() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('x'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_plus() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('n'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_s1() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('o'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_s9() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('s'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_diamond() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('`'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_ckboard() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('a'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_degree() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('f'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_plminus() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('g'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_bullet() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('~'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_larrow() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs(','),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_rarrow() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('+'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_darrow() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('.'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_uarrow() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('-'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_board() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('h'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_lantern() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('i'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_block() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('0'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_s3() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('p'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_s7() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('r'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_lequal() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('y'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_gequal() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('z'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_pi() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('{'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_nequal() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('|'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}

pub fn acs_sterling() -> CursesGlyph {
  CursesGlyph {
    ascii: ncurses_acs('}'),
    color_pair: ColorPair(0),
    attributes: Attributes::ALT_CHAR_SET,
  }
}
*/
