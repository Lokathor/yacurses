#![no_std]
#![warn(missing_docs)]

//! Yet another curses library.
//!
//! This crate binds to either the system [ncurses][1] on Unix (MIT-X11
//! license), or a bundled [pdcurses][2] on Windows (public domain).
//!
//! [1]: https://invisible-island.net/ncurses/
//!
//! [2]: https://pdcurses.org/

use core::{
  convert::{TryFrom, TryInto},
  num::NonZeroU8,
  ops::*,
  sync::atomic::{AtomicBool, Ordering},
};

#[cfg_attr(windows, allow(dead_code))]
mod ncurses_bind;
use ncurses_bind::*;

#[cfg(windows)]
const PADSLASH: u32 = 0x1ca;
#[cfg(windows)]
const PADENTER: u32 = 0x1cb;
#[cfg(windows)]
const PADSTAR: u32 = 0x1cf;
#[cfg(windows)]
const PADMINUS: u32 = 0x1d0;
#[cfg(windows)]
const PADPLUS: u32 = 0x1d1;
#[cfg(windows)]
const KEY_A1: u32 = 0x1c1;
#[cfg(windows)]
const KEY_A2: u32 = 0x1c2;
#[cfg(windows)]
const KEY_A3: u32 = 0x1c3;
#[cfg(windows)]
const KEY_B1: u32 = 0x1c4;
#[cfg(windows)]
const KEY_B3: u32 = 0x1c6;
#[cfg(windows)]
const KEY_C1: u32 = 0x1c7;
#[cfg(windows)]
const KEY_C2: u32 = 0x1c8;
#[cfg(windows)]
const KEY_C3: u32 = 0x1c9;
// Note(Lokathor): shadows the entry from bind.rs
#[cfg(windows)]
const KEY_END: u32 = 0x166;
// Note(Lokathor): shadows the entry from bind.rs
#[cfg(windows)]
const KEY_B2: u32 = 0x1c5;

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
/// Just to be sure, we will at least `debug_assert` that we didn't get an
/// error.
macro_rules! unsafe_always_ok {
  ($func:ident($($tree:tt)*)) => {{
    let ret = unsafe { $func($($tree)*) };
    debug_assert!(ret != ERR);
  }}
}

