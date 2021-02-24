#[cfg(unix)]
fn main() {
  println!("cargo:rustc-link-lib=dylib=ncurses");
}

#[cfg(windows)]
fn main() {
  let mut build = cc::Build::new();

  // This puts all the `*.c` files we can find into the build.

  for res_dir_entry in std::fs::read_dir("pdcurses_win32").unwrap() {
    if let Ok(dir_entry) = res_dir_entry {
      if let Ok(file_type) = dir_entry.file_type() {
        if file_type.is_file() {
          let path = dir_entry.path();
          if let Some(os_str_ref) = path.extension() {
            if os_str_ref.to_str() == Some("c") {
              build.file(path);
            }
          }
        }
      }
    }
  }

  build
    .include("pdcurses_win32")
    .define("PDC_WIDE", Some("Y"))
    .define("PDC_FORCE_UTF8", Some("Y"))
    .define("PDC_RGB", Some("Y"));

  build.compile("pdcurses");

  println!("cargo:rustc-link-lib=dylib=pdcurses");
  println!("cargo:rustc-link-lib=dylib=user32");
}

#[cfg(not(any(unix, windows)))]
fn main() {
  panic!("yacurses only knows how to build for unix and windows.");
}
