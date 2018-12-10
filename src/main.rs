extern crate chrono;
extern crate regex;

use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod day1;
mod day2;
mod day3;
mod day4;
mod util;

fn main() {
  day1::a();
  day1::b();
  day2::a();
  day2::b();
  day3::a();
  day4::a();
}


/*
A claim like #123 @ 3,2: 5x4 means that claim ID 123 specifies a rectangle 3 inches from the left edge, 
2 inches from the top edge, 5 inches wide, and 4 inches tall. 
How many square inches of fabric are within two or more claims?
*/
fn day_3() {
  let s = util::read_file_to_string("input3");
  let split = s.split("\n");
  let reg =
    Regex::new("#(?P<id>\\d+) @ (?P<x>\\d+),(?P<y>\\d+): (?P<w>\\d+)x(?P<h>\\d+)$").unwrap();
  let mut claimedSquares: HashSet<(usize, usize)> = HashSet::new();
  let mut contestedSquares: HashSet<(usize, usize)> = HashSet::new();
  let mut candidateClaims: HashSet<(str, usize, usize, usize, usize)> = HashSet::new();

  for line in split {
    let claim = reg.captures(&line);
    match claim {
      Some(c) => {
        let id: str = &c["id"];
        let x: usize = c["x"].parse().expect("no x!");
        let y: usize = c["y"].parse().expect("no y!");
        let w: usize = c["w"].parse().expect("no w!");
        let h: usize = c["h"].parse().expect("no h!");
        let mut uncontested = true;
        // println!("{} {} {} {} {}", id, x, y, w, h);

        if check_claim(&x, &y, &w, &h, &mut claimedSquares, &mut contestedSquares) {
          candidateClaims.insert((id, x, y, w, h));
        }
      }
      None => {
        // println!("Non-matching line: {}", line);
      }
    }
  }
  for cc in candidateClaims {}

  println!("3A: {}", contestedSquares.len());
}

fn check_claim(
  x: &usize,
  y: &usize,
  w: &usize,
  h: &usize,
  claimedSquares: &mut HashSet<(usize, usize)>,
  contestedSquares: &mut HashSet<(usize, usize)>,
) -> bool {
  let mut uncontested = true;
  for cx in x + 1..=x + w {
    for cy in y + 1..=y + h {
      if !claimedSquares.insert((cx, cy)) {
        // square is already claimed
        contestedSquares.insert((cx, cy));
        uncontested = false;
      }
    }
  }
  return uncontested;
}
