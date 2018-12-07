use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_file_to_string(name: &str) -> String {
  let path = Path::new(name);

  let mut file = match File::open(&path) {
    Err(why) => panic!(why),
    Ok(file) => file,
  };

  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Err(why) => panic!(why),
    Ok(_) => return s,
  }
}