[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.48-green.svg)
[![crates.io](https://img.shields.io/crates/v/yacurses.svg)](https://crates.io/crates/yacurses)
[![docs.rs](https://docs.rs/yacurses/badge.svg)](https://docs.rs/yacurses/)

# yacurses

yet another curses lib

## Examples

### Basic

```rust
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
```

### Some window attributes

```rust
use yacurses::*;

fn main() {
  // Start curses mode
  let mut win = Curses::init();

  // Prevent user inuput from printing to the screen
  win.set_echo(false);

  // Hide cursor
  win.set_cursor_visibility(CursorVisibility::Invisible);

  // Set the background glyph
  win.set_background('+');
}
```


### Character attributes

Attributes effects are OS dependent and  may not work on some terminals. More details [here](https://docs.rs/yacurses/latest/yacurses/struct.Attributes.html). Attributes can be combined.

```rust
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
```


### Events

The `poll_events()` is blocking.

```rust
use yacurses::*;

fn main() {
  // Start curses mode
  let mut win = Curses::init();

  loop {
    match win.poll_events() {
       // Key P is pressed
       Some(CursesKey::from_ascii(b'p')) => break,
       Some(CursesKey::Enter) => {
         // handle event...
       }
       _ => continue,
    }
  }
}
```

### Special characters

|      Function     | Character |
|:------------------|:---------:|
| `acs_block()`     |    `#`    |
| `acs_board()`     |    `#`    |
| `acs_btee()`      |    `┴`    |
| `acs_bullet()`    |    `·`    |
| `acs_ckboard()`   |    `▒`    |
| `acs_darrow()`    |    `v`    |
| `acs_degree()`    |    `°`    |
| `acs_diamond()`   |    `◆`    |
| `acs_gequal()`    |    `≥`    |
| `acs_hline()`     |    `─`    |
| `acs_lantern()`   |    `␋`    |
| `acs_larrow()`    |    `<`    |
| `acs_lequal()`    |    `≤`    |
| `acs_llcorner()`  |    `└`    |
| `acs_lrcorner()`  |    `┘`    |
| `acs_ltee()`      |    `├`    |
| `acs_nequal()`    |    `≠`    |
| `acs_pi()`        |    `π`    |
| `acs_plminus()`   |    `±`    |
| `acs_plus()`      |    `┼`    |
| `acs_rarrow()`    |    `>`    |
| `acs_rtee()`      |    `┤`    |
| `acs_s1()`        |    `⎺`    |
| `acs_s3()`        |    `⎻`    |
| `acs_s7()`        |    `⎼`    |
| `acs_s9()`        |    `⎽`    |
| `acs_sterling()`  |    `£`    |
| `acs_ttee()`      |    `┬`    |
| `acs_uarrow()`    |    `^`    |
| `acs_ulcorner()`  |    `┌`    |
| `acs_urcorner()`  |    `┐`    |
| `acs_vline()`     |    `│`    |
