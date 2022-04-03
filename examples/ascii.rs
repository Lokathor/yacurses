use yacurses::*;

fn main() {
  // Start curses mode
  let mut win = Curses::init();

  win.print_ch(win.acs_block()).unwrap();
  win.print_ch(win.acs_board()).unwrap();
  win.print_ch(win.acs_btee()).unwrap();
  win.print_ch(win.acs_bullet()).unwrap();
  win.print_ch(win.acs_ckboard()).unwrap();
  win.print_ch(win.acs_darrow()).unwrap();
  win.print_ch(win.acs_degree()).unwrap();
  win.print_ch(win.acs_diamond()).unwrap();
  win.print_ch(win.acs_gequal()).unwrap();
  win.print_ch(win.acs_hline()).unwrap();
  win.print_ch(win.acs_lantern()).unwrap();
  win.print_ch(win.acs_larrow()).unwrap();
  win.print_ch(win.acs_lequal()).unwrap();
  win.print_ch(win.acs_llcorner()).unwrap();
  win.print_ch(win.acs_lrcorner()).unwrap();
  win.print_ch(win.acs_ltee()).unwrap();
  win.print_ch(win.acs_nequal()).unwrap();
  win.print_ch(win.acs_pi()).unwrap();
  win.print_ch(win.acs_plminus()).unwrap();
  win.print_ch(win.acs_plus()).unwrap();
  win.print_ch(win.acs_rarrow()).unwrap();
  win.print_ch(win.acs_rtee()).unwrap();
  win.print_ch(win.acs_s1()).unwrap();
  win.print_ch(win.acs_s3()).unwrap();
  win.print_ch(win.acs_s7()).unwrap();
  win.print_ch(win.acs_s9()).unwrap();
  win.print_ch(win.acs_sterling()).unwrap();
  win.print_ch(win.acs_ttee()).unwrap();
  win.print_ch(win.acs_uarrow()).unwrap();
  win.print_ch(win.acs_ulcorner()).unwrap();
  win.print_ch(win.acs_urcorner()).unwrap();
  win.print_ch(win.acs_vline()).unwrap();

  win.poll_events().unwrap();
}
