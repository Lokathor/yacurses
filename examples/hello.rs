use yacurses::*;

fn main() {
  // Start curses mode
  let mut win = Curses::init();

  // Move and print
  win.move_cursor(Position { x: 3, y: 2 }).unwrap();
  win.print_str("Hello world!").unwrap();

  // Update screen
  win.refresh().unwrap();

  // Waits for events
  win.poll_events();
}
