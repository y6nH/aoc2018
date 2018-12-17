use crate::util;
use std::str;

pub fn a() {
  let s: String = util::read_file_to_string("input7");
  let lines: Vec<&str> = s.lines().collect();
  // Lines look like "Step F must be finished before step M can begin." - steps at chars 5 and 36
  let mut rules: Vec<(u8, u8)> = Vec::new();

  for line in lines {
    let c: Vec<u8> = line.bytes().collect();
    rules.push((c[5], c[36]));
  }

  //println!("{:?}", rules);

  assert_eq!(rules[0], (b'P', b'F'));

  let mut sorted: Vec<u8> = Vec::new();

  for b in (b'A')..=(b'Z') {
    sorted.push(b);
  }

  // This is ridiculous &**&*&*&*&*&****
  sorted.sort_by(|x, y| rules.contains(&(*x, *y)).cmp(&rules.contains(&(*y, *x))));
  // Not quite right - alphabetical sorting missed

  println!(
    "7A: {:?}",
    // rules.iter().map(|r| char::from(r.0)).collect::<String>()
    String::from_utf8(sorted).unwrap()
  );
}
