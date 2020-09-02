use yacurses::*;

use std::panic::{catch_unwind, AssertUnwindSafe};

#[allow(unused)]
fn main() {
  // This will rust-panic on double-init, and on failure in the C layer the C
  // code will "helpfully" print the error message and abort the process.
  // Otherwise, this "always succeeds".
  let mut win = Curses::init();

  // If you panic normally while in curses mode, the panic message "prints" (and
  // is eaten by curses mode) *before* the unwinding that ends curses mode.
  // Accordingly, we must use `catch_unwind` to catch the panic instead of
  // printing it immediately. Because we're transferring the `&mut Curses`
  // across the panic bound, we put `AssertUnwindSafe` over our closure or rustc
  // gets jumpy about things possibly going wrong.
  let catch_result = catch_unwind(AssertUnwindSafe(|| {
    win.set_echo(false);
    win.set_cursor_visibility(CursorVisibility::Invisible);
    if win.can_change_colors() {
      win.set_color_id_rgb(ColorID::WHITE, [1.0, 1.0, 1.0]).unwrap();
    } else {
      win.print_str("This terminal cannot change colors :(");
    }
    win.move_cursor(Position { x: 75, y: 1 });
    let ascii = b'@';
    let opt_color_pair = None;
    win.print_ch(CursesGlyph {
      ascii,
      opt_color_pair,
      attributes: Attributes(0),
    });
    for n in 0..16 {
      let attributes = Attributes(1 << n);
      win.print_ch(CursesGlyph { ascii, opt_color_pair, attributes });
    }
    win.move_cursor(Position { x: 75, y: 5 });
    win.print_str("Hello there, General Kenobi!");
    win.poll_events().unwrap();
    //
    win.set_background('!');
    win.clear();
    win.print_str("ACS:");
    for ch in [
      win.acs_sterling(),
      win.acs_degree(),
      win.acs_plminus(),
      win.acs_bullet(),
      win.acs_pi(),
      win.acs_lequal(),
      win.acs_gequal(),
      win.acs_hline(),
      win.acs_vline(),
      win.acs_ulcorner(),
      win.acs_urcorner(),
      win.acs_llcorner(),
      win.acs_lrcorner(),
      win.acs_ltee(),
      win.acs_rtee(),
      win.acs_ttee(),
      win.acs_btee(),
      win.acs_plus(),
      win.acs_ckboard(),
    ]
    .iter()
    .copied()
    {
      win.print_ch(ch);
    }
    win.move_cursor(Position { x: 75, y: 8 });
    win.copy_glyphs(&[CursesGlyph::from(ascii); 10]);
    const Q: CursesKey = CursesKey::from_ascii(b'q');
    const P: CursesKey = CursesKey::from_ascii(b'p');
    loop {
      match win.poll_events() {
        Some(Q) => break,
        Some(P) => {
          let sh = win.shell_mode().unwrap();
          for cid in 0..8 {
            let [r, g, b] = sh.get_color_id_rgb(ColorID(cid)).unwrap();
            eprintln!("CID({}): [{},{},{}]", cid, r, g, b);
          }
          eprintln!("{:?}", sh.get_cursor_position());
          eprintln!("{:?}", sh.get_terminal_size());
          eprintln!("{:?}", sh.get_max_color_id_inclusive());
          eprintln!("{:?}", sh.get_max_color_pair_inclusive());
          let mut str_buf = String::with_capacity(1024);
          std::io::stdin().read_line(&mut str_buf).unwrap();
          println!("got line: {}", str_buf);
        }
        Some(CursesKey::UnknownKey(u)) => {
          let sh = win.shell_mode().unwrap();
          panic!("Unknown Key: {}", u);
        }
        _ => continue,
      }
    }
    win.poll_events().unwrap();
  }));

  // After we run `catch_unwind`, if we had a panic we change the window to
  // shell mode (which causes stdout/stderr to work normally) and then we
  // continue the panic with whatever the cause from before was.
  if let Err(cause) = catch_result {
    let sh = win.shell_mode().unwrap();
    panic!(cause)
  }
}
