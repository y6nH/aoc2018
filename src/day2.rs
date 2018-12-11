use std::cmp;
use crate::util;

pub fn a() {
  let s = util::read_file_to_string("input2");

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

pub fn b() {
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
