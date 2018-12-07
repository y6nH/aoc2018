extern crate regex;

use regex::Regex;
use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod util;

fn main() {
  day_1_a();
  day_1_b();
  day_2_a();
  day_2_b();
  day_3_a();
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
      for line in split {
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
        for line in split {
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
  let s = util::read_file_to_string("input2");

  let mut twos = 0;
  let mut threes = 0;

  let split = s.split_whitespace();
  for line in split {
    let mut two = false;
    let mut three = false;

    for c in line.chars() {
      let counter: Vec<&str> = line.matches(c).collect();
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
  let s = util::read_file_to_string("input2");
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

  for i in 0..length {
    if c1[i] != c2[i] {
      diff = diff + 1;
    }
    if diff > 1 {
      return false;
    }
  }

  return diff == 1;
}

/*
A claim like #123 @ 3,2: 5x4 means that claim ID 123 specifies a rectangle 3 inches from the left edge, 
2 inches from the top edge, 5 inches wide, and 4 inches tall. 
How many square inches of fabric are within two or more claims?
*/
fn day_3_a() {
  let s = util::read_file_to_string("input3");
  let split = s.split("\n");
  let reg =
    Regex::new("#(?P<id>\\d+) @ (?P<x>\\d+),(?P<y>\\d+): (?P<w>\\d+)x(?P<h>\\d+)$").unwrap();
  for line in split {
    let claim = reg.captures(&line);
    match claim {
      Some(c) => {
        let id = &c["id"];
        let x = &c["x"];
        let y = &c["y"];
        let w = &c["w"];
        let h = &c["h"];

        println!("{} {} {} {} {}", id, x, y, w, h);

        // TODO:
        // Convert strings to usize
        // Find and list all claimed squares
        // If claimed a second time, add it to a hash set
        // Count the hash set
      }
      None => {
        println!("Non-matching line: {}", line);
      }
    }
  }
}
