#[cfg(not(windows))]
fn main() {
  // pass
}

#[cfg(windows)]
fn main() {
  let mut build = cc::Build::new();

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

  println!("cargo:rustc-link-lib=dylib=user32");

  build.compile("libpdcurses.a");
}
