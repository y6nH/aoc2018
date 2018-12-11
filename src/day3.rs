use crate::util;
use regex::Regex;
use std::collections::HashSet;

/*
A claim like #123 @ 3,2: 5x4 means that claim ID 123 specifies a rectangle 3 inches from the left edge,
2 inches from the top edge, 5 inches wide, and 4 inches tall.
How many square inches of fabric are within two or more claims?
*/
pub fn a() {
  let s: String = util::read_file_to_string("input3");
  let split = s.lines();
  let reg =
    Regex::new("#(?P<id>\\d+) @ (?P<x>\\d+),(?P<y>\\d+): (?P<w>\\d+)x(?P<h>\\d+)$").unwrap();
  let mut claimed_squares: HashSet<(usize, usize)> = HashSet::new();
  let mut contested_squares: HashSet<(usize, usize)> = HashSet::new();
  let mut candidate_claims: HashSet<(String, usize, usize, usize, usize)> = HashSet::new();
  let mut overlapping_claims: HashSet<(String, usize, usize, usize, usize)> = HashSet::new();

  for line in split {
    let claim = reg.captures(&line);
    match claim {
      Some(c) => {
        let id: String = c["id"].to_string();
        let x: usize = c["x"].parse().expect("no x!");
        let y: usize = c["y"].parse().expect("no y!");
        let w: usize = c["w"].parse().expect("no w!");
        let h: usize = c["h"].parse().expect("no h!");
        //let mut uncontested = true;
        // println!("{} {} {} {} {}", id, x, y, w, h);

        if check_claim(&x, &y, &w, &h, &mut claimed_squares, &mut contested_squares) {
          candidate_claims.insert((id, x, y, w, h));
        }
      }
      None => {
        // println!("Non-matching line: {}", line);
      }
    }
  }
  for cc in &candidate_claims {
    // Run the check again, our candidates against the known contested squares
    if !check_claim(
      &cc.1,
      &cc.2,
      &cc.3,
      &cc.4,
      &mut contested_squares,
      &mut HashSet::new(),
    ) {
      &overlapping_claims.insert(cc.clone());
    }
  }
  let last_claim = &candidate_claims.difference(&overlapping_claims);
  println!("3A: {}", contested_squares.len());
  println!("3B: {:?}", last_claim);
}

fn check_claim(
  x: &usize,
  y: &usize,
  w: &usize,
  h: &usize,
  claimed_squares: &mut HashSet<(usize, usize)>,
  contested_squares: &mut HashSet<(usize, usize)>,
) -> bool {
  let mut uncontested = true;
  for cx in x + 1..=x + w {
    for cy in y + 1..=y + h {
      if !claimed_squares.insert((cx, cy)) {
        // square is already claimed
        contested_squares.insert((cx, cy));
        uncontested = false;
      }
    }
  }
  return uncontested;
}