/// We're doing an unsafe call that returns nothing.
macro_rules! unsafe_void {
  ($func:ident($($tree:tt)*)) => {{
    let _: () = unsafe { $func($($tree)*) };
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
  /// * If your previous curses handle has been dropped it **is** legal to init
  ///   another one. Curses mode will resume just fine.
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
        // somehow does fail anyway, then the worst that happens is that the
        // user can't use the keypad keys.
        let _ = unsafe_call_result!(keypad(win.ptr, true));
        // We always want to operate in cbreak mode, so set it here and don't
        // expose this option to the user. In this case, if `cbreak` isn't set
        // then things will be weird as hell, so we panic on failure.
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

  /// Prints the str given, advancing the cursor.
  ///
  /// This is identical to calling [`print_ch`](Curses::print_ch) on every byte
  /// in `s`. If your `&str` has non-ascii data you'll get garbage on the
  /// screen.
  ///
  /// * Wraps to the next line if in the final col.
  /// * Will scroll the terminal if in the final row, if scrolling is enabled.
  pub fn print_str(&mut self, s: &str) -> Result<(), ()> {
    unsafe_call_result!(waddnstr(
      self.ptr,
      s.as_ptr().cast(),
      s.len().try_into().unwrap()
    ))
  }

  /// Inserts the given character under the cursor.
  ///
  /// * The cursor doesn't move.
  /// * Other characters to the right get pushed 1 cell forward.
  /// * The last character of the line gets pushed off the screen.
  pub fn insert_ch<C: Into<CursesGlyph>>(&mut self, c: C) -> Result<(), ()> {
    unsafe_call_result!(winsch(self.ptr, c.into().as_chtype()))
  }

  /// Deleted character under the cursor.
  ///
  /// * The cursor doesn't move.
  /// * Other characters to the right get pulled 1 cell backward.
  /// * The last character of the line is now blank.
  pub fn delete_ch(&mut self) -> Result<(), ()> {
    unsafe_call_result!(wdelch(self.ptr))
  }

  /// Copies the slice of glyphs starting from the cursor position.
  ///
  /// * Does not advance the cursor.
  /// * Does not wrap the content to the next line.
  pub fn copy_glyphs(&mut self, s: &[CursesGlyph]) -> Result<(), ()> {
    unsafe_call_result!(waddchnstr(
      self.ptr,
      s.as_ptr().cast(),
      s.len().try_into().unwrap()
    ))
  }

  /// Clears the entire screen and moves the cursor to `(0,0)`.
  pub fn clear(&mut self) -> Result<(), ()> {
    unsafe_call_result!(wclear(self.ptr))
  }

  /// Set the given attribute bits to be on or off.
  pub fn set_attributes(
    &mut self, attr: Attributes, on: bool,
  ) -> Result<(), ()> {
    let attr: i32 = ((attr.0 as u32) << 16) as i32;
    if on {
      unsafe_call_result!(wattron(self.ptr, attr))
    } else {
      unsafe_call_result!(wattroff(self.ptr, attr))
    }
  }

  /// Assigns the timeout to use with [`poll_events`](Curses::poll_events).
  ///
  /// * Negative: infinite time, `poll_events` is blocking.
  /// * Zero: No timeout, `poll_events` returns `None` immediately if no input
  ///   is ready.
  /// * Positive: wait up to this many milliseconds before returning `None`.
  pub fn set_timeout(&mut self, time: i32) {
    unsafe_void!(wtimeout(self.ptr, time))
  }

  /// Gets an input event.
  ///
  /// * Ascii keys are returned as their ascii value.
  /// * Keypad keys are values >= 256.
  /// * If the terminal is resized, that shows up as a special key in this
  ///   queue.
  /// * If you have a timeout set and the time expires, you get `None` back.
  pub fn poll_events(&mut self) -> Option<CursesKey> {
    const ERR_U32: u32 = ERR as u32;
    const KEY_F64: u32 = KEY_F0 + 64;
    match (unsafe { wgetch(self.ptr) }) as u32 {
      ERR_U32 => None,
      ascii if (ascii <= u8::MAX as u32) => Some(CursesKey::Ascii(ascii as u8)),
      #[cfg(windows)]
      KEY_A1 => Some(CursesKey::Home),
      #[cfg(windows)]
      KEY_A2 => Some(CursesKey::ArrowUp),
      #[cfg(windows)]
      KEY_A3 => Some(CursesKey::PageUp),
      #[cfg(windows)]
      KEY_B1 => Some(CursesKey::ArrowLeft),
      #[cfg(windows)]
      KEY_B3 => Some(CursesKey::ArrowRight),
      #[cfg(windows)]
      KEY_C1 => Some(CursesKey::End),
      #[cfg(windows)]
      KEY_C2 => Some(CursesKey::ArrowDown),
      #[cfg(windows)]
      KEY_C3 => Some(CursesKey::PageDown),
      #[cfg(windows)]
      PADENTER => Some(CursesKey::Enter),
      #[cfg(windows)]
      PADSLASH => Some(CursesKey::Ascii(b'/')),
      #[cfg(windows)]
      PADSTAR => Some(CursesKey::Ascii(b'*')),
      #[cfg(windows)]
      PADMINUS => Some(CursesKey::Ascii(b'-')),
      #[cfg(windows)]
      PADPLUS => Some(CursesKey::Ascii(b'+')),
      //
      KEY_BACKSPACE => Some(CursesKey::Backspace),
      KEY_UP => Some(CursesKey::ArrowUp),
      KEY_DOWN => Some(CursesKey::ArrowDown),
      KEY_LEFT => Some(CursesKey::ArrowLeft),
      KEY_RIGHT => Some(CursesKey::ArrowRight),
      KEY_IC => Some(CursesKey::Insert),
      KEY_DC => Some(CursesKey::Delete),
      KEY_HOME => Some(CursesKey::Home),
      KEY_END => Some(CursesKey::End),
      KEY_PPAGE => Some(CursesKey::PageUp),
      KEY_NPAGE => Some(CursesKey::PageDown),
      KEY_B2 => Some(CursesKey::Keypad5NoNumlock),
      KEY_RESIZE => Some(CursesKey::TerminalResized),
      KEY_ENTER => Some(CursesKey::Enter),
      //
      f if (f >= KEY_F0 && f <= KEY_F64) => {
        Some(CursesKey::Function((f - KEY_F0) as u8))
      }
      other => Some(CursesKey::UnknownKey(other)),
    }
  }

  /// Pushes this event to the front of the event queue so that the next
  /// `poll_events` returns this value.
  pub fn un_get_event(&mut self, event: Option<CursesKey>) -> Result<(), ()> {
    let ev: u32 = match event {
      None => ERR as u32,
      Some(CursesKey::Ascii(ascii)) => ascii as u32,
      Some(CursesKey::Function(f)) => KEY_F0 + (f as u32),
      Some(CursesKey::Enter) => KEY_ENTER,
      Some(CursesKey::Backspace) => KEY_BACKSPACE,
      Some(CursesKey::ArrowUp) => KEY_UP,
      Some(CursesKey::ArrowDown) => KEY_DOWN,
      Some(CursesKey::ArrowLeft) => KEY_LEFT,
      Some(CursesKey::ArrowRight) => KEY_RIGHT,
      Some(CursesKey::Insert) => KEY_IC,
      Some(CursesKey::Delete) => KEY_DC,
      Some(CursesKey::Home) => KEY_HOME,
      Some(CursesKey::End) => KEY_END,
      Some(CursesKey::PageUp) => KEY_PPAGE,
      Some(CursesKey::PageDown) => KEY_NPAGE,
      Some(CursesKey::Keypad5NoNumlock) => KEY_B2,
      Some(CursesKey::TerminalResized) => KEY_RESIZE,
      Some(CursesKey::UnknownKey(u)) => u,
    };
    unsafe_call_result!(ungetch(ev as i32))
  }

  /// Flushes all pending key events.
  pub fn flush_events(&mut self) -> Result<(), ()> {
    unsafe_call_result!(flushinp())
  }

  /// Return the terminal to shell mode temporarily.
  pub fn shell_mode<'a>(&'a mut self) -> Result<CursesShell<'a>, ()> {
    unsafe_always_ok!(def_prog_mode());
    unsafe_call_result!(endwin()).map(move |_| CursesShell { win: self })
  }

  /// If the terminal supports colors at all.
  pub fn has_color(&self) -> bool {
    unsafe { has_colors() }
  }

  /// If the terminal is able to change the RGB values of a given [`ColorID`]
  pub fn can_change_colors(&self) -> bool {
    unsafe { can_change_color() }
  }

  /// Gets the highest allowed color id for this terminal.
  pub fn get_max_color_id_inclusive(&self) -> Option<ColorID> {
    let colors = unsafe { COLORS };
    if colors > 0 {
      Some(ColorID(unsafe { (COLORS - 1).try_into().unwrap_or(u8::MAX) }))
    } else {
      None
    }
  }

  /// Gets the highest allowed color pair for this terminal.
  pub fn get_max_color_pair_inclusive(&self) -> Option<ColorPair> {
    NonZeroU8::new(unsafe { (COLOR_PAIRS - 1).try_into().unwrap_or(u8::MAX) })
      .map(ColorPair)
  }

  /// Sets the color id to use the RGB values given, or closest approximation
  /// available.
  ///
  /// Inputs are clamped to the range `0.0 ..= 1.0`
  pub fn set_color_id_rgb(
    &mut self, c: ColorID, [r, g, b]: [f32; 3],
  ) -> Result<(), ()> {
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
    unsafe_call_result!(color_content(
      c.0.into(),
      &mut r_i16,
      &mut g_i16,
      &mut b_i16
    ))
    .map(|_| {
      let r = r_i16 as f32 / 1000.0;
      let g = g_i16 as f32 / 1000.0;
      let b = b_i16 as f32 / 1000.0;
      [r, g, b]
    })
  }

  /// Assigns the selected color pair to use the foreground and background
  /// specified.
  ///
  /// A character cell is associated to a given color pair, so changing any
  /// color pair will immediately change all character cells displaying the
  /// color pair.
  pub fn set_color_pair_content(
    &mut self, pair: ColorPair, fg: ColorID, bg: ColorID,
  ) -> Result<(), ()> {
    unsafe_call_result!(init_pair(
      pair.0.get().into(),
      fg.0.into(),
      bg.0.into()
    ))
  }

  /// Gets the RGB values of the given color id.
  pub fn get_color_pair_content(
    &self, c: ColorID,
  ) -> Result<(ColorID, ColorID), ()> {
    let mut f_i16 = 0;
    let mut b_i16 = 0;
    unsafe_call_result!(pair_content(c.0.into(), &mut f_i16, &mut b_i16))
      .and_then(|_| match (u8::try_from(f_i16), u8::try_from(b_i16)) {
        (Ok(f), Ok(b)) => Ok((ColorID(f), ColorID(b))),
        _ => Err(()),
      })
  }

  /// Sets the default coloring for all newly printed glyphs.
  pub fn set_active_color_pair(
    &mut self, opt_pair: Option<ColorPair>,
  ) -> Result<(), ()> {
    let p = opt_pair.map(|cp| cp.0.get()).unwrap_or(0).into();
    unsafe_call_result!(wcolor_set(self.ptr, p, core::ptr::null_mut()))
  }

  /// Set if the window can be scrolled or not.
  ///
  /// * Off by default.
  pub fn set_scrollable(&mut self, yes: bool) -> Result<(), ()> {
    unsafe_call_result!(scrollok(self.ptr, yes))
  }

  /// Sets the top line and bottom line that mark the edges of the scrollable
  /// region.
  ///
  /// Lines `0..=top` and `bottom..` will stay static when the window is
  /// scrolled. All other lines will move 1 row upward.
  ///
  /// * By default the scroll region is the entire terminal.
  pub fn set_scroll_region(&mut self, top: u32, bottom: u32) -> Result<(), ()> {
    unsafe_call_result!(wsetscrreg(self.ptr, top as i32, bottom as i32))
  }

  /// Scrolls the window by the given number of lines.
  ///
  /// * Negative: text moves down the page.
  /// * Positive: text moves up the page.
  /// * Zero: text doesn't move.
  pub fn scroll(&mut self, n: i32) -> Result<(), ()> {
    unsafe_call_result!(wscrl(self.ptr, n))
  }

  /// Sets the cursor visibility.
  ///
  /// Returns the old visibility, or Err if it can't be set.
  pub fn set_cursor_visibility(
    &mut self, vis: CursorVisibility,
  ) -> Result<CursorVisibility, ()> {
    let old = unsafe { curs_set(vis as i32) };
    Ok(match old {
      0 => CursorVisibility::Invisible,
      1 => CursorVisibility::Normal,
      2 => CursorVisibility::VeryVisible,
      _ => return Err(()),
    })
  }

  /// Sets the background glyph.
  pub fn set_background<C: Into<CursesGlyph>>(
    &mut self, c: C,
  ) -> Result<(), ()> {
    unsafe_call_result!(wbkgd(self.ptr, c.into().as_chtype()))
  }

  /// Gets the background glyph.
  pub fn get_background(&self) -> CursesGlyph {
    unsafe { core::mem::transmute(getbkgd(self.ptr)) }
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
/// A struct because it's 2-dimensional instead of just a single length or
/// whatever.
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

/// Use with [`set_cursor_visibility`](Curses::set_cursor_visibility)
#[repr(i32)]
pub enum CursorVisibility {
  /// Cursor is invisible.
  Invisible = 0,
  /// Cursor is normal.
  Normal = 1,
  /// Cursor is extra visible (not always supported).
  VeryVisible = 2,
}

/// Names a color within curses.
///
/// This is **not** an actual RGB color value. It's just an index into a color
/// palette.
/// * Assuming that a terminal supports color at all, you usually get at least 8
///   palette slots.
/// * The linux console generally has only 8 colors, but terminal emulators
///   often have far more.
///
/// This type has some associated constants.
/// Each constant names the id value that is most likely to display as that
/// color by default.
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
/// Each color pair maps to a foreground color and background color id palette
/// value. The pair id is a single byte, but 0 is a special "no color" value.
/// It uses whatever the terminal's default colors were before curses was
/// initialized. Accordingly, we model the color pair in Rust as a NonZeroU8,
/// and wrap it in an Option as appropriate.
///
/// You can generally change what color ids that are associated with a color
/// pair id. The number of color pairs actually available varies by terminal,
/// though generally at least 64 are supported.
///
/// If you change the colors of a color pair, all character cells on the screen
/// using that pairing will have their displayed colors immediately changed
/// accordingly.
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
  pub const STANDOUT: Attributes = if cfg!(unix) {
    Attributes(1 << 0)
  } else {
    Attributes(Attributes::REVERSE.0 | Attributes::BOLD.0)
  };

  /// The text should be underlined.
  ///
  /// * Linux Console: No.
  /// * Terminal Emulator: Maybe.
  pub const UNDERLINE: Attributes = if cfg!(unix) {
    Attributes(1 << 1)
  } else {
    Attributes((0x00100000 >> 16) as u16)
  };

  /// The text should have foreground and background colors reversed.
  ///
  /// * Works basically everywhere color does.
  pub const REVERSE: Attributes = if cfg!(unix) {
    Attributes(1 << 2)
  } else {
    Attributes((0x00200000 >> 16) as u16)
  };

  /// The text should blink.
  ///
  /// * Linux Console: Visually distinct, but doesn't actually blink.
  /// * Terminal Emulator: Probably actually blinks.
  pub const BLINK: Attributes = if cfg!(unix) {
    Attributes(1 << 3)
  } else {
    Attributes((0x00400000 >> 16) as u16)
  };

  /// The text should be dim.
  ///
  /// * Linux Console: No.
  /// * Terminal Emulator: Maybe.
  pub const DIM: Attributes =
    if cfg!(unix) { Attributes(1 << 4) } else { Attributes(0) };

  /// The text should be bold.
  ///
  /// * Works basically everywhere color does.
  pub const BOLD: Attributes = if cfg!(unix) {
    Attributes(1 << 5)
  } else {
    Attributes((0x00800000 >> 16) as u16)
  };

  /// The text should use the alternative character set.
  ///
  /// * This always "works", in that you'll see the alternate character for the
  ///   given byte, but what actually displays is up to the terminal. Each ACS
  ///   character is named after the *intended* appearance, at least.
  pub const ALT_CHAR_SET: Attributes = if cfg!(unix) {
    Attributes(1 << 6)
  } else {
    Attributes((0x00010000 >> 16) as u16)
  };

  /// The text should be invisible (foreground and background the same).
  ///
  /// * Linux Console: No.
  /// * Terminal Emulator: Maybe.
  pub const INVIS: Attributes =
    if cfg!(unix) { Attributes(1 << 7) } else { Attributes(0) };

  /// The text should be italic.
  ///
  /// * Linux Console: No.
  /// * Terminal Emulator: Maybe.
  pub const ITALIC: Attributes = if cfg!(unix) {
    Attributes(1 << 15)
  } else {
    Attributes((0x00080000 >> 16) as u16)
  };

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

/// The types of input keys that `ncurses` can generate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursesKey {
  /// An ascii input (most all the keys with symbols on them).
  Ascii(u8),
  /// The terminal was resized.
  TerminalResized,
  /// Enter key
  Enter,
  /// Backspace key
  Backspace,
  /// Arrow upward (arrow key or numpad without numlock)
  ArrowUp,
  /// Arrow downward (arrow key or numpad without numlock)
  ArrowDown,
  /// Arrow left (arrow key or numpad without numlock)
  ArrowLeft,
  /// Arrow right (arrow key or numpad without numlock)
  ArrowRight,
  /// Insert key
  Insert,
  /// Delete key
  Delete,
  /// Home key (or numpad 7 without numlock on)
  Home,
  /// End key (or numpad 1 without numlock on)
  End,
  /// Page up / Previous Page  (or numpad 9 without numlock on)
  PageUp,
  /// Page down / Next Page (or numpad 3 without numlock on)
  PageDown,
  /// The middle key of the numpad if numlock isn't on.
  Keypad5NoNumlock,
  /// A function key (F1, F2, etc.).
  ///
  /// These aren't the best supported because the terminal emulator often eat
  /// them before the program sees it.
  Function(u8),
  /// Some unknown input value.
  ///
  /// You might want to file an issue to get this value included.
  UnknownKey(u32),
}
impl CursesKey {
  /// Convert a byte into a `CursesKey::Ascii(byte)`
  pub const fn from_ascii(ascii: u8) -> Self {
    CursesKey::Ascii(ascii)
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

macro_rules! acs_getter {
  ($fn_name:ident, $ch:expr, $d:expr) => {
    #[doc = $d]
    #[cfg(unix)]
    pub fn $fn_name(&self) -> CursesGlyph {
      let c: char = $ch;
      CursesGlyph {
        ascii: unsafe { (*acs_map.as_ptr().add(c as u8 as usize)) as u8 },
        opt_color_pair: None,
        attributes: Attributes::ALT_CHAR_SET,
      }
    }

    #[doc = $d]
    #[cfg(windows)]
    pub fn $fn_name(&self) -> CursesGlyph {
      let c: char = $ch;
      CursesGlyph {
        ascii: c as u8,
        opt_color_pair: None,
        attributes: Attributes::ALT_CHAR_SET,
      }
    }
  }
}

impl Curses {
  acs_getter!(acs_block, '0', "Solid square block, but sometimes a hash.");
  acs_getter!(acs_board, 'h', "Board of squares, often just a hash.");
  acs_getter!(acs_btee, 'v', "Bottom T");
  acs_getter!(acs_bullet, '~', "Bullet point");
  acs_getter!(acs_ckboard, 'a', "Checkerboard, usually like a 50% stipple");
  acs_getter!(acs_darrow, '.', "Down arrow");
  acs_getter!(acs_degree, 'f', "Degree symbol (like with an angle)");
  acs_getter!(acs_diamond, '`', "Diamond");
  acs_getter!(acs_gequal, 'z', "Greater-than or equal to.");
  acs_getter!(acs_hline, 'q', "Horizontal line");
  acs_getter!(acs_lantern, 'i', "Lantern symbol");
  acs_getter!(acs_larrow, ',', "Left arrow");
  acs_getter!(acs_lequal, 'y', "Less-than or equal to.");
  acs_getter!(acs_llcorner, 'm', "Lower left corner of a box.");
  acs_getter!(acs_lrcorner, 'j', "Lower right corner of a box.");
  acs_getter!(acs_ltee, 't', "Left T");
  acs_getter!(acs_nequal, '|', "Not-equal to.");
  acs_getter!(acs_pi, '{', "Pi");
  acs_getter!(acs_plminus, 'g', "Plus/Minus");
  acs_getter!(acs_plus, 'n', "Plus shaped \"line\" in all four directions");
  acs_getter!(acs_rarrow, '+', "Right arrow");
  acs_getter!(acs_rtee, 'u', "Right T");
  acs_getter!(acs_s1, 'o', "Horizontal Scanline 1");
  acs_getter!(acs_s3, 'p', "Horizontal Scanline 3");
  acs_getter!(acs_s7, 'r', "Horizontal Scanline 7");
  acs_getter!(acs_s9, 's', "Horizontal Scanline 9");
  acs_getter!(acs_sterling, '}', "British pounds sterling.");
  acs_getter!(acs_ttee, 'w', "Top T");
  acs_getter!(acs_uarrow, '-', "Up arrow");
  acs_getter!(acs_ulcorner, 'l', "Upper left corner of a box.");
  acs_getter!(acs_urcorner, 'k', "Upper right corner of a box.");
  acs_getter!(acs_vline, 'x', "Vertical line");
}
