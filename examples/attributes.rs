use yacurses::*;

fn main() {
  // Start curses mode
  let mut win = Curses::init();

  win.set_attributes(Attributes::UNDERLINE, true).unwrap();
  win.print_str("Underlined text\n").unwrap();
  win.set_attributes(Attributes::UNDERLINE, false).unwrap();

  win.set_attributes(Attributes::REVERSE, true).unwrap();
  win.print_str("Color inversed\n").unwrap(); // Same as STANDOUT attribute
  win.set_attributes(Attributes::REVERSE, false).unwrap();

  win.set_attributes(Attributes::BLINK, true).unwrap();
  win.print_str("Blinking text\n").unwrap();
  win.set_attributes(Attributes::BLINK, false).unwrap();

  win.set_attributes(Attributes::BOLD, true).unwrap();
  win.print_str("Bold text\n").unwrap();
  win.set_attributes(Attributes::BOLD, false).unwrap();

  win.set_attributes(Attributes::INVIS, true).unwrap();
  win.print_str("Invisible text\n").unwrap();
  win.set_attributes(Attributes::INVIS, false).unwrap();

  win.set_attributes(Attributes::ITALIC, true).unwrap();
  win.print_str("Italic text\n").unwrap();
  win.set_attributes(Attributes::ITALIC, false).unwrap();

  win.refresh().unwrap();

  win.poll_events().unwrap();
}
