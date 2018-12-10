use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn a() {
  let path = Path::new("input1");

  let mut file = match File::open(&path) {
    Err(why) => panic!(why),
    Ok(file) => file,
  };

  let mut s = String::new();

  match file.read_to_string(&mut s) {
    Err(why) => panic!(why),
    Ok(_) => {
      let mut result: i32 = 0;
      let split = s.split_whitespace();
      for mut line in split {
        // println!("{}", line);
        result += match i32::from_str_radix(line, 10) {
          Ok(x) => x,
          Err(_e) => 0,
        };

        // println!("Total: {}", result);
      }
      println!("1A: {}", result);
    }
  }
}

pub fn b() {
  let path = Path::new("input1");

  let mut file = match File::open(&path) {
    Err(why) => panic!(why),
    Ok(file) => file,
  };

  let mut s = String::new();

  match file.read_to_string(&mut s) {
    Err(why) => panic!(why),
    Ok(_) => {
      let mut result: i32 = 0;
      let mut steps: HashSet<i32> = HashSet::new();
      let mut finished = false;
      while !finished {
        let split = s.split_whitespace();
        for mut line in split {
          // println!("{}", line);
          result += match i32::from_str_radix(line, 10) {
            Ok(x) => x,
            Err(_e) => 0,
          };
          if steps.contains(&result) {
            println!("1B: {}", result);
            finished = true;
            break;
          } else {
            steps.insert(result);
          }
        }
      }
    }
  }
}
