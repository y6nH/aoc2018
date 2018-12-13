use crate::util;

pub fn a() {
  let mut s: String = util::read_file_to_string("input5");
  unsafe {
    let bytes = s.as_mut_vec().to_vec();
    let chomped = chomp_bytes(bytes);
    println!("5a: {}", chomped.len());
    //println!("{}", String::from_utf8(chomped).unwrap());
  }
}

fn chomp_bytes(bytes: Vec<u8>) -> Vec<u8> {
  let l = bytes.len();
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

  // Recurse until it stops getting smaller
  if result.len() == l {
    return result;
  } else {
    // println!("Length: {}", result.len());
    return chomp_bytes(result);
  }
}
