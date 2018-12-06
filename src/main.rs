// extern crate regex;

// use regex::Regex;
use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
  day_1_a();
  day_1_b();
  day_2_a();
  day_2_b();
}

fn read_file_to_string(name: &str) -> String {
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

fn day_1_a() {
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

fn day_1_b() {
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

fn day_2_a() {
  let s = read_file_to_string("input2");

  let mut twos = 0;
  let mut threes = 0;

  let split = s.split_whitespace();
  for mut line in split {
    let mut two = false;
    let mut three = false;

    for c in line.chars() {
      let mut counter: Vec<&str> = line.matches(c).collect();
      // println!("{:?}", &counter);
      match counter.len() {
        2 => {
          two = true;
        }
        3 => {
          three = true;
        }
        _ => {}
      }
    }
    if two {
      twos = twos + 1;
    }
    if three {
      threes = threes + 1;
    }
  }
  println!("2A: {}", twos * threes);
}

fn day_2_b() {
  let s = read_file_to_string("input2");
  let split = s.split_whitespace();
  let mut copied: Vec<&str> = Vec::new();

  for line in split {
    for line2 in &copied {
      if only_one_difference(line, line2) {
        println!("2B: {}, {}", line, line2);
        return;
      }
    }

    copied.push(line);
  }

  println!("2B: Not found");
}

fn only_one_difference(s1: &str, s2: &str) -> bool {
  let c1: Vec<char> = s1.chars().collect();
  let c2: Vec<char> = s2.chars().collect();
  let length = cmp::min(c1.len(), c2.len());
  let mut diff = 0;

  for mut i in 0..length {
    if c1[i] != c2[i] {
      diff = diff + 1;
    }
    if diff > 1 {
      return false;
    }
  }

  return diff == 1;
}
