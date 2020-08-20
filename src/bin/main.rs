use yacurses::*;

#[allow(unused)]
fn main() {
  let mut win = Curses::init();
  win.set_echo(false);
  win.set_color_id_rgb(ColorID::WHITE, [1.0, 1.0, 1.0]).unwrap();
  win.move_cursor(Position { x: 0, y: 1 });
  let ascii = b'@';
  let opt_color_pair = None;
    win.print_ch(CursesGlyph { ascii, opt_color_pair, attributes: Attributes(0) });
  for n in 0 .. 16 {
    let attributes = Attributes::BOLD;
    win.print_ch(CursesGlyph { ascii, opt_color_pair, attributes });
  }
  const Q: u32 = b'q' as u32;
  const P: u32 = b'p' as u32;
  loop {
    match win.poll_events() {
      Some(Q) => break,
      Some(P) => {
        let sh = win.shell_mode().unwrap();
        for cid in 0 .. 8 {
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
      _ => continue,
    }
  }
  /*
  win.add_ch(acs_sterling());
  win.add_ch(acs_degree());
  win.add_ch(acs_plminus());
  win.add_ch(acs_bullet());
  win.add_ch(acs_pi());
  win.add_ch(acs_lequal());
  win.add_ch(acs_gequal());
  win.add_ch(acs_hline());
  win.add_ch(acs_vline());
  win.add_ch(acs_ulcorner());
  win.add_ch(acs_urcorner());
  win.add_ch(acs_llcorner());
  win.add_ch(acs_lrcorner());
  win.add_ch(acs_ltee());
  win.add_ch(acs_rtee());
  win.add_ch(acs_ttee());
  win.add_ch(acs_btee());
  win.add_ch(acs_plus());
  win.add_ch(acs_ckboard());
  */
}
