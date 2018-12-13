use crate::util;

pub fn a() {
  let mut s: String = util::read_file_to_string("input5");
  unsafe {
    let bytes = s.as_mut_vec().to_vec();
    let collapsed = collapse_bytes_loop(bytes);
    println!("5A: {}", collapsed.len());
  }
}

pub fn b() {
  let mut s: String = util::read_file_to_string("input5");
  // Find the smallest collapsed size resulting from removing one type of unit

  let mut smallest = s.len();

  for c in 65u8..=90u8 {
    unsafe {
      let mut bytes = s.as_mut_vec().to_vec();
      bytes.retain(|&x| x != c && x != c + 32);

      let collapsed = collapse_bytes_loop(bytes);
      if collapsed.len() < smallest {
        smallest = collapsed.len();
      }
      // println!(
      //   "{}: {}",
      //   String::from_utf8(vec![c, c + 32]).unwrap(),
      //   collapsed.len()
      // );
    }
  }
  println!("5B: {}", smallest);
}

fn collapse_bytes(bytes: &Vec<u8>) -> Vec<u8> {
  // let l = bytes.len();
  let mut result: Vec<u8> = Vec::new();
  let mut maybe_ok: u8 = bytes[0];

  for w in bytes.windows(2) {
    // The difference between the codes of lowercase and capital letters is 32.
    // Using wrapping subtraction logic, 224 indicates a difference of 32 in the other direction.
    let diff = w[1].wrapping_sub(w[0]);

    if maybe_ok > 0 && (diff == 32 || diff == 224) {
      maybe_ok = 0;
    } else {
      if maybe_ok > 0 {
        result.push(maybe_ok);
      }
      maybe_ok = w[1];
    }
    // println!("result {:?}", result);
  }
  if maybe_ok != 0 {
    result.push(maybe_ok);
  }

  return result;

  // Recursive strategy hit a stach overflow on part B!
  // if result.len() == l {
  //   return result;
  // } else {
  //   // println!("Length: {}", result.len());
  //   return collapse_bytes(result);
  // }
}

fn collapse_bytes_loop(bytes: Vec<u8>) -> Vec<u8> {
  let mut length = bytes.len();
  let mut result = bytes;

  loop {
    result = collapse_bytes(&result);
    if result.len() == length {
      return result;
    }
    length = result.len();
  }
}
