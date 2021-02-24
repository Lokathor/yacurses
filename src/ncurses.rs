#![allow(bad_style)]

//! ncurses-specific declarations.

use crate::curses_common::chtype;

pub const KEY_B2: u32 = 350;
pub const KEY_END: u32 = 360;

// Note(Lokathor): READ ONLY!
extern "C" {
  /// The Alternate Character Set mappings.
  ///
  /// This isn't filled in until *after* curses has been initialized.
  pub static mut acs_map: [chtype; 0usize];
}
